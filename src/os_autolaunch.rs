use anyhow::{anyhow, Result};
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;
use std::env::current_exe;
use std::sync::Arc;

pub struct Osys {
    auto_launch: Arc<Mutex<Option<AutoLaunch>>>,
}

impl Osys {
    pub fn global() -> &'static Osys {
        static SYSOPT: OnceCell<Osys> = OnceCell::new();

        SYSOPT.get_or_init(|| Osys {
            auto_launch: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn init_launch(&self) -> Result<()> {
        let app_exe = current_exe()?;
        let app_name = app_exe
            .file_stem()
            .and_then(|f| f.to_str())
            .ok_or(anyhow!("failed to get file stem"))?;

        let app_path = app_exe
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("failed to get app_path"))?
            .to_string();

        #[cfg(target_os = "windows")]
        let app_path = format!("\"{app_path}\"");

        #[cfg(target_os = "macos")]
        let app_path = (|| -> Option<String> {
            let path = std::path::PathBuf::from(&app_path);
            let path = path.parent()?.parent()?.parent()?;
            let extension = path.extension()?.to_str()?;
            match extension == "app" {
                true => Some(path.as_os_str().to_str()?.to_string()),
                false => None,
            }
        })()
        .unwrap_or(app_path);

        #[cfg(target_os = "linux")]
        let app_path = {
            use crate::core::handle::Handle;
            use tauri::Manager;

            let handle = Handle::global();
            match handle.app_handle.lock().as_ref() {
                Some(app_handle) => {
                    let appimage = app_handle.env().appimage;
                    appimage
                        .and_then(|p| p.to_str().map(|s| s.to_string()))
                        .unwrap_or(app_path)
                }
                None => app_path,
            }
        };

        let auto = AutoLaunchBuilder::new()
            .set_app_name(app_name)
            .set_app_path(&app_path)
            .build()?;
        *self.auto_launch.lock().await = Some(auto);
        Ok(())
    }

    pub async fn update_launch(&self, enable: bool) -> Result<()> {
        let auto_launch = self.auto_launch.lock().await;
        if auto_launch.is_none() {
            self.init_launch().await?;
        }

        // 如果程序启动的时候调用disable，而原本注册表没有自动启动的话 就会在disable那边传播错误
        // 所以要忽略disable的错误
        match enable {
            true => auto_launch.as_ref().unwrap().enable()?,
            false => match auto_launch.as_ref().unwrap().disable() {
                _ => {}
            },
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tokio;

    // 辅助函数：创建测试路径
    fn create_test_exe_path() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            PathBuf::from(r"C:\Program Files\MyApp\app.exe")
        }
        #[cfg(target_os = "macos")]
        {
            PathBuf::from("/Applications/MyApp.app/Contents/MacOS/app")
        }
        #[cfg(target_os = "linux")]
        {
            PathBuf::from("/usr/local/bin/app")
        }
    }

    #[tokio::test]
    async fn test_osys_singleton() {
        let instance1 = Osys::global();
        let instance2 = Osys::global();
        
        // 验证单例模式
        assert!(std::ptr::eq(instance1, instance2), "应该返回相同的实例");
    }

    #[tokio::test]
    async fn test_init_launch() {
        let osys = Osys::global();
        let result = osys.init_launch().await;
        assert!(result.is_ok(), "初始化应该成功");
        
        // 验证 auto_launch 已被初始化
        let auto_launch = osys.auto_launch.lock().await;
        assert!(auto_launch.is_some(), "auto_launch 不应为 None");
    }

    #[tokio::test]
    async fn test_update_launch() {
        let osys = Osys::global();
        
        // 测试启用自动启动
        let enable_result = osys.update_launch(true).await;
        assert!(enable_result.is_ok(), "启用自动启动应该成功");

        // 测试禁用自动启动
        let disable_result = osys.update_launch(false).await;
        assert!(disable_result.is_ok(), "禁用自动启动应该成功");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_macos_app_path_resolution() {
        let test_paths = vec![
            ("/Applications/MyApp.app/Contents/MacOS/binary", "/Applications/MyApp.app"),
            ("/usr/local/bin/myapp", "/usr/local/bin/myapp"),
            (
                "/Applications/MyApp.app/Contents/Frameworks/Helper.app/Contents/MacOS/binary",
                "/Applications/MyApp.app/Contents/Frameworks/Helper.app"
            ),
        ];

        for (input, expected) in test_paths {
            let result = (|| -> Option<String> {
                let path = std::path::PathBuf::from(input);
                let mut current = path.as_path();
                while let Some(parent) = current.parent() {
                    if let Some(extension) = current.extension() {
                        if extension == "app" {
                            return current.to_str().map(|s| s.to_string());
                        }
                    }
                    current = parent;
                }
                None
            })()
            .unwrap_or(input.to_string());

            assert_eq!(result, expected);
        }
    }
}
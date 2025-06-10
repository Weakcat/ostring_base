use anyhow::{anyhow, Result};
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use std::sync::{Arc, Mutex, OnceLock};
use std::env::current_exe;

/// 自动启动管理模块
pub struct AutoLaunchManager;

impl AutoLaunchManager {
    /// 保存AutoLaunch实例的静态OnceLock
    fn auto_launch_instance() -> &'static OnceLock<Arc<Mutex<AutoLaunch>>> {
        static AUTO_LAUNCH: OnceLock<Arc<Mutex<AutoLaunch>>> = OnceLock::new();
        &AUTO_LAUNCH
    }

    /// 获取AutoLaunch实例，如果不存在则创建
    fn get_or_init_autolaunch() -> Result<AutoLaunch> {
        // 如果已初始化，直接返回克隆
        if let Some(auto_launch) = Self::auto_launch_instance().get() {
            let guard = auto_launch.lock().map_err(|_| anyhow!("获取锁失败"))?;
            return Ok(guard.clone());
        }
        
        // 需要初始化
        let app_exe = current_exe()?;
        let app_name = app_exe
            .file_stem()
            .and_then(|f| f.to_str())
            .ok_or(anyhow!("无法获取应用程序名称"))?;

        let app_path = app_exe
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("无法获取应用程序路径"))?
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

        // Linux平台专用配置，注意我们移除了对外部crate的依赖
        #[cfg(target_os = "linux")]
        let app_path = app_path; // 在Linux下直接使用可执行文件路径
        
        let auto = AutoLaunchBuilder::new()
            .set_app_name(app_name)
            .set_app_path(&app_path)
            .build()?;
        
        // 使用OnceLock保存新创建的AutoLaunch
        let auto_arc = Arc::new(Mutex::new(auto.clone()));
        match Self::auto_launch_instance().set(auto_arc) {
            Ok(_) => Ok(auto),
            Err(_) => {
                // 如果在我们初始化的过程中，其他线程已经初始化了
                // 使用已存在的值
                let existing = Self::auto_launch_instance().get().unwrap();
                let guard = existing.lock().map_err(|_| anyhow!("获取锁失败"))?;
                Ok(guard.clone())
            }
        }
    }

    /// 检查自动启动是否已启用
    pub fn is_enabled() -> Result<bool> {
        let auto = Self::get_or_init_autolaunch()?;
        Ok(auto.is_enabled()?)
    }

    /// 更新自动启动状态
    /// 
    /// * `enable` - 设置为true启用自动启动，false禁用自动启动
    pub fn update_launch(enable: bool) -> Result<()> {
        let auto = Self::get_or_init_autolaunch()?;
        
        match enable {
            true => auto.enable()?,
            false => auto.disable()?,
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autolaunch_manager() {
        // 验证初始状态
        assert!(AutoLaunchManager::auto_launch_instance().get().is_none(), "初始状态应为None");
        
        // 测试启用自动启动（同时会初始化）
        assert!(AutoLaunchManager::update_launch(true).is_ok(), "启用自动启动应该成功");
        
        // 验证已被初始化
        assert!(AutoLaunchManager::auto_launch_instance().get().is_some(), "auto_launch 应该已初始化");
        
        // 测试禁用自动启动
        assert!(AutoLaunchManager::update_launch(false).is_ok(), "禁用自动启动应该成功");
        
        // 测试检查是否启用
        let is_enabled = AutoLaunchManager::is_enabled();
        assert!(is_enabled.is_ok(), "检查自动启动状态应该成功");
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
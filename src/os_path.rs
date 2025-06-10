use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};
use dirs;

/// 路径类型，用于区分文件和目录
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathType {
    /// 表示目录路径
    Directory,
    /// 表示文件路径
    File,
}

/// 路径管理，简化文件路径操作并支持链式调用
#[derive(Debug, Clone, PartialEq)]
pub struct PathManager {
    path: PathBuf,
    path_type: PathType,
}

impl PathManager {
    /// 从应用数据目录创建路径
    pub fn get_data_dir() -> Result<Self> {
        let path = dirs::data_dir().ok_or(anyhow!("无法获取数据目录"))?;
        Ok(Self::dir(path))
    }

    /// 从任意路径创建目录
    pub fn dir<P: AsRef<Path>>(path: P) -> Self {
        Self { 
            path: path.as_ref().to_path_buf(),
            path_type: PathType::Directory
        }
    }
    
    /// 从任意路径创建文件
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        Self { 
            path: path.as_ref().to_path_buf(),
            path_type: PathType::File
        }
    }
    
    /// 连接子目录，如果当前路径是文件则返回错误
    pub fn join_dir<P: AsRef<Path>>(mut self, dir: P) -> Result<Self> {
        if self.path_type == PathType::File {
            return Err(anyhow!("无法在文件路径上进行join操作"));
        }
        
        self.path = self.path.join(dir);
        self.path_type = PathType::Directory;
        Ok(self)
    }
    
    /// 连接文件名，如果当前路径是文件则返回错误
    pub fn join_file<P: AsRef<Path>>(mut self, filename: P) -> Result<Self> {
        if self.path_type == PathType::File {
            return Err(anyhow!("无法在文件路径上进行join操作"));
        }
        
        self.path = self.path.join(filename);
        self.path_type = PathType::File;
        Ok(self)
    }

    /// 确保路径存在，根据路径类型自动创建目录或文件
    pub fn ensure(self) -> Result<Self> {
        match self.path_type {
            PathType::Directory => self.ensure_dir(),
            PathType::File => self.ensure_file(),
        }
    }

    /// 确保目录存在，如果不存在则创建
    fn ensure_dir(self) -> Result<Self> {
        if self.path.exists() {
            // 如果路径存在，确保它是一个目录
            if !self.path.is_dir() {
                return Err(anyhow!("路径 '{}' 已存在但不是目录", 
                    self.path.to_string_lossy()));
            }
        } else {
            // 路径不存在，创建目录
            std::fs::create_dir_all(&self.path)?;
        }
        Ok(self)
    }

    /// 确保文件存在，如果不存在则创建(包括所需的父目录)
    fn ensure_file(self) -> Result<Self> {
        if self.path.exists() {
            // 如果路径存在，确保它是一个文件
            if !self.path.is_file() {
                return Err(anyhow!("路径 '{}' 已存在但不是文件", 
                    self.path.to_string_lossy()));
            }
        } else {
            // 确保父目录存在
            if let Some(parent) = self.path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                } else if !parent.is_dir() {
                    return Err(anyhow!("父路径 '{}' 存在但不是目录", 
                        parent.to_string_lossy()));
                }
            }
            
            // 创建文件
            std::fs::File::create(&self.path)?;
        }
        
        Ok(self)
    }
    
    /// 获取PathBuf
    pub fn path(self) -> PathBuf {
        self.path
    }
    
    /// 获取字符串路径
    pub fn string(self) -> Result<String> {
        self.path.to_str()
            .map(String::from)
            .ok_or(anyhow!("无法将路径转换为字符串"))
    }
}

/// 获取应用数据文件路径，自动创建必要的目录和文件
pub fn get_data_file_path(app_name: &str, filename: &str) -> Result<PathManager> {
    PathManager::get_data_dir()?.join_dir(app_name)?.join_file(filename)
}

/// 获取应用数据子目录路径，自动创建必要的目录和文件
pub fn get_data_child_dir_path(app_name: &str, child_dir: Option<String>) -> Result<PathManager> {
    let path_manager = PathManager::get_data_dir()?.join_dir(app_name)?;
    let path_manager = match child_dir {
        Some(child_dir) => path_manager.join_dir(&child_dir)?,
        None => path_manager
    };
    Ok(path_manager)
}
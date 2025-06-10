# OS System Tools

一个用 Rust 编写的系统工具集合，提供系统信息获取、文件路径管理、串口管理和自动启动配置等功能。

## 功能特性

- 📊 系统信息监控
  - 内存使用情况
  - 系统名称和版本
  - 主机名
  - 网络接口信息

- 📁 路径管理
  - 链式API设计
  - 目录/文件自动创建
  - 类型安全的路径操作

- 🔌 串口管理
  - 列出可用串口
  - 获取串口详细信息（ID、名称、制造商）

- 🚀 自动启动配置
  - 支持 Windows/macOS/Linux
  - 配置应用程序自启动
  - 支持启用/禁用自启动

## 系统要求

- Rust 1.70 或更高版本
- 支持的操作系统：
  - Windows 10/11
  - macOS 10.15+
  - Linux (主流发行版)

## 安装

1. 克隆仓库：
   ```bash
   git clone https://github.com/Weakcat/ostring_base.git
   ```

2. 构建项目：
   ```bash
   cargo build --release
   ```

## 使用示例

### 系统信息获取

```rust
use ostring_base::os_sysinfo::OsSysInfo;

let info = OsSysInfo::get_info();
println!("{:?}", info);
```

### 路径管理 (链式API)

```rust
use ostring_base::os_path::{PathManager, app_file_path};

// 获取并确保应用配置文件存在
let config_path = app_file_path("myapp", "config.json")?;

// 自定义路径操作
let mut log_manager = PathManager::get_data_dir()?;
log_manager.join_dir("myapp")?
           .join_dir("logs")?
           .ensure()?;
let log_dir = log_manager.path();
    
// 文件路径（注意：文件路径后不能再join）
let mut file_manager = PathManager::get_data_dir()?;
file_manager.join_dir("myapp")?
            .join_file("data.log")?;  // 此时path_type为File
    
// 以下操作会返回错误
// file_manager.join_dir("logs")?; // 错误：无法在文件路径上进行join操作

// 创建并获取文件路径
file_manager.ensure()?;
let file_path = file_manager.string()?;
```

### 串口列表

```rust
use ostring_base::os_serialport::serial_port_list;

let ports = serial_port_list();
println!("{:?}", ports);
```

### 自动启动配置
```rust
use ostring_base::os_autolaunch::AutoLaunchManager;

// 检查是否启用了自动启动
let is_enabled = AutoLaunchManager::is_enabled()?;
println!("自动启动状态: {}", is_enabled);

// 启用自动启动
AutoLaunchManager::update_launch(true)?;

// 禁用自动启动
AutoLaunchManager::update_launch(false)?;
```


## 依赖项

- `serde`: 序列化/反序列化支持
- `sysinfo`: 系统信息获取
- `serialport`: 串口通信
- `auto-launch`: 自动启动配置
- `dirs`: 系统目录路径获取
- `anyhow`: 错误处理

## 开发

### 运行测试
```bash
cargo test
```
# OS System Tools

一个用 Rust 编写的系统工具集合，提供系统信息获取、串口管理和自动启动配置等功能。

## 功能特性

- 📊 系统信息监控
  - 内存使用情况
  - 系统名称和版本
  - 主机名
  - 网络接口信息

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

### 串口列表

```rust
use ostring_base::os_serialport::serial_port_list;

let ports = serial_port_list();
println!("{:?}", ports);
```

### 自动启动配置
```rust
use ostring_base::os_autolaunch::OsAutoLaunch;

let osys = OsAutoLaunch::new();
osys.update_launch(true).await;
```


## 依赖项

- `serde`: 序列化/反序列化支持
- `sysinfo`: 系统信息获取
- `serialport`: 串口通信
- `auto-launch`: 自动启动配置
- `tokio`: 异步运行时
- `anyhow`: 错误处理

## 开发

### 运行测试
```bash
cargo test
```
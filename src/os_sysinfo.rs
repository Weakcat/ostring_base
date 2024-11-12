use serde::{Deserialize, Serialize};
use sysinfo::{Networks, System};

const GB_IN_BYTES: f64 = 1_073_741_824.0;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OsSysInfo {
    name: String,
    version: String,
    host: String,
    memory: String,
    networks: Vec<OsNet>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct OsNet {
    id: usize,
    name: String,
    mac: String,
}

impl OsSysInfo {
    pub fn get_info() -> OsSysInfo {
        let mut sys_info = OsSysInfo::default();
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_memory_bytes = sys.total_memory();
        let used_memory_bytes = sys.used_memory();

        let total_memory_gb = total_memory_bytes as f64 / GB_IN_BYTES;
        let used_memory_gb = used_memory_bytes as f64 / GB_IN_BYTES;
        let memory_usage = format!("{:.2} GB / {:.2} GB", used_memory_gb, total_memory_gb);

        sys_info.memory = memory_usage;
        sys_info.name = System::name().unwrap_or_default();
        sys_info.version = System::os_version().unwrap_or_default();
        sys_info.host = System::host_name().unwrap_or_default();
        sys_info.networks = Networks::new_with_refreshed_list()
            .iter()
            .enumerate()
            .map(|(id, (interface_name, data))| OsNet {
                id: id + 1,
                name: interface_name.to_string(),
                mac: data.mac_address().to_string(),
            })
            .collect();

        sys_info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_sysinfo_structure() {
        let info = OsSysInfo::get_info();
        
        // 验证基本结构不为空
        assert!(!info.name.is_empty(), "系统名称不应为空");
        assert!(!info.version.is_empty(), "系统版本不应为空");
        assert!(!info.host.is_empty(), "主机名不应为空");
        
        // 验证内存格式
        assert!(info.memory.contains("GB"), "内存信息应包含 GB");
        assert!(info.memory.contains("/"), "内存信息应包含分隔符 /");
        
        // 验证网络信息
        for net in info.networks {
            assert!(net.id > 0, "网络接口ID应大于0");
            assert!(!net.name.is_empty(), "网络接口名称不应为空");
            // MAC地址可能为空，所以不做验证
        }
    }
}
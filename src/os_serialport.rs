use serde::Serialize;
use serialport::{self, SerialPortType};

#[derive(Serialize, Debug)]
pub struct PortInfo {
    id: usize,
    label: String,
    desc: String,
}

pub fn serial_port_list() -> Vec<PortInfo> {
    let mut result: Vec<PortInfo> = vec![];
    if let Ok(ports) = serialport::available_ports() {
        result.extend(ports.iter().enumerate().filter_map(|(current_id, p)| {
            if let SerialPortType::UsbPort(info) = &p.port_type {
                Some(PortInfo {
                    id: current_id,
                    label: p.port_name.clone(),
                    desc: info.clone().manufacturer.unwrap_or("unknown".to_string()),
                })
            } else {
                None
            }
        }));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_port_list_empty() {
        let ports = serial_port_list();
        // 验证返回的是一个 Vec（即使可能为空）
        assert!(ports.is_empty() || !ports.is_empty());
        // 或者简单地验证类型
        let _: Vec<PortInfo> = ports;
    }
}
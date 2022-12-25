use crate::app;
use serde::{Serialize, Deserialize};

use app::core::{
    keyboard_listen::keyboard_listen_state::KeyboardRecordStruct,
    system_info::system_info_state::{
        SystemInfoStruct,
        SytemStruct,
        RamAndSwapStruct,
        DiskStruct,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MainStruct {
    pub system_info: SystemInfoStruct,
    pub keyboard: Vec<KeyboardRecordStruct>,
}

impl MainStruct {
    pub fn new() -> Self {
        Self {
            keyboard: Vec::new(),
            system_info: SystemInfoStruct {
                system: SytemStruct{
                    name: "".to_string(),
                    host_name: "".to_string(),
                    kernel_version: "".to_string(),
                    os_version: "".to_string(),
                    long_os_version: "".to_string(),
                    boot_at_seconds: 0,
                    running_at_seconds: 0,
                },
                ram_swap: RamAndSwapStruct{
                    total_memory: 0,
                    free_memory: 0,
                    available_memory: 0,
                    used_memory: 0,
                    total_swap: 0,
                    free_swap: 0,
                    used_swap: 0,
                },
                disk: DiskStruct{
                    storage_type: "".to_string(),
                    name: "".to_string(),
                    file_system: "".to_string(),
                    mount_point: "".to_string(),
                    total_space: "".to_string(),
                    available_space: "".to_string(),
                    is_removable: "".to_string(),
                },
                cpu: Vec::new(),
                network: Vec::new(),
                process: Vec::new(),
            },
        }
    }
}

#[derive(Debug)]
pub enum MainEnum {
    KeyboardEnum(KeyboardRecordStruct),
    SystemInfoEnum(SystemInfoStruct),
}

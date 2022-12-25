use std::fmt::{Display, Formatter, Result};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfoStruct {
    pub system: SytemStruct,
    pub ram_swap: RamAndSwapStruct,
    pub disk: DiskStruct,
    pub cpu: Vec<CPUStruct>,
    pub network: Vec<NetworkStruct>,
    pub process: Vec<ProcessStruct>
} 

/// 系统信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SytemStruct {
    pub name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub long_os_version: String,
    pub boot_at_seconds: u64,
    pub running_at_seconds: u64,
}

/// ram 和 swap 信息
#[derive(Debug, Serialize, Deserialize)]
pub struct RamAndSwapStruct {
    pub total_memory: u64,
    pub free_memory: u64,
    pub available_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub free_swap: u64,
    pub used_swap: u64,
}

/// 磁盘信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DiskStruct {
    pub storage_type: String,
    pub name: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_space: String,
    pub available_space: String,
    pub is_removable: String,
}

/// cpu 信息, 获取 cpu 的每核信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CPUStruct {
    pub name: String,
    pub vendor_id: String,
    pub brand: String,
    pub frequency: String,
}

/// 网络接口信息
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStruct {
    pub interface_name: String,
}

/// 进程信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessStruct {
    pub parent_pid: String,
    pub pid: String,
    pub user_id: String,
    pub group_id: String,
    pub name: String,
    pub status: String,
    pub session_id: String,
    pub running_seconds: u64,
    pub execute_path: String,
    pub command: String,
    pub environ: String,
    pub cwd_path: String,
    pub root_path: String,
    pub memory: u64,
    pub virtual_memory: u64,
    pub disk_usage_read_bytes: u64,
    pub disk_usage_total_read_bytes: u64,
}


impl Display for SystemInfoStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, 
r#"{{
    "system": {{{}}},
    "ram_swap": {{{}}},
    "disk": {{{}}},
    "cpu": {{{:#?}}},
    "network_interface": {{{:?}}},
    "process": {{{:?}}},
}}\n"#, 
            self.system, self.ram_swap, self.disk, self.cpu, 
            self.network, self.process,
        )
    }
}

impl Display for SytemStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, 
    r#"
        "name": "{}",
        "host_name": "{}",
        "kernel_version": "{}",
        "os_version": "{}",
        "long_os_version": "{}",
        "boot_at_seconds": {},
        "running_at_seconds": {}
    "#, 
            self.name, self.host_name, self.kernel_version, self.os_version, 
            self.long_os_version, self.boot_at_seconds, self.running_at_seconds,
        )
    }
}

impl Display for RamAndSwapStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,         
    r#"
        "total_memory": {},
        "free_memory": {},
        "available_memory": {},
        "used_memory": {},
        "total_swap": {},
        "free_swap": {},
        "used_swap": {}
    "#, 
            self.total_memory, self.free_memory, self.available_memory, self.used_memory, 
            self.total_swap, self.free_swap, self.used_swap,
        )
    }
}

impl Display for DiskStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, 
    r#"
        "storage_type": "{}",
        "name": {},
        "file_system": "{}",
        "mount_point": {},
        "total_space": {},
        "available_space": {},
        "is_removable": {}
    "#, 
            self.storage_type, self.name, self.file_system, self.mount_point, 
            self.total_space, self.available_space, self.is_removable,
        )
    }
}

impl Display for CPUStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,
    r#"
        "name": "{}",
        "vendor_id": "{}",
        "brand": "{}",
        "frequency": {}
    "#, 
            self.name, self.vendor_id, self.brand, self.frequency, 
        )
    }
}

impl Display for NetworkStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\n{}\n", 
            self.interface_name,
        )
    }
}

impl Display for ProcessStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n{},\n", 
            self.parent_pid, self.pid, self.user_id, self.group_id, self.name, self.status, self.session_id,
            self.running_seconds, self.execute_path, self.command, self.environ, self.cwd_path, self.root_path,
            self.memory, self.virtual_memory, self.disk_usage_read_bytes, self.disk_usage_total_read_bytes,
        )
    }
}

use std::fs;
use chrono::Local;
use serde_json::json;
use std::time::Instant;
use tokio::sync::mpsc::Sender;
use sysinfo::{System, SystemExt, DiskExt, CpuExt, ProcessExt};

use crate::{app, bootstrap};
use bootstrap::app_state::{MainStruct, MainEnum};
use app::core::system_info::system_info_state::{
    DiskStruct,
    CPUStruct,
    NetworkStruct,
    RamAndSwapStruct,
    SytemStruct,
    ProcessStruct,
};

pub async fn get_info(sender: Sender<MainEnum>) {
    println!("System info- Fetch... {}", Local::now().to_string());
    
    let start = Instant::now();
    let mut main_struct = MainStruct::new();

    // 获取系统信息并刷新一下
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // 获取磁盘、cpu、网络等信息
    disk(&sys, &mut main_struct).await;
    cpu(&sys, &mut main_struct).await;
    network(&sys, &mut main_struct).await;
    ram_swap(&sys, &mut main_struct).await;
    system(&sys, &mut main_struct).await;
    process(&sys, &mut main_struct).await;
    
    // 将数据序列化并格式化
    let serialized_result = json!(&mut main_struct.system_info);
    let new_string = format!("{}", serialized_result);

    // 写入到 json 文件中
    fs::write("target/storage/computer_info.json", new_string.as_bytes()).unwrap();

    // 将数据发送给消费者
    sender.send(MainEnum::SystemInfoEnum(main_struct.system_info)).await.unwrap();

    println!("System info- Fetch success, total time: {:?}", start.elapsed());
}

async fn disk(sys: &System, main_struct: &mut MainStruct) {
    for disk in sys.disks() {
        main_struct.system_info.disk = DiskStruct{
            storage_type: format!("{:?}", disk.type_()),            // 磁盘类型
            name: format!("{:?}", disk.name()),                     // 磁盘名称
            file_system: format!("{:?}", disk.file_system()),       // 此磁盘上使用的文件系统（例如: EXT4、NTFS等)
            mount_point: format!("{:?}", disk.mount_point()),       // 返回磁盘的安装点（例如: /)
            total_space: format!("{}", disk.total_space()),         // 总磁盘大小，以字节为单位
            available_space: format!("{}", disk.available_space()), // 可用磁盘大小，以字节为单位
            is_removable: format!("{}", disk.is_removable()),       // 磁盘是否可移动
        };
    }
}

async fn cpu(sys: &System, main_struct: &mut MainStruct) {
    for cpu in sys.cpus() {
        main_struct.system_info.cpu.push(CPUStruct{
            name: format!("{}", cpu.name()),            // CPU 的名称
            vendor_id: format!("{}", cpu.vendor_id()),  // CPU 的供应商 ID
            brand: format!("{}", cpu.brand()),          // CPU 的品牌
            frequency: format!("{}", cpu.frequency()),  // CPU 的频率
        });
    }
}

async fn network(sys: &System, main_struct: &mut MainStruct) {
    for (interface_name, _data) in sys.networks() {
        main_struct.system_info.network.push(NetworkStruct{
            interface_name: interface_name.to_string(),
        })
    }
}

async fn ram_swap(sys: &System, main_struct: &mut MainStruct) {
    main_struct.system_info.ram_swap = RamAndSwapStruct{
        total_memory: sys.total_memory(),          // 以字节为单位返回 RAM 大小
        free_memory: sys.free_memory(),            // 返回空闲 RAM 的字节数
        available_memory: sys.available_memory(),  // 返回可用 RAM 的字节数
        used_memory: sys.used_memory(),            // 以字节为单位返回已用 RAM 的数量
        total_swap: sys.total_swap(),              // 以字节为单位返回 SWAP 大小
        free_swap: sys.free_swap(),                // 以字节为单位返回空闲 SWAP 的数量   
        used_swap: sys.used_swap(),                // 以字节为单位返回已使用的 SWAP 数量
    };
}

async fn system(sys: &System, main_struct: &mut MainStruct) {
    main_struct.system_info.system = SytemStruct{
        name: sys.name().unwrap(),                          // 系统名称
        host_name: sys.host_name().unwrap(),                // 基于 DNS 的系统主机名
        kernel_version: sys.kernel_version().unwrap(),      // 系统的内核版本
        os_version: sys.os_version().unwrap(),              // 系统版本（对于 MacOS，这将返回 13.0.1 而不是内核版本）      
        long_os_version: sys.long_os_version().unwrap(),    // 系统长操作系统版本（如“MacOS 13.0.1”）
        boot_at_seconds: sys.boot_time(),                   // 系统从 UNIX 纪元开始启动的时间（以秒为单位）
        running_at_seconds: sys.uptime(),                   // 系统正常运行时间（以秒为单位）
    };
}

async fn process(sys: &System, main_struct: &mut MainStruct) {
    for (pid, process) in sys.processes() {
        let disk_usage = process.disk_usage();

        main_struct.system_info.process.push(ProcessStruct{
            parent_pid: format!("{:?}", process.parent()),              // 进程的父 pid
            pid: pid.to_string(),                                       // 进程的 pid
            user_id: format!("{:?}", process.user_id()),                // 进程的所有者用户的 ID
            group_id: format!("{:?}", process.group_id()),              // 进程的进程组 ID
            name: format!("{}", process.name()),                        // 进程的名称
            status: format!("{:?}", process.status()),                  // 进程的状态
            session_id: format!("{:?}", process.session_id()),          // 当前进程的会话 ID               
            running_seconds: process.run_time(),                        // 进程运行的时间（以秒为单位）
            execute_path: format!("{:?}", process.exe().display()),     // 进程的路径
            command: format!("{:?}", process.cmd()),                    // 命令行
            environ: format!("{:?}", process.environ()),                // 进程的环境变量
            cwd_path: format!("{}", process.cwd().display()),           // 当前工作目录
            root_path: format!("{}", process.root().display()),         // 根目录的路径
            memory: process.memory(),                                   // 内存使用情况（以字节为单位）
            virtual_memory: process.virtual_memory(),                   // 虚拟内存使用情况（以字节为单位）
            disk_usage_read_bytes: disk_usage.read_bytes,               // 读取磁盘的字节数
            disk_usage_total_read_bytes: disk_usage.total_read_bytes,   // 总共读取磁盘的字节数
        });
    }
}

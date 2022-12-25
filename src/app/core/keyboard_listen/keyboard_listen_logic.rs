use chrono::Local;
use std::{fs, thread};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use device_query::{DeviceState, DeviceQuery};

use crate::{app, bootstrap};
use bootstrap::app_state::{MainStruct, MainEnum::{self, KeyboardEnum}};
use app::core::keyboard_listen::keyboard_listen_state::KeyboardRecordStruct;

pub async fn listen(sender: Sender<MainEnum>) {
    println!("Keyboard state- Listening... {}", Local::now().to_string());
    
    let device_state = DeviceState::new();
    let mut main_struct = MainStruct::new();

    loop {
        // 获取当前键盘状态
        let keys = DeviceQuery::get_keys(&device_state);

        if !keys.is_empty() {
            // 暂停线程 100 ms
            thread::sleep(Duration::from_millis(100));

            // 保存一份，为了统计长度
            main_struct.keyboard.push(KeyboardRecordStruct{
                id: main_struct.keyboard.len(),
                value: format!("{:?}", keys),
                datetime: Local::now().format(&"%Y-%m-%d %H:%M:%S").to_string(),
            });

            // 将数据发送给消费者
            sender.send(KeyboardEnum(KeyboardRecordStruct{
                id: main_struct.keyboard.len(),
                value: format!("{:?}", keys),
                datetime: Local::now().format(&"%Y-%m-%d %H:%M:%S").to_string(),
            })).await.unwrap();

            let new_string = format!("\n{:?}", main_struct.keyboard);
            fs::write("target/storage/key_log.json", new_string.as_bytes()).unwrap();
        }
    }
}
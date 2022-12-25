// extern crate rustls;

// // use rustls::{ClientConfig, ConfigBuilder, ClientConnection, Stream};
// use std::{net::TcpStream, sync::Arc};
// use webpki::DnsNameRef;

// pub struct _TlsStream {
//     address: String,
// }

// impl _TlsStream {
//     pub fn new(&self) {
//         // 使用 try_from_ascii_str 方法来将字符串转换为 DnsNameRef
//         let dns_name = DnsNameRef::try_from_ascii_str("www.example.com").unwrap();

        
//         // 创建一个 TLS 配置构建器
//         let config_builder = rustls::client::ClientConfig::builder();

//         // 设置安全的默认密码套件
//         config_builder.with_safe_default_cipher_suites();

//         // 调用 build 方法来实际构建 TLS 配置对象
//         // let config = config_builder;

//         let mut stream = TcpStream::connect(&self.address).unwrap();
//         // let mut session = ClientConnection::new(Arc::new(config), dns_name);

//         // let mut tls = Stream::new(&mut session, &mut stream);

//         // let message = b"Hello, world!";
//         // tls.write_all(message)?;
    
//         // let mut buf = [0u8; 1024];
//         // let len = tls.read(&mut buf)?;
//         // let response = str::from_utf8(&buf[..len]).unwrap();
//         // println!("Response: {}", response);
//     }
// }

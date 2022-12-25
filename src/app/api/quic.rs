// use s2n_quic;

// pub struct Connect;

// impl Connect {
    
// }





























// // use std::{net::SocketAddr};
// // use quinn::{Endpoint, Connection, SendStream, ClientConfig};
// // pub struct Connect;
// // impl Connect {
// //     pub fn client_addr() -> SocketAddr {
// //         "127.0.0.1:8080".parse::<SocketAddr>().unwrap()
// //     }
// //     pub fn server_addr() -> SocketAddr {
// //         "127.0.0.1:8080".parse::<SocketAddr>().unwrap()
// //     }
// //     pub async fn client() -> Connection {
// //         // 创建一个 Endpoint 对象来连接到 QUIC 服务器
// //         let mut endpoint = Endpoint::client(Self::client_addr()).unwrap();
// //         let config = ClientConfig::with_native_roots();
// //         endpoint.set_default_client_config(config);
// //         // 创建一个连接并打开一个流
// //         let connect = endpoint.connect(Self::server_addr(), "localhost").unwrap();
// //         let connection = connect.await.unwrap();
// //         connection
// //     }
// //     pub async fn open_unidirectional_stream(connection: Connection) -> SendStream {
// //         let send = connection
// //             .open_uni()
// //             .await.unwrap();
// //         send
// //     }
// // }

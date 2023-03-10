// use s2n_quic::{Server, stream};
// use std::error::Error;

// const CERT_PEM: &str = include_str!("cert.pem");
// const KEY_PEM: &str = include_str!("key.pem");

// pub async fn run() -> Result<(), Box<dyn Error>> {
//     println!("server running...\n");

//     let mut server = Server::builder()
//         .with_tls((CERT_PEM, KEY_PEM))?
//         .with_io("127.0.0.1:4433")?
//         .start()?;

//     while let Some(mut connection) = server.accept().await {
//         // spawn a new task for the connection
//         tokio::spawn(async move {
//             while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
//                 // spawn a new task for the stream
//                 tokio::spawn(async move {
//                     // echo any data back to the stream
//                     while let Ok(Some(data)) = stream.receive().await {
//                         stream.send(data).await.expect("stream should be open");
//                     }
//                 });
//             }
//         });
//     }

//     Ok(())
// }
// use crate::untils;
// use reqwest::{Client, Proxy, Response};

// use untils::encrypt_data;

// pub async fn screen_data() -> Response {
//     let client = Client::builder()
//         .proxy(Proxy::http("http://proxy.example.com:8080").unwrap())
//         .build()
//         .expect("创建请求失败");
        
//     let response = client.post("https://192.168.0.1/")
//         .header(reqwest::header::ACCEPT, "application/json")
//         .body("the exact body that is sent")
//         .send()
//         .await
//         .expect("POST 请求失败");

//     return response;
// }
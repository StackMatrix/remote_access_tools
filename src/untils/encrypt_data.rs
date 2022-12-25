// use ring::{
//     aead::{Algorithm, Nonce, SealingKey, UnboundKey},
//     rand::SystemRandom,
// };

// pub fn create() {
//     // 创建密钥
//     // 表示创建一个长度为 32 的字节数组 key，并将所有元素都初始化为 0。
//     // 这个字节数组就是加密用的密钥，其中包含了一些随机数据，用于安全地加密和解密数据。
//     // let key = [0u8; 32];
//     // let sealed_key = aead::SealingKey::new(
//     //     aead::CHACHA20_POLY1305,
//     //     &key
//     // ).expect("创建密钥失败");
//     let key = [0u8; 32];

//     let key =  UnboundKey::from(key);
//     let key = UnboundKey::from_slice(Algorithm::AES_256_GCM, &key).unwrap();
//     let key = SealingKey::new(key, Nonce::assume_unique_for_key([0; 12])).unwrap();

//     // 对数据进行加密
//     let data = "原始数据";
//     let nonce = aead::Nonce::assume_unique_for_key([1; 12]);
//     aead::SealingKey::new(aead::CHACHA20_POLY1305, nonce_sequence);

//     let sealed_key = aead::SealingKey::new(
//         aead::CHACHA20_POLY1305,
//         &nonce
//     ).expect("创建密钥失败");
//     let mut ciphertext = vec![0; aead::CHACHA20_POLY1305.tag_len()];
//     let ciphertext = aead::seal_in_place(&sealed_key, nonce, b"", data, &mut ciphertext)
//         .expect("加密失败");

// }


// use ring::{aead, error};

// fn encrypt(message: &[u8], key: &[u8]) -> Result<Vec<u8>, error::Unspecified> {
//     let nonce = [0u8; 12];
//     let aad = [0u8; 0];
//     let alg = aead::AES_256_GCM;

    
//     let key = aead::UnboundKey::from_slice(alg, key).unwrap();

//     let enc_key = aead::LessSafeKey::new(key);

//     enc_key.encrypt(nonce, aad, message)
// }

// fn decrypt(message: &[u8], key: &[u8]) -> Result<Vec<u8>, error::Unspecified> {
//     let nonce = [0u8; 12];
//     let aad = [0u8; 0];
//     let alg = aead::AES_256_GCM;

//     let key = aead::UnboundKey::from_slice(alg, key).unwrap();
//     let dec_key = aead::LessSafeKey::new(key);

//     dec_key.decrypt(nonce, aad, message)
// }
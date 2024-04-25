#![warn(clippy::all, clippy::pedantic)]
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

fn main() {
// Создаем экземпляр AES-256 с использованием ключа
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let cipher = Aes256::new(&key);

    // Инициализируем режим блочного шифрования CBC
    type Aes256Cbc = Cbc<Aes256, Pkcs7>;
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes256Cbc::new_var(&key, &iv).unwrap();

// Шифруем данные
    let mail = "mail@example.com".as_bytes();
    let password = "2468".as_bytes();
    let ciphertext = cipher.encrypt_vec(plaintext);
    let cipher_password = cipher.encrypt_vec(password);

// Дешифруем данные
    let decrypted_data = cipher.decrypt_vec(&ciphertext).unwrap();

// Выводим результаты
    println!("Шифрованный текст: {:?}", ciphertext);
    println!("Дешифрованный текст: {:?}", String::from_utf8(decrypted_data)
        .unwrap());
}
//libraries: block_modes, block_padding, as: aes, block_modes
extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;

fn main() {
    // Получаем список сетевых интерфейсов
    let interfaces = datalink::interfaces();
    let password_length = 3;
    let characters = "abcdefghijklmnopqrstuvwxyz";

    brute_force_attack(password_length, characters);

    // Выбираем сетевой интерфейс для захвата пакетов (например, Wi-Fi интерфейс)
    let interface = interfaces
        .into_iter()
        .find(|iface: &NetworkInterface| iface.is_up() && iface.is_broadcast()
            && iface.is_multicast())
        .unwrap();

    // Открываем канал для захвата пакетов на выбранном интерфейсе
    let (mut tx, mut rx) = match datalink::channel(&interface,
                                                   Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create channel: {}", e),
    };
    const MAX_RETRIES: u32 = 3;
    let mut retries = 0;

    // Захватываем пакеты Wi-Fi и обрабатываем их
    loop {
        match rx.next() {
            Ok(packet) => {
                // Обрабатываем захваченный пакет Wi-Fi
                println!("Captured packet: {:?}", packet);
            },
            Err(e) => {
                // Обрабатываем ошибку захвата пакета
                println!("Error capturing packet: {}", e);
                retries += 1;
                if retries > MAX_RETRIES {
                    println!("Exceeded maximum retries. Exiting.");
                    break;
                }
            }
        }
    }
    fn brute_force_attack(password_length: usize, characters: &str) {
        let mut password = vec![0; password_length];
        let base = characters.len();

        loop {
            // Check password
            let password_str: String = password.iter()
                .map(|&i| characters.chars()
                    .nth(i).unwrap()).collect();
            println!("Trying password: {}", password_str);

            // Increment password
            for i in (0..password_length).rev() {
                if password[i] < base - 1 {
                    password[i] += 1;
                    break;
                } else {
                    password[i] = 0;
                }
            }
            // Check if all combinations have been tried
            if password.iter().all(|&x| x == 0) {
                break;
            }
        }
    }
}

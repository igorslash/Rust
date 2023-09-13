use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // Открытие файла устройства
    let mut device = File::open("/dev/my_device")
        .expect("Не удалось открыть устройство");

    // Запись данных в устройство
    let data = [1, 2, 3, 4, 5];
    device.write_all(&data)
        .expect("Не удалось записать данные в устройство");

    // Чтение данных из устройства
    let mut read_data = [0; 5];
    device.read_exact(&mut read_data)
        .expect("Не удалось прочитать данные из устройства");

    // Обработка прочитанных данных
    println!("Прочитанные данные: {:?}", read_data);
}
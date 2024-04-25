fn main() {
    // Создаем клиент для выполнения HTTP-запросов
    let client = reqwest::Client::new();
    //Настройка параметров для отправки формы
    let params = [("username", "admin"), ("password", "password")];
    //Создаем URL-адрес
    let url = Url::parse("https://www.example.com\
    /search?document.forms[0].addEventListener('submit', function(event) {
    // Отменяем стандартное действие отправки формы
    event.preventDefault();=<script>alert('hack')</script>").unwrap();
    println!("{:?}", url);
    //Отправляем POST-запрос
    let response = client
        .post("https://www.example.com/login")
        .form(&params)
        .send()
        .unwrap();
    //Получаем ответ
    match response {
        Ok(e) => println!("{:?} great", e),
        Err(e) => println!("{:?} bad", e),
    }
}
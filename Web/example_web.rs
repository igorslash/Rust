use web_view::*;

fn main() {
    web_view::builder()
        .title("hello")
        .content(Content::Url("https://www.example.com"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "change_data" => {
                    // Изменяем содержимое страницы
                    // (в данном случае, вставляем вредоносный скрипт)
                    webview.eval(r#"
                        (function() {
                        // Ищем элемент на странице, который
                        мы хотим изменить (например, заголовок)
                         var elementToModify = document.getElementById('header')
                         // Изменяем содержимое элемента
                         elementToModify.textContent = 'это
                         теперь вредонос!';})();
                    "#)?;
                }
                _ => {}
            }
            Ok(())
        })
        .run()
        .unwrap();
}
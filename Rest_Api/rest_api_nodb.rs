use serde::{Deserialize, Serialize};
use warp::{Filter};

#[tokio::main]
async fn main() {
    let items = warp::path("users")
        .and(warp::path::end())
        .and(warp::get())
        .map(|| {
            let users = users();
            warp::reply::json(&users)
        });

    let get_user_name = warp::path("getUsersName")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query::<u32>())
        .map(|id| {
            let users_list = users();
            match users_list.get(id as usize) {
                Some(user) => warp::reply::json(&user),
                None => warp::reply::json(&"User not found"),
            }
        });

    let users_route = items.or(get_user_name);

    let users = warp::path("users")
        .and(users_route);

    let websocket = warp::path("users")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|mut socket| async move {
                if let Err(e) = socket.send(warp::ws::Message
                ::text("hello")).await {
                    eprintln!("websocket send error: {}", e);
                }
            })
        });

    warp::serve(users.or(websocket))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
}

fn users() -> Vec<User> {
    vec![
        User {
            username: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            username: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ]
}
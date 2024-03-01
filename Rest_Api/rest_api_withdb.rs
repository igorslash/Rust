use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    // Создание пула подключений к базе данных PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://username:password@localhost/database")
        .await
        .expect("Failed to create pool");

    // Конечные точки REST API
    let users = warp::path("users")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(get_users_handler(pool.clone()));

    let get_user_name = warp::path("getUsersName")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query::<u32>())
        .and_then(get_user_by_id_handler(pool.clone(), 1));

    let delete_user = warp::path("deleteUser")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query::<u32>())
        .and_then(delete_user_by_id_handler(pool.clone(), 2));

    let routes = users.or(get_user_name);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
}

async fn get_users_handler(pool: sqlx::PgPool) -> Result<impl Reply, Rejection> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            warp::reject()
        })?;
    Ok(warp::reply::json(&users))
}

async fn get_user_by_id_handler(pool: sqlx::PgPool,
                                id: u32) -> Result<impl Reply, Rejection> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id as i32)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            warp::reject()
        })?;
    match user {
        Some(u) => Ok(warp::reply::json(&u)),
        None => Ok(warp::reply::json(&"User not found")),
    }
}
async fn add_user(pool: sqlx::PgPool, user: User) -> Result<impl
warp::Reply, Rejection> {
    let user = sqlx::query_as::<_, User>("INSERT INTO \
    users (username, email) VALUES ($1, $2) RETURNING *")
        .bind(user.username)
        .bind(user.email)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            warp::reject()
        })?;
    Ok(warp::reply::json(&user))
}
async fn delete_user_by_id_handler(pool: sqlx::PgPool,
                                    id: u32) -> Result<impl Reply, Rejection> {
    let user = sqlx::query_as::<_, User>("DELETE FROM users WHERE id = $1")
        .bind(id as i32)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            warp::reject()
        })?;
    match user {
        Some(u) => Ok(warp::reply::json(&u)),
        None => Ok(warp::reply::json(&"User not found")),
    }
}
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432")
        .await?;
    let user = sqlx::query_as::<_, User>
        ("SELECT id, email FROM users WHERE id = $1")
        .bind(1)
        .fetch_one(&pool)
        .await?;
    for users in user {
        println!("{:?}", users);
    }
    Ok(())

}
#[derive(Debug, Serialize,Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    email: String,
}
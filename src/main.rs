use bookstore::db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = db::pool::create_database_pool();

    let user_list = db::repositories::user::load_users(pool);

    println!("Loaded Users {:?}", user_list);
    
    Ok(())
}


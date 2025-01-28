#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    env_logger::init();   
}

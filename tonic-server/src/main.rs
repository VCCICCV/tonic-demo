mod server_lib;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server_lib::start_server().await?;
    Ok(())
}
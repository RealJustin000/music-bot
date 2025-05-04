
mod handler;
mod commands;
mod utils;

use serenity::prelude::*;
use songbird::SerenityInit;
use commands::GENERAL_GROUP;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Missing token in .env");

    let framework = serenity::framework::StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(handler::Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        eprintln!("Client error: {:?}", e);
    }
}

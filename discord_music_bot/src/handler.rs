
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

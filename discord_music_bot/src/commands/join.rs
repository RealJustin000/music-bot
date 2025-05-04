
use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::SerenityInit;

#[command]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let channel_id = match msg.author.voice_state(&ctx.cache).await.and_then(|vs| vs.channel_id) {
        Some(c) => c,
        None => {
            msg.reply(ctx, "You're not in a voice channel!").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await.unwrap().clone();
    let _ = manager.join(guild_id, channel_id).await;

    msg.reply(ctx, "🔊 Joined voice channel!").await?;
    Ok(())
}

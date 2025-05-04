
use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::{input::ffmpeg, SerenityInit};

#[command]
pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let url = msg.content.split_whitespace().nth(1);
    if url.is_none() {
        msg.reply(ctx, "❌ Provide a URL to play!").await?;
        return Ok(());
    }
    let url = url.unwrap();

    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    let handler_lock = manager.get(guild_id).ok_or("Not in a voice channel")?;
    let mut handler = handler_lock.lock().await;

    let source = ffmpeg(url).await.map_err(|e| {
        eprintln!("Error: {:?}", e);
        "Failed to play audio"
    })?;

    handler.play_source(source);
    msg.reply(ctx, "🎶 Now playing!").await?;

    Ok(())
}

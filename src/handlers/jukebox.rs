pub struct Jukebox;
use serenity::all::Ready;

use {
    crate::command,
    serenity::all::{Context, Message},
    songbird::input::YoutubeDl,
};

#[serenity::async_trait]
impl serenity::all::EventHandler for Jukebox {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        debug!("Bot is ready!");
    }

    async fn message(&self, ctx: Context, message: Message) {
        command_play(&message, &ctx).await;
        command_join(&message, &ctx).await;
    }
}

async fn command_play(message: &Message, ctx: &Context) {
    let Some(args) = command::parse(
        message,
        "play",
        command::Case::Insensitive,
        command::Prefix::Yes,
    ) else {
        return;
    };

    debug!("Play");

    if args.len() != 1 {
        if let Err(why) = message
            .reply(
                &ctx.http,
                "Expected 1 argument, please specify a song name or link to play",
            )
            .await
        {
            error!("Could not send error message due to: {why}");
        }
        return;
    }
    let url = args.first().unwrap().to_string();

    play(ctx, message, url).await;
}

async fn play(ctx: &Context, msg: &Message, url: String) {
    let do_search = !url.starts_with("http");

    let guild_id = msg.guild_id.unwrap();

    let http_client = {
        let data = ctx.data.read().await;
        data.get::<crate::HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        if let Err(e) = msg
            .channel_id
            .say(&ctx.http, "Not in a voice channel to play in")
            .await
        {
            error!("{e}")
        }
        return;
    };

    let mut handler = handler_lock.lock().await;

    let src = if do_search {
        YoutubeDl::new_search(http_client, url)
    } else {
        YoutubeDl::new(http_client, url)
    };

    let _ = handler.play_input(src.clone().into());

    if let Err(e) = msg.channel_id.say(&ctx.http, "Playing song").await {
        error!("{e}")
    }
}

async fn command_join(message: &Message, ctx: &Context) {
    let Some(_args) = command::parse(
        message,
        "join",
        command::Case::Insensitive,
        command::Prefix::Yes,
    ) else {
        return;
    };

    let (guild_id, channel_id) = {
        let guild = message.guild(&ctx.cache).unwrap();
        let channel_id = guild
            .voice_states
            .get(&message.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            if let Err(e) = message.reply(ctx, "Not in a voice channel").await {
                error!("Failed to send a message due to: {e}");
            }

            return;
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(_handler_lock) = manager.join(guild_id, connect_to).await {
        // Attach an event handler to see notifications of all track errors.
        error!("Failed to join");
    }
}

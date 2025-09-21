#[macro_use]
extern crate log;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;
use reqwest::Client as HttpClient;
use serenity::all::{ActivityData, Client, GatewayIntents};
use songbird::SerenityInit;

mod command;
mod config;
mod handlers;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn setup_loggers() {
    use {log::LevelFilter, std::time::Duration};

    const DEP_FILTERS: &[(&str, LevelFilter)] = &[
        ("log_panics", LevelFilter::Trace),
        ("serenity", LevelFilter::Warn),
        ("h2", LevelFilter::Error),
        ("tokio", LevelFilter::Warn),
        ("hyper", LevelFilter::Warn),
        ("tungstenite", LevelFilter::Warn),
        ("reqwest", LevelFilter::Warn),
        ("rustls", LevelFilter::Warn),
    ];

    logger::init([
        logger::Config::default()
            .output(logger::Output::Stdout)
            .colored(true)
            .filters(DEP_FILTERS),
        logger::Config::default()
            .output(logger::Output::new_timed_file(
                "./logs/jukebot.log",
                Duration::from_secs(86400), // 1day
            ))
            .colored(false)
            .filters(DEP_FILTERS),
    ])
}

struct HttpKey;

impl serenity::prelude::TypeMapKey for HttpKey {
    type Value = HttpClient;
}

#[tokio::main]
async fn main() {
    setup_loggers();

    dotenv::dotenv().unwrap();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let cb = Client::builder(&token, intents)
        .register_songbird()
        .event_handler(handlers::Jukebox)
        .event_handler(handlers::Help)
        .type_map_insert::<HttpKey>(HttpClient::new())
        .status(serenity::all::OnlineStatus::DoNotDisturb)
        .activity(ActivityData::listening(format!(
            "{}help",
            command::DEFAULT_PREFIX
        )));

    let mut client = cb.await.unwrap();

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

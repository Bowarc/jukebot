use {
    crate::command,
    serenity::all::{Context, Message},
};

pub struct Help;

#[serenity::async_trait]
impl serenity::all::EventHandler for Help {
    async fn message(&self, ctx: Context, message: Message) {
        let Some(args) = command::parse(
            &message,
            "help",
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
                    "What do you need help with ? (expected 1 argument)",
                )
                .await
            {
                error!("Could not send error message due to: {why}");
            }
            return;
        }

        let content = match args.first().unwrap().to_lowercase().as_str() {
            "play" => String::from("TODO"),
            _ => {
                const SUBJECTS: &[&str] = &["play"];
                format!(
                    "Invalid option. Try with any of [{}].",
                    SUBJECTS
                        .iter()
                        .fold(String::new(), |mut s, subject| {
                            use std::fmt::Write as _;
                            write!(s, "{subject}, ").unwrap(); // Writing to a string cannot fail
                            s
                        })
                        .trim_end_matches([' ', ','])
                )
            }
        };

        if let Err(why) = message.reply(&ctx.http, content).await {
            error!("Could not send error message due to: {why}");
        }
    }
}

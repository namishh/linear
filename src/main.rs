use dotenv::dotenv;
mod commands;
mod handler;
use mongodb::Client as MongoClient;
use poise::builtins;
use poise::serenity_prelude as serenity;
use serenity::all::OnlineStatus;
use std::sync::Arc;

use serenity::model::gateway::GatewayIntents;

//mod seed;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub mongo: Arc<mongodb::Client>,
}

pub struct MongoClientKey;

impl poise::serenity_prelude::prelude::TypeMapKey for MongoClientKey {
    type Value = Arc<MongoClient>;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Initialize the bot with your Discord bot token

    let uri = std::env::var("DATABASE_URL").expect("No database url");
    let mongo = MongoClient::with_uri_str(uri)
        .await
        .expect("Error while connecting");

//    let _ = seed::seed(&mongo).await;

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::all();

    let mongo_arc = Arc::new(mongo);

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            manual_cooldowns: true,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("?c".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.data
                    .write()
                    .await
                    .insert::<MongoClientKey>(Arc::clone(&mongo_arc));
                Ok(Data {
                    mongo: Arc::clone(&mongo_arc),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .activity(serenity::gateway::ActivityData::listening("?c"))
        .status(OnlineStatus::Online)
        .framework(framework)
        .event_handler(handler::Handler::new())
        .await;

    client.unwrap().start().await.unwrap();
}

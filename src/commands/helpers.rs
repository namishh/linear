use crate::{Context, Error};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::Client as MongoClient;
use mongodb::Collection;
use poise::serenity_prelude::User;

pub async fn check_cooldown(ctx: &Context<'_>, seconds: u64) -> Result<(), Error> {
    let mut cooldown_tracker = ctx.command().cooldowns.lock().unwrap();

    let mut cooldown_durations = poise::CooldownConfig::default();
    cooldown_durations.user = Some(std::time::Duration::from_secs(seconds));

    match cooldown_tracker.remaining_cooldown(ctx.cooldown_context(), &cooldown_durations) {
        Some(remaining) => Err(format!(
            "Please wait for {} seconds before running the command again.",
            remaining.as_secs()
        )
        .into()),
        None => {
            cooldown_tracker.start_cooldown(ctx.cooldown_context());
            Ok(())
        }
    }
}

pub async fn get_user(user: &User, ctx: Context<'_>) -> Result<Vec<Document>, Error> {
    let db = ctx.data().mongo.clone();
    let client_ref: &MongoClient = db.as_ref();
    let db_ref = client_ref.database("linear");
    let collection: Collection<Document> = db_ref.collection("User");

    let mut cursor: Result<mongodb::Cursor<Document>, mongodb::error::Error>;
    cursor = collection.find(doc! { "user": user.tag()}, None).await;

    let mut current_presents: Vec<Document> = Vec::new();

    while let Ok(cursor) = &mut cursor {
        if let Some(doc) = cursor.try_next().await? {
            current_presents.push(doc);
        } else {
            break; // No more documents in the cursor, exit the loop
        }
    }

    Ok(current_presents)
}

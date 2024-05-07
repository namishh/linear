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
    cursor = collection.find(doc! { "username": user.tag()}, None).await;

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

pub async fn logged_in(user: &User, ctx: Context<'_>) -> bool {
    let user = match get_user(&user, ctx.clone()).await {
        Ok(user) => user,
        Err(_err) => {
            return false;
        }
    };
    if user.len() > 0 {
        if !user[0]
            .get_str("team_name")
            .expect("could not find name")
            .is_empty()
        {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub async fn get_team(name: &str, ctx: Context<'_>) -> Result<Vec<Document>, Error> {
    let db = ctx.data().mongo.clone();
    let client_ref: &MongoClient = db.as_ref();
    let db_ref = client_ref.database("linear");
    let collection: Collection<Document> = db_ref.collection("Teams");

    let mut cursor: Result<mongodb::Cursor<Document>, mongodb::error::Error>;
    cursor = collection.find(doc! { "name": name}, None).await;

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

pub async fn get_question(level: &i32, ctx: Context<'_>) -> Result<Vec<Document>, Error> {
    let db = ctx.data().mongo.clone();
    let client_ref: &MongoClient = db.as_ref();
    let db_ref = client_ref.database("linear");
    let collection: Collection<Document> = db_ref.collection("Questions");

    let mut cursor: Result<mongodb::Cursor<Document>, mongodb::error::Error>;
    cursor = collection.find(doc! { "level": level}, None).await;

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

pub async fn get_team_question(ctx: Context<'_>) -> Result<Vec<Document>, Error> {
    let author = &ctx.author();
    let user = match get_user(&author, ctx.clone()).await {
        Ok(user) => user,
        Err(err) => {
            return Err(err);
        }
    };
    let teamname = user[0]
        .get_str("team_name")
        .unwrap_or("Failed To Fetch Team");

    let team = match get_team(&teamname, ctx.clone()).await {
        Ok(team) => team,
        Err(err) => {
            return Err(err);
        }
    };

    let level = team[0].get_i32("level").unwrap_or(0);

    let question = match get_question(&level, ctx.clone()).await {
        Ok(question) => question,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(question)
}

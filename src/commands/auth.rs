use crate::commands::helpers;
use crate::{Context, Error};
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::Client as MongoClient;
use mongodb::Collection;
use pwhash::bcrypt;

#[poise::command(slash_command, prefix_command, aliases("rt"))]
pub async fn register_team(
    ctx: Context<'_>,
    #[description = "Team Name?"] team_name: String,
    #[description = "Password"] password: String,
) -> Result<(), Error> {
    let author = &ctx.author();
    //helpers::check_cooldown(&ctx, 10).await?;

    let user = match helpers::get_user(&author, ctx.clone()).await {
        Ok(user) => user,
        Err(err) => {
            return Err(err);
        }
    };

    if user.is_empty() {
        ctx.say("No Such User, Register your self with ```?c register```")
            .await?;
    } else {
        let team = match helpers::get_team(&team_name, ctx.clone()).await {
            Ok(team) => team,
            Err(err) => {
                return Err(err);
            }
        };
        if !team.is_empty() {
            ctx.say("Team With This Name Already Exists").await?;
        } else if team_name.len() > 20 {
            ctx.say("Team Name Cannot Be More Than 20 Characters.")
                .await?;
        } else {
            let db = ctx.data().mongo.clone();
            let client_ref: &MongoClient = db.as_ref();
            let db_ref = client_ref.database("linear");
            let collection: Collection<Document> = db_ref.collection("Teams");
            let hashed = bcrypt::hash(password);
            let document = doc! {
                "password": hashed?.clone(),
                "name": team_name.clone()
            };
            let _ = collection.insert_one(document, None).await?;
            ctx.say(format!("Team **{}** registered", team_name))
                .await?;
        }
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, aliases("ru"))]
pub async fn register_user(ctx: Context<'_>) -> Result<(), Error> {
    let author = &ctx.author();
    helpers::check_cooldown(&ctx, 100).await?;

    let user = match helpers::get_user(&author, ctx.clone()).await {
        Ok(user) => user,
        Err(err) => {
            return Err(err);
        }
    };

    if user.is_empty() {
        let db = ctx.data().mongo.clone();
        let client_ref: &MongoClient = db.as_ref();
        let db_ref = client_ref.database("linear");
        let collection: Collection<Document> = db_ref.collection("User");
        let document = doc! {
            "username": author.tag(),
            "team_name": "",
        };
        let _ = collection.insert_one(document, None).await;
        ctx.say("User Successfully Registered").await?;
    } else {
        ctx.say("User Already Registered").await?;
    }
    Ok(())
}

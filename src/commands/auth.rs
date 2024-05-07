use crate::commands::helpers;
use crate::{Context, Error};
use chrono::Utc;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::Client as MongoClient;
use mongodb::Collection;
use pwhash::bcrypt;

#[poise::command(slash_command, prefix_command, aliases("rt"), help_text_fn = "help_rt")]
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
                "level": 0,
                "last_updated": Utc::now().timestamp_millis(),
                "name": team_name.clone(),
                "giveaways": Vec::<i32>::new(),
                "hints": Vec::<i32>::new(),
            };
            let _ = collection.insert_one(document, None).await?;
            ctx.say(format!("Team **{}** registered", team_name))
                .await?;
        }
    }
    Ok(())
}

fn help_rt() -> String {
    String::from(
        "\
Example Usage (Team Name and Password are one word):
?c register_team <team_name> <password>",
    )
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

// Login Command, takes in team name and password

#[poise::command(slash_command, prefix_command, aliases("lt"), help_text_fn = "help_lt")]
pub async fn login_team(
    ctx: Context<'_>,
    #[description = "Team Name"] team_name: String,
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

    if helpers::logged_in(author, ctx).await {
        ctx.say(format!(
            "Already Logged In With Team {}",
            user[0].get_str("team_name")?
        ))
        .await?;
    } else {
        let team = match helpers::get_team(&team_name, ctx.clone()).await {
            Ok(team) => team,
            Err(err) => {
                return Err(err);
            }
        };

        if team.is_empty() {
            ctx.say("No Such Team, Register your self with ```?c rt teamname password```")
                .await?;
        } else {
            let db = ctx.data().mongo.clone();
            let client_ref: &MongoClient = db.as_ref();

            let hashed = bcrypt::verify(
                password,
                team[0]
                    .get_str("password")
                    .unwrap_or("Could not verify password."),
            );

            if !hashed {
                ctx.say("Wrong Password").await?;
            } else {
                let filter = doc! { "username": author.tag() };
                let update = doc! { "$set": doc! {"team_name": &team_name} };
                let collection: Collection<Document> =
                    client_ref.database("linear").collection("User");
                collection.update_one(filter, update, None).await?;
                ctx.say(format!("Logged in as **{}**", team_name)).await?;
            }
        }
    }
    Ok(())
}

fn help_lt() -> String {
    String::from(
        "\
Example Usage (Team Name and Password are one word):
?c login_team <team_name> <password>",
    )
}

// Logout Command
#[poise::command(slash_command, prefix_command, aliases("lo"))]
pub async fn logout(ctx: Context<'_>) -> Result<(), Error> {
    let author = &ctx.author();
    let user = match helpers::get_user(&author, ctx.clone()).await {
        Ok(user) => user,
        Err(err) => {
            return Err(err);
        }
    };
    if !user.is_empty() {
        if !user[0].get_str("team_name")?.is_empty() {
            let db = ctx.data().mongo.clone();
            let client_ref: &MongoClient = db.as_ref();
            let db_ref = client_ref.database("linear");
            let collection: Collection<Document> = db_ref.collection("User");
            let filter = doc! { "username": author.tag() };
            let update = doc! { "$set": doc! {"team_name": ""} };
            let _ = collection.update_one(filter, update, None).await?;
            ctx.say("Logged Out").await?;
        } else {
            ctx.say("Not Logged In").await?;
        }
    } else {
        ctx.say("No Such User, Register your self with ```?c register```")
            .await?;
    }
    Ok(())
}

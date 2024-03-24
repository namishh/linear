use crate::commands::helpers;
use crate::{Context, Error};
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::Client as MongoClient;
use mongodb::Collection;
use pwhash::bcrypt;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::model::colour::Colour;

#[poise::command(slash_command, prefix_command, aliases("question"))]
pub async fn get_question(ctx: Context<'_>) -> Result<(), Error> {
    // helpers::check_cooldown(&ctx, 10).await?;
    let author = &ctx.author();
    if helpers::logged_in(author, ctx).await {
        let user = match helpers::get_user(&author, ctx.clone()).await {
            Ok(user) => user,
            Err(err) => {
                return Err(err);
            }
        };
        let teamname = user[0]
            .get_str("team_name")
            .unwrap_or("Failed To Fetch Team");

        let team = match helpers::get_team(&teamname, ctx.clone()).await {
            Ok(team) => team,
            Err(err) => {
                return Err(err);
            }
        };

        let level = team[0].get_i32("level").unwrap_or(0);

        let question = match helpers::get_question(&level, ctx.clone()).await {
            Ok(question) => question,
            Err(err) => {
                return Err(err);
            }
        };

        let embed = CreateEmbed::default()
            .title(format!(
                "Question {}: ",
                question[0].get_i32("level").unwrap_or(0)
            ))
            .description(
                question[0]
                    .get_str("question")
                    .unwrap_or("Could Not Find Question"),
            )
            .footer(CreateEmbedFooter::new(
                "Answer By Saying ?c answer <answer>",
            ))
            .color(Colour::BLURPLE);
        let builder = poise::CreateReply::default().embed(embed);
        let _msg = ctx.send(builder).await?;
    } else {
        ctx.say("Not Logged In With A Team").await?;
    }
    Ok(())
}

// WARN: Hardcoded number of questions
#[poise::command(slash_command, prefix_command, aliases("an"))]
pub async fn answer(ctx: Context<'_>, #[description = "Guess"] guess: String) -> Result<(), Error> {
    let author = &ctx.author();
    if helpers::logged_in(author, ctx).await {
        let user = match helpers::get_user(&author, ctx.clone()).await {
            Ok(user) => user,
            Err(err) => {
                return Err(err);
            }
        };
        let teamname = user[0]
            .get_str("team_name")
            .unwrap_or("Failed To Fetch Team");

        let team = match helpers::get_team(&teamname, ctx.clone()).await {
            Ok(team) => team,
            Err(err) => {
                return Err(err);
            }
        };

        let level = team[0].get_i32("level").unwrap_or(0);

        if level == 5 {
            let _ = ctx.say("You have already completed the game").await;
            return Ok(());
        }

        let question = match helpers::get_question(&level, ctx.clone()).await {
            Ok(question) => question,
            Err(err) => {
                return Err(err);
            }
        };

        if !question.is_empty() {
            if bcrypt::verify(guess, question[0].get_str("answer").unwrap_or("")) {
                let db = ctx.data().mongo.clone();
                let client_ref: &MongoClient = db.as_ref();
                let collection: Collection<Document> =
                    client_ref.database("linear").collection("Teams");

                let filter = doc! { "name": teamname };
                let update = doc! { "$set": doc! {"level":  level + 1} };

                collection.update_one(filter, update, None).await?;

                if level + 1 == 5 {
                    ctx.say("Congrats! You have completed the game").await?;
                } else {
                    ctx.say(format!(
                        "Correct Answer. Promoted to level {}",
                        &team[0].get_i32("level").unwrap_or(0) + 1
                    ))
                    .await?;
                }
            } else {
                ctx.say("Wrong Answer").await?;
            }
        } else {
            ctx.say("Invalid Question (Hint: first question is 0)")
                .await?;
        }
    } else {
        ctx.say("Not Logged In With A Team").await?;
    }
    Ok(())
}

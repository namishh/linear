use crate::commands::helpers;
use crate::{Context, Error};
use futures::stream::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::options::FindOptions;
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
#[poise::command(
    slash_command,
    prefix_command,
    aliases("an"),
    help_text_fn = "help_answer"
)]
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
                let update = doc! { "$set": doc! {"level":  level + 1, "last_updated": chrono::Utc::now().timestamp_millis()} };

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

fn help_answer() -> String {
    String::from(
        "\
example usage:
?c answer <answer>",
    )
}

#[poise::command(slash_command, prefix_command, aliases("lb"))]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let db = ctx.data().mongo.clone();
    let client_ref: &MongoClient = db.as_ref();
    let collection: Collection<Document> = client_ref.database("linear").collection("Teams");

    let sort_criteria = doc! {
        "level": -1,
        "last_updated": 1
    };

    let options = FindOptions::builder().sort(sort_criteria).build();
    let mut cursor = collection.find(None, options).await?;

    let mut embed = CreateEmbed::default().title("👑 LeaderBoard");
    let mut m = CreateEmbed::default();

    let mut leaderboard: Vec<Document> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                leaderboard.push(document.clone()); // Cloning the document to print it later
            }
            Err(e) => {
                eprintln!("Error fetching team: {}", e);
            }
        }
    }

    for (index, document) in leaderboard.iter().enumerate() {
        embed = embed.field(
            format!(
                "#{} Team {} ",
                index + 1,
                document.get_str("name").unwrap_or("NIL")
            ),
            format!("{}", document.get_i32("level").unwrap_or(0) * 1000),
            true,
        );
        m = embed.clone()
    }

    let builder = poise::CreateReply::default().embed(m);
    ctx.send(builder).await?;

    Ok(())
}

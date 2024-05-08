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

#[poise::command(slash_command, guild_only, prefix_command, aliases("question"))]
pub async fn get_question(ctx: Context<'_>) -> Result<(), Error> {
    // helpers::check_cooldown(&ctx, 10).await?;
    let author = &ctx.author();
    if helpers::logged_in(author, ctx).await {
        let question = helpers::get_team_question(ctx).await?;
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
    guild_only,
    aliases("an"),
    help_text_fn = "help_answer"
)]
pub async fn answer(ctx: Context<'_>, #[description = "Guess"] guess: String) -> Result<(), Error> {
    let author = &ctx.author();
    if helpers::logged_in(author, ctx).await {
        let team = match helpers::get_team_by_user(ctx).await {
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

                let filter = doc! { "name": team[0].get_str("name").expect("LOL") };
                let update = doc! { "$set": doc! {"level":  level + 1, "points": team[0].get_i32("points").unwrap_or(0) + 1000, "last_updated": chrono::Utc::now().timestamp_millis()} };

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

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn hint(ctx: Context<'_>) -> Result<(), Error> {
    let author = &ctx.author();
    if helpers::logged_in(author, ctx).await {
        let db = ctx.data().mongo.clone();
        let client_ref: &MongoClient = db.as_ref();
        let collection: Collection<Document> = client_ref.database("linear").collection("Teams");
        let mut team = match helpers::get_team_by_user(ctx).await {
            Ok(team) => team,
            Err(err) => {
                return Err(err);
            }
        };
        let used_hints = team[0]
            .get_array_mut("hints")
            .expect("Could not fetch used hints");

        let mut uh = used_hints.clone();
        let question_arr = helpers::get_team_question(ctx).await?;
        let question = &question_arr[0];

        let curr_level = question
            .get_i32("level")
            .expect("curr level not found: hint");

        let hint = question.get_str("hint").expect("Cannot get hint");

        if used_hints
            .iter()
            .any(|i| i == &mongodb::bson::Bson::Int32(curr_level))
        {
            let _ = ctx
                .say(format!(
                    "Hint Already Given! No Further Points Would be Deducted.\n HINT: {}",
                    hint
                ))
                .await;
        } else {
            uh.append(&mut vec![mongodb::bson::Bson::Int32(curr_level)]);
            let filter = doc! { "name": &team[0].get_str("name").expect("LOL") };
            let update = doc! { "$set": doc! {"hints": uh, "points": &team[0].get_i32("points").unwrap_or(0) - 100}};
            collection.update_one(filter, update, None).await?;
            let _ = ctx
                .say(format!("100 Points Would Be Deducted.\n HINT: {}", hint))
                .await;
        }
    } else {
        ctx.say("Not Logged In With A Team").await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn giveaway(ctx: Context<'_>) -> Result<(), Error> {
    let author = &ctx.author();
    if helpers::logged_in(author, ctx).await {
        let db = ctx.data().mongo.clone();
        let client_ref: &MongoClient = db.as_ref();
        let collection: Collection<Document> = client_ref.database("linear").collection("Teams");
        let mut team = match helpers::get_team_by_user(ctx).await {
            Ok(team) => team,
            Err(err) => {
                return Err(err);
            }
        };
        let used_hints = team[0]
            .get_array_mut("giveaways")
            .expect("Could not fetch used hints");

        let mut uh = used_hints.clone();
        let question_arr = helpers::get_team_question(ctx).await?;
        let question = &question_arr[0];

        let curr_level = question
            .get_i32("level")
            .expect("curr level not found: hint");

        let hint = question.get_str("giveaway").expect("Cannot get hint");

        if used_hints
            .iter()
            .any(|i| i == &mongodb::bson::Bson::Int32(curr_level))
        {
            let _ = ctx
                .say(format!(
                    "Giveaway Already Given! No Further Points Would be Deducted.\n HINT: {}",
                    hint
                ))
                .await;
        } else {
            uh.append(&mut vec![mongodb::bson::Bson::Int32(curr_level)]);
            let filter = doc! { "name": &team[0].get_str("name").expect("LOL") };
            let update = doc! { "$set": doc! {"giveaways": uh, "points": &team[0].get_i32("points").unwrap_or(0) - 100}};
            collection.update_one(filter, update, None).await?;
            let _ = ctx
                .say(format!("100 Points Would Be Deducted.\n HINT: {}", hint))
                .await;
        }
    } else {
        ctx.say("Not Logged In With A Team").await?;
    }
    Ok(())
}

fn divide_into_parts<T: Clone>(vec: Vec<T>, n: usize) -> Vec<Vec<T>> {
    vec.chunks(n).map(|chunk| chunk.into()).collect()
}

#[poise::command(slash_command, guild_only, prefix_command, aliases("lb"))]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let db = ctx.data().mongo.clone();
    let client_ref: &MongoClient = db.as_ref();
    let collection: Collection<Document> = client_ref.database("linear").collection("Teams");

    let sort_criteria = doc! {
        "points": -1,
        "last_updated": 1
    };

    let options = FindOptions::builder().sort(sort_criteria).build();
    let mut cursor = collection.find(None, options).await?;

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

    let mut pages = Vec::<String>::new();
    let j = divide_into_parts(leaderboard.clone(), 4);

    for (_index, column) in j.iter().enumerate() {
        let mut page = "".to_owned();
        for (_i, doc) in column.iter().enumerate() {
            let fin = format!(
                "Team Name: {}\n**Points** - {}\n\n",
                doc.get_str("name").unwrap_or("NIL"),
                doc.get_i32("points").unwrap_or(0),
            );
            page.push_str(&fin);
        }

        pages.push(page);
    }

    let vec_of_strs: Vec<&str> = pages.iter().map(|s| s.as_str()).collect();

    poise::samples::paginate(ctx, &vec_of_strs).await?;

    Ok(())
}

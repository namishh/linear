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
                "Answer By Saying ?c answer <level> <answer>",
            ))
            .color(Colour::BLURPLE);
        let builder = poise::CreateReply::default().embed(embed);
        let _msg = ctx.send(builder).await?;
    } else {
        ctx.say("Not Logged In With A Team").await?;
    }
    Ok(())
}

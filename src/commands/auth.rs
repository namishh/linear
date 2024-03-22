use crate::commands::helpers;
use crate::{Context, Error};
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::Client as MongoClient;
use mongodb::Collection;
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command, aliases("rt"))]
pub async fn register_team(
    ctx: Context<'_>,
    #[description = "Team Name?"]
    #[rest]
    team_name: String,
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
        //let db = ctx.data().mongo.clone();
        //let client_ref: &MongoClient = db.as_ref();
        //let db_ref = client_ref.database("linear");
        //let collection: Collection<Document> = db_ref.collection("Teams");
        // let document = doc! {};
        //let _ = collection.insert_one(document, None).await;
    }
    Ok(())
}

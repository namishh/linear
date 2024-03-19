use crate::commands::helpers;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    helpers::check_cooldown(&ctx, 10).await?;
    ctx.say(format!(
        "Pong for {}!",
        user.as_ref().unwrap_or_else(|| ctx.author()).tag()
    ))
    .await?;
    Ok(())
}

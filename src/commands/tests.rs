use crate::commands::helpers;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::builder::CreateEmbed;
use serenity::model::colour::Colour;

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

#[poise::command(slash_command, prefix_command, aliases("av"))]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let icon_url = user
        .avatar_url()
        .unwrap_or_else(|| user.default_avatar_url());
    let embed = CreateEmbed::default()
        .title(&user.tag())
        .image(icon_url)
        .color(Colour::BLURPLE);
    let builder = poise::CreateReply::default().embed(embed);
    let _msg = ctx.send(builder).await?;
    Ok(())
}

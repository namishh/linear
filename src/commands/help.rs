use crate::{Context, Error};
#[poise::command(slash_command, guild_only, prefix_command, aliases("ht"))]
pub async fn howto(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("How to play!\n## Authentication\n1. Register Yourself - `?c register_user`\n2. Register Your Team - `?c register_team teamname password`\n3. Login With Your Team - `?c login_team teamname password`\n4. To Logout - `?c logout`\n## Playing The Game\n1. Get Question - `?c question`\n2. Answer Question - `?c answer yourguess`\n3. View Leaderboard - `?c leaderboard`\n4. Get a hint (-100 Points) - `?c hint`\n5. Get a giveaway (-100 Points) - `?c giveaway` \n\nHave Fun!").await?;
    Ok(())
}

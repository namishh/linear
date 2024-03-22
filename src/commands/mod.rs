pub mod auth;
pub mod helpers;
pub mod tests;
use crate::Data;

pub fn get_commands(
) -> Vec<poise::Command<Data, Box<dyn std::error::Error + std::marker::Send + Sync>>> {
    let commands: Vec<poise::Command<Data, Box<dyn std::error::Error + std::marker::Send + Sync>>> =
        vec![tests::ping(), tests::avatar(), auth::register_team()];
    commands
}

use crate::{Context, Error};

pub async fn check_cooldown(ctx: &Context<'_>, seconds: u64) -> Result<(), Error> {
    let mut cooldown_tracker = ctx.command().cooldowns.lock().unwrap();

    // You can change the cooldown duration depending on the message author, for example
    let mut cooldown_durations = poise::CooldownConfig::default();
    cooldown_durations.user = Some(std::time::Duration::from_secs(seconds));

    match cooldown_tracker.remaining_cooldown(ctx.cooldown_context(), &cooldown_durations) {
        Some(remaining) => Err(format!("Please wait {} seconds", remaining.as_secs()).into()),
        None => {
            cooldown_tracker.start_cooldown(ctx.cooldown_context());
            Ok(())
        }
    }
}


use poise::async_trait;
use serenity::client;
use serenity::model::gateway::Ready;

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl client::EventHandler for Handler {
    async fn ready(&self, _ctx: client::Context, ready: Ready) {
        println!("{} is online", ready.user.tag());
    }
}

use std::env;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
  async fn message(&self, context: Context, msg: Message) {
    if msg.content == "!ping" {
      if let Err(why) = msg.channel_id.say(&context.http, "Pong! :3").await {
        println!("Error sending message: {:?}", why);
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token.");

  let intents: GatewayIntents = 
    GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

  let mut client =
  Client::builder(token, intents).event_handler(Handler).await.expect("Err creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
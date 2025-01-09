use std::env;

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
  async fn message(&self, context: Context, msg: Message) {
    if msg.content.contains("0w0 ping") {
      let channel = match msg.channel_id.to_channel(&context).await {
        Ok(channel) => channel,
        Err(why) => {
          println!("Error getting channel: {why:?}");
          return;
        },
      };

      let response = MessageBuilder::new()
        .push("User ")
        .mention(&msg.author)
        .push(" used the 'ping' command in the ")
        .mention(&channel)
        .push(" channel")
        .build();

      
      if let Err(why) = msg.channel_id.say(&context.http, &response).await {
        println!("Error sending message: {why:?}");
      }
      if let Err(why) = msg.reply(&context.http, &response).await {
        println!("Error sending message: {why:?}");
      }
    }
    match msg.mentions_me(&context.http).await {
      Ok(true) => {
        let response = MessageBuilder::new()
        .push("Hello ")
        .mention(&msg.author)
        .push(", wazzup??")
        .build();
        
        if let Err(why) = msg.reply(&context.http, &response).await {
          println!("Error sending message: {why:?}");
        }
      },
      Ok(false) => {
          // No need to do anything if the bot wasn't mentioned.
      }
      Err(why) => {
          println!("Error checking mentions: {:?}", why);
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{}, all systems go!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token.");

  let intents: GatewayIntents = 
    GatewayIntents::GUILD_MESSAGES 
    | GatewayIntents::MESSAGE_CONTENT
    | GatewayIntents::DIRECT_MESSAGES;

  let mut client =
  Client::builder(token, intents).event_handler(Handler).await.expect("Err creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
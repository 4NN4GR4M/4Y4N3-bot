use std::env;

use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::guild;
use serenity::model::prelude::{Reaction, ReactionType};
use serenity::{builder, prelude::*};
use serenity::model::guild::Member;
use serenity::utils::{audit_log, Emoji, MessageBuilder, Role};

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

  async fn reaction_add(&self, context: Context, reaction: Reaction) {
    println!("{:?}", reaction.emoji.to_string());
    if let ReactionType::Unicode(ref emoji) =  reaction.emoji {
      if emoji == "1️⃣" {
        let reaction_user = match reaction.user(&context.http).await {
          Ok(reaction_user) => reaction_user,
          Err(why) => {
            println!("Error getting user. {why:?}");
            return;
          },
        };
        let response = MessageBuilder::new()
        .mention(&reaction_user)
        .build();
        if let Err(why) = reaction.channel_id.say(&context.http, &response).await {
          println!("Error sending reaction response: {why:?}");
          return;
        }
        if let Some(guild_id) = reaction.guild_id {
          let guild_id_u64 = guild_id.0;
          let user_id_u64 = reaction_user.id.0;
          let role_id_u64 = 1327345393415225459;

          if let Err(why) = context.http.add_member_role(guild_id_u64, user_id_u64, role_id_u64, None).await {
            println!("Error adding member role. {why:?}");
            return;
          }
        }
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
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::GUILD_MESSAGE_REACTIONS
    | GatewayIntents::GUILD_MEMBERS
    | GatewayIntents::GUILD;

  let mut client =
  Client::builder(token, intents).event_handler(Handler).await.expect("Err creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
use core::panic;
use std::collections::HashMap;
use std::env;

use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::guild;
use serenity::model::prelude::{Reaction, ReactionType};
use serenity::model::user::User;
use serenity::{builder, prelude::*};
use serenity::model::guild::Member;
use serenity::utils::{audit_log, Emoji, MessageBuilder, Role};

struct Handler;

struct Utilities {
  reaction_menus: HashMap<u64, bool>,
}


#[serenity::async_trait]
impl EventHandler for Handler {
  async fn message(&self, context: Context, msg: Message) {
    let mut msg_breakdown= msg.content.split_whitespace().next();
    println!("{:?}", msg_breakdown);
    match msg_breakdown {
      Some("->") => { }
      None => {
        return;
      }
      _ => {
        return;
      }
    }

    if msg.content.contains("ping") {
      let channel = match msg.channel_id.to_channel(&context.http).await {
        Ok(channel) => channel,
        Err(err) => {
          eprintln!("channel could not be retrieved. {:?}", err);
          return;
        }
      };
      let response = MessageBuilder::new()
        .push("User ")
        .mention(&msg.author)
        .push(" used the 'ping' command in the ")
        .mention(&channel)
        .push(" channel")
        .build();

      if let Err(why) = msg.reply(&context.http, &response).await {
        println!("Error sending message: {why:?}");
      }
    }

    // Must check if the user sending the message has the "admin" role.
    // Make a command parser to ensure commands are properly executed and in the correct order.
    if msg.content.contains("vibe-check") {
      let response = MessageBuilder::new()
      .push("VIBE CHECK :3")
      .push("\n*Drops grenade in an orphanage*")
      .build();
      
      if let Err(why) = msg.channel_id.say(&context.http, &response).await {
        println!("Error sending vibe check. {why:?}");
      }
    }

    if msg.content.contains("pat") {
      if let true = msg.mentions.len() == 0 {
        return;
      }
      let users_mentioned = msg.mentions;
      let user_no = users_mentioned.len();
      println!("{:?}", users_mentioned);
      for user in users_mentioned {
        let response = MessageBuilder::new()
        .push("*Pats ")
        .mention(&user)
        .push(" aggressively.*")
        .build();

        msg.channel_id.say(&context.http, &response).await;
      }
    }
    if msg.content.contains("generate-role-menu") {
      // Here, we will map in a hashmap, the codes of each reaction emoji in the message, to their corresponding role. 
      // Once the bot prints the role selection menu, it will take it's message id and store it in a hashmap that indicate role selection menus.
      // The .reactionAdd async function is going to check if the reaction was done to a role selection menu message, in the correct channel.
      // If it was, then it will extract the reaction emoji code, and look it up in the emoji: role hashmap to gain the correct role id.
      // Then it will assign the user that added the reaction, that role.
      let mut msg_breakdown: Vec<&str> = msg.content.split_whitespace().collect();
      println!("{:?}", msg_breakdown);
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
    | GatewayIntents::GUILD_MEMBERS;

  let mut client =
  Client::builder(token, intents).event_handler(Handler).await.expect("Err creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
use std::env;
use rust_discord_bot::{consts, CatJson};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    match msg.content.as_str().trim() {
      consts::HELP_COMMAND => {
        let response = msg.channel_id.say(&ctx.http, consts::HELP_MESSAGE).await;
        match response {
          Err(why) => {
            println!("Error sending message: {:?}", why);
          }
          Ok(_val) => println!("Successfully sent help msg"),
        }
      }
      consts::CAT_COMMAND => {
        let body = reqwest::get("https://api.thecatapi.com/v1/images/search").await;
        match body {
          Ok(val) => {
            let url = val.json::<Vec<CatJson>>().await;
            match url {
              Ok(data) => {
                let response = msg.channel_id.say(&ctx.http, &data.get(0).unwrap().url).await;
                match response {
                  Err(why) => {
                    println!("Error sending message: {:?}", why);
                  }
                  Ok(_val) => println!("Successfully sent cat msg"),
                }
              }
                ,
              Err(why) => println!("Failed to parse response: {}", why)
            }
          }
          Err(why) => println!("Couldn't get cat api response, {}", why)
        }
      }
      _ => println!("Illegal command"),
    }
}

async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN")
  .expect("Expected a token in the environment");

  let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
  let mut client = Client::builder(&token, intents)
  .event_handler(Handler)
  .await
  .expect("Err creating client");

  if let Err(why) = client.start().await {
      println!("Client error: {:?}", why);
  }
}
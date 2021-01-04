use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use std::io::Read;
use select::document::Document;
use select::predicate::Name;
use rand::Rng;

const HELP_MESSAGE: &str = "
Hello there, I'm a friendly llama!";

const HELP_COMMAND: &str = "%help";
const SHOW_COMMAND: &str = "%pokazlame";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == HELP_COMMAND {
      if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
          println!("Error sending message: {:?}", why);
    }
    if msg.content == SHOW_COMMAND {
        let image = findimage().unwrap();
        println!("{}", image);
        if let Err(why) = msg.channel_id.say(&ctx.http, image).await {
            println!("Error sending message: {:?}", why);
      }
    }
  }
}

async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
    }
}

fn findimage() -> Result<String, Box<dyn std::error::Error>>  {
    let mut res = reqwest::blocking::get("https://www.google.com/search?q=llama&sclient=img&tbm=isch")?;
    let mut body = String::new();
    let mut rng = rand::thread_rng();
    res.read_to_string(&mut body)?;

    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);
    
    let body = Document::from(body.as_str());
    let image = body
    .find(Name("img"))
    .filter_map(|n| n.attr("src")) // this is an iterator
    .nth(rng.gen_range(0..10)); // TODO: handle None scenario

    return Ok(String::from(image.unwrap()));
}

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN")
  .expect("Expected a token in the environment");

  let mut client = Client::new(&token)
  .event_handler(Handler)
  .await
  .expect("Err creating client");

  if let Err(why) = client.start().await {
      println!("Client error: {:?}", why);
  }
}
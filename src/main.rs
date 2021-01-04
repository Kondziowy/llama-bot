use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use rand::Rng;
use select::document::Document;
use select::predicate::Name;

const HELP_MESSAGE: &str = "
Hello there, I'm a friendly llama!";

const HELP_COMMAND: &str = "%help";
const SHOW_COMMAND: &str = "%pokazlame";
const YESNO_COMMAND: &str = "lame czy";

const YESNO_RESPONSES: [&str; 10] = ["Tak.", "Nie.", "Oczywiście!", "Zapomnij!", "да!", "Нет!", "Jeszcze jak!", "No chyba nie.", "Ja!", "Nein!"];

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Got {}", msg.content);
        let mut response = String::new();
        match msg.content.as_str() {
            HELP_COMMAND => {
                response = String::from(HELP_MESSAGE);
            }
            SHOW_COMMAND => {
                response = String::from(findimage().await.unwrap());
            }
            &_ => {
                if msg.content.starts_with(YESNO_COMMAND) {
                    let reply = format!(":8ball: {}", YESNO_RESPONSES[rand::thread_rng().gen_range(0..10)]);
                    if let Err(why) = msg.reply(&ctx.http, reply).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
        if response.len() == 0 {
            return;
        }
        println!("Sending response {}", response);
        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn findimage() -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::get("https://www.google.com/search?q=llama&sclient=img&tbm=isch").await;
    let random_num = rand::thread_rng().gen_range(0..10);
    let body = res.unwrap().text().await.unwrap();

    let body = Document::from(body.as_str());
    let image = body
        .find(Name("img"))
        .filter_map(|n| n.attr("src")) // this is an iterator
        .nth(random_num); // TODO: handle None scenario

    return Ok(String::from(image.unwrap()));
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

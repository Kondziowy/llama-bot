#![type_length_limit="1884939"]
use std::{convert::Infallible, env, net::SocketAddr};
use hyper::{Body, Request, Response, Server, service::{make_service_fn, service_fn}};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use rand::Rng;
use select::{document::Document, node::Node};
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
        // println!("Got {}", msg.content);
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

fn get_absolute_uri(n: Node) -> Option<&str> {
    match n.attr("src") {
        Some(link) => {
            if link.starts_with("http") {
                return Some(link);
            }
        }
        None => {
            return None;
        }
    }
    return None
}

async fn findimage() -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::get("https://www.google.com/search?q=llama&sclient=img&tbm=isch").await;
    
    let body = res.unwrap().text().await.unwrap();

    let body = Document::from(body.as_str());
    let image_iter = body
        .find(Name("img"))
        .filter_map(|n| get_absolute_uri(n)); // this is an iterator
    let random_num = rand::thread_rng().gen_range(0..image_iter.count());
    // TODO: find out how can we copy iterator instead of having to regenerate it
    let mut image_iter = body
        .find(Name("img"))
        .filter_map(|n| get_absolute_uri(n)); // this is an iterator
    let image = image_iter.nth(random_num); // TODO: handle None scenario

    return Ok(String::from(image.unwrap()));
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    // run simple http server to satisfy healthchecks
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });
    let server = Server::bind(&addr).serve(make_svc);
    
    let (server_result, client_result) = tokio::join!(server, client.start());

    if let Err(why) = client_result {
        println!("Client error: {:?}", why);
    }
    if let Err(why) = server_result {
        println!("Server error: {:?}", why);
    }
}

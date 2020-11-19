use std::env;
use log;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let event;
        match env::var("EVENT") {
            Ok(v) => event = v,
            Err(_) => panic!("Expected a EVENT in the environment"),
        }

        let reply;
        match env::var("REPLY") {
            Ok(v) => reply = v,
            Err(_) => panic!("Expected a REPLY in the environment"),
        }

        if msg.content == event {
            if let Err(why) = msg.channel_id.say(&ctx.http, &reply).await {
                println!("failed to send message: {:?}", why);
            } else {
                println!("{} -> {}", &event, &reply)
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a DISCORD_TOKEN in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");


    if let Err(why) = client.start().await {
        log::warn!("client error: {:?}", why);
    }
}


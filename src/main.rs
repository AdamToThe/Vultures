#![allow(deprecated)] 
use std::{env, fs::File, io::{BufRead, BufReader}, sync::Arc};
use::dotenvy::dotenv;
use markov::Chain;
use serenity::{
    all::{standard::Configuration, CreateAllowedMentions, CreateAttachment, CreateMessage, StandardFramework}, async_trait, model::prelude::{
        Message,
        Ready
    }, prelude::*
};
use rand::prelude::*;

mod commands;

/* currentuser user id is broken, we gonna have to stick to this */
const YS_USER_ID: u64 = 1258492646138314883;

struct Handler;

struct Shared {
    markov: Chain<String>
}

impl TypeMapKey for Shared {
    type Value = Arc<Shared>;
}

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }


    async fn message(&self, ctx: Context, msg: Message) -> () {
        let men = msg.mentions_me(&ctx.http).await;
        println!("{}", msg.author.id.get());
        

        if !(msg.author.id.get().eq(&YS_USER_ID)) {

            match men {
                Ok(b) => {
                    let random = {
                        let mut rng = rand::thread_rng();
                        rng.gen::<f64>()
                    };

                    println!("{}", random);

                    if b ||  random > 0.99 {
                        let shared = {
                            let data = ctx.data.read().await;

                            data.get::<Shared>()
                                .expect("No shared struct")
                                .clone()
                        };

                        let reply = shared.markov.generate_str();

                        msg.reply(&ctx.http, &reply).await.expect("couldnt autorespond. how sad");
                        ()
                    }
                },

                _ => {}
            };

            // racist ew, u replace the n_word urself
            if msg.content.to_lowercase().contains("n_word") || msg.content.to_lowercase().contains("n_word2")  {
                let pic = CreateAttachment::url(&ctx.http, "https://compote.slate.com/images/fb69a16d-7f35-4103-98c1-62d466196b9a.jpg?crop=590%2C375%2Cx0%2Cy0&width=840").await;

                if let Ok(mut p) = pic {
                    p.filename = String::from("mike.png");
                    let rep = CreateMessage::new().add_file(p);
                    
                    let _ = msg.channel_id.send_message(&ctx.http, 
                        rep
                        .reference_message(&msg)
                        .allowed_mentions(CreateAllowedMentions::new().replied_user(false))
                    ).await;
                };

                
            }
        }

        ()

    }
}


#[tokio::main]
async fn main() {
    dotenv().expect("No .env");

    let token = env::var("T").expect("token");

    let framework = StandardFramework::new()
            
            .group(&commands::FUN_GROUP);

    framework.configure(
        Configuration::new()
            .with_whitespace(true)
            .prefix(".")
        );

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let mut chain: Chain<String> = Chain::new();

    let file = File::open("msgs.txt").expect("no messages.txt file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                chain.feed_str(&line);
            }
            _ => continue
        }
    }


    

    {
        
        let mut data = client.data.write().await;
        data.insert::<Shared>(Arc::new(Shared {
            markov: chain
        }));
    }
    
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }


}

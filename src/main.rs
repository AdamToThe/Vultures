#![allow(deprecated)] 
use std::{env, fs::File, io::{BufRead, BufReader}, sync::Arc};
use::dotenvy::dotenv;

use markov::Chain;
use serenity::{
    async_trait, client::ClientBuilder, framework::StandardFramework, model::prelude::{
        Message,
        Ready
    }, prelude::*
};

use rand::prelude::*;

mod commands;


struct Handler;

struct Shared {
    markov: Chain<String>
}

impl TypeMapKey for Shared {
    type Value = Arc<Mutex<Shared>>;
}

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }


    async fn message(&self, ctx: Context, msg: Message) -> () {
        let men = msg.mentions_me(&ctx.http).await;
        let data = ctx.data.read().await;

        let me = ctx.cache.clone().current_user();

        let random = {
            let mut rng = rand::thread_rng();
            rng.gen::<f64>()
        };

        println!("{}", random);
        
        if msg.author.id.0 == me.id.0 {
            ()
        }

        match men {
            Ok(b) => {
                if b || random > 0.0910 {
                    let repl = {
                        data.get::<Shared>()
                            .expect("couldnt get the shared")
                            .lock()
                            .await
                            .markov
                            .generate_str()
                    };
                    
                    msg.reply(&ctx.http, &repl).await.expect("shit, cant reply with generated text o well");

                }
            },
            _ => {}
        };

        ()

    }
}
#[tokio::main]
async fn main() -> Result<(), serenity::Error> {
    dotenv().expect("No .env");

    let token = env::var("T").expect("No token");
    
    println!("Starting... ");

    let intents = GatewayIntents::all();


    

    let framework = StandardFramework::new()
            .configure(|c| c
                .with_whitespace(true)
                
                .prefix(".")
                
                .delimiters(vec![", ", ","])
            )
            .group(&commands::FUN_GROUP);
               

    
    
    let mut client = ClientBuilder::new(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await?;

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

    let shared = Shared {
        markov: chain
    };

    {
        
        let mut data = client.data.write().await;
        data.insert::<Shared>(Arc::new(Mutex::new(shared)));
    }


    client.start().await
    
}
#[macro_use] extern crate serenity;
extern crate trot_bot;  

use serenity::client::Client;
use std::env;
use serenity::model::{Message, permissions};
use serenity::client::Context;

fn main() {
    let mut client = Client::login_bot(&env::var("TROT_TOKEN").expect("token"));

    client.on_ready(|_ctx, ready| {
                println!("{} is connected!", ready.user.name);
    });

    client.with_framework(|f| f
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!" 
        .before(|ctx, msg, command_name| { 
            println!("Got command '{}' by user '{}'", command_name, msg.author.name);
            true 
        })
        .on("ping", trot_bot::ear::ping::ping)
        .command("list", |c| c
            .check(owner_check)
            .exec(trot_bot::brain::my_ls::my_ls)
        )
    );
    let _ = client.start();
}

fn owner_check(_: &mut Context, msg: &Message) ->  bool {
    let disc_id = &env::var("DISC_ID").expect("discord id");
    msg.author.id == disc_id.parse::<u64>().unwrap()
}

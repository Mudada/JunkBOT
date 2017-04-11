#[macro_use] extern crate serenity;
extern crate time;
extern crate postgres;

use std::env;
use postgres::{Connection, TlsMode};
use serenity::client::{Client, Context};
use serenity::model::Message;
mod cmds;

fn main() {

    let mut client = Client::login_bot(&env::var("TROT_TOKEN").expect("token"));
    let conn = Connection::connect(env::var("HOLLOW_DB").expect("database"), TlsMode::None).unwrap();

    client.on_ready(|_ctx, ready| {
        println!("{} is connected!", ready.user.name);
    });

    client.with_framework(|f| f
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!" 
        .before(|ctx, msg, command_name| { 
            let now = time::now().tm_hour.to_string() + "." + &time::now().tm_min.to_string();
            println!("[ {} ] - Got command '{}' by user '{}'", now, command_name, msg.author.name);
            true 
        })
        .on("help", help)
        .command("ping", |c| c.exec(cmds::ping::ping))
    );

    let _ = client.start();
}

command!(help(_context, msg) {
    msg.reply(
        "Hey bro, here are the commands:
        - !start, create your profil if you doesn't have one."
        ); 
});

fn owner_check(_: &mut Context, msg: &Message) ->  bool {
    let disc_id = &env::var("DISC_ID").expect("discord id");
    msg.author.id == disc_id.parse::<u64>().unwrap()
}

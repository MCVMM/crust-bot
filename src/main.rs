use std::env;

use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::*,
};

mod handlers;

#[group]
#[commands(ping)]
struct General;

fn main() {
    let mut client = Client::new(
        &env::var("DISCORD_TOKEN").expect("token"),
        handlers::Handler,
    )
    .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(&env::var("DISCORD_PREFIX").unwrap_or("~".to_string())))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Error has occured: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

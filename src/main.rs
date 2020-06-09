use std::env;

use serenity::{framework::standard::StandardFramework, prelude::*};

mod handlers;

mod commands {
    pub mod general;
}

fn main() {
    let mut client = Client::new(
        &env::var("DISCORD_TOKEN").expect("token"),
        handlers::Handler,
    )
    .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(&env::var("DISCORD_PREFIX").unwrap_or("~".to_string())))
            .group(&commands::general::GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Error has occured: {:?}", why);
    }
}

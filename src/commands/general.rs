use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::*;

#[group]
#[commands(ping, meminfo)]
struct General;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn meminfo(ctx: &mut Context, msg: &Message) -> CommandResult {
    use procinfo::pid::statm_self;

    match statm_self() {
        Ok(stats) => msg.reply(ctx, format!("Total memory used: {}", stats.size))?,
        Err(why) => msg.reply(ctx, format!("Meminfo error: {:?}", why))?,
    };
    Ok(())
}

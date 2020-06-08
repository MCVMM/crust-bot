use std::env;

use serenity::{
    prelude::*,
    model::channel::{Reaction, ReactionType, Message},
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group,
        }
    },
};

#[group]
#[commands(ping)]
struct General;

fn get_message_from_cache(ctx: &Context, reaction: &Reaction) -> Option<Message> {
    let cache_lock = ctx.cache.read();
    let message = cache_lock.messages
    .get(&reaction.channel_id)
    .map(|hm| hm.get(&reaction.message_id))
    .flatten();

    message.cloned()
}

fn emoji_count(ctx: &Context, reaction: &Reaction, emoji: &str) -> Option<(Message, u64)> {
    if emoji != "📌" {
        return None
    }

    let message = get_message_from_cache(&ctx, &reaction)
    .unwrap_or_else(|| reaction.message(&ctx.http).unwrap());
    let pin_emoji_count = message
    .reactions
    .iter()
    .find(|e| &e.reaction_type.as_data() == emoji) 
    .map_or(0, |v| v.count);

    Some((message, pin_emoji_count))
}

struct Handler;

impl EventHandler for Handler {
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        println!("Reaction: {:?}", reaction);
        if let ReactionType::Unicode(emoji) = &reaction.emoji {
            if let Some((msg, count)) = emoji_count(&ctx, &reaction, emoji) {
                println!("{:?}, {}", msg, count);
                if count == 1 {
                    msg.pin(ctx).unwrap_or(());
                }
            }
        }
    }

    fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if let ReactionType::Unicode(emoji) = &reaction.emoji {
            if let Some((msg, count)) = emoji_count(&ctx, &reaction, emoji) {
                if count == 1 {
                    msg.unpin(ctx).unwrap_or(());
                }
            }
        }
    }
}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
    .expect("Error creating client");
    client.with_framework(StandardFramework::new()
    .configure(|c| c.prefix(&env::var("DISCORD_PREFIX").unwrap_or("~".to_string())))
    .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        println!("Error has occured: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
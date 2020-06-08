use serenity::{
    prelude::*,
    model::channel::{Reaction, ReactionType, Message}
};

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

pub struct Handler;

impl EventHandler for Handler {
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let ReactionType::Unicode(emoji) = &reaction.emoji {
            if let Some((msg, count)) = emoji_count(&ctx, &reaction, emoji) {
                if count == 2 {
                    println!("Pinning!");
                    if let Err(why) = msg.pin(ctx) {
                        println!("{:?}", why);
                    }
                }
            }
        }
    }

    fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if let ReactionType::Unicode(emoji) = &reaction.emoji {
            if let Some((msg, count)) = emoji_count(&ctx, &reaction, emoji) {
                if count == 1 {
                    println!("Unpinning!");
                    if let Err(why) = msg.unpin(ctx) {
                        println!("{:?}", why);
                    }
                }
            }
        }
    }
}
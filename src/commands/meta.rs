use keys;

use lavalink::stats::RemoteStats;
use serenity::client::Context;
use serenity::framework::standard::Args;
use serenity::model::*;

pub fn ping(ctx: &mut Context, msg: &Message, _: Args) -> Result<(), String> {
    let _ = if let Some(latency) = ctx.shard.lock().latency() {
        msg.channel_id.say(
            format!("Pong! Shard gateway heartbeat latency: {}.{}s",
                latency.as_secs(), latency.subsec_nanos()))
    } else {
        msg.channel_id.say("Pong! No latency recorded!")
    };

    Ok(())
}

fn get_socket_stats(ctx: &mut Context) -> Result<RemoteStats, &'static str> {
    // note when more stats functions are added this should be passed between them
    // instead of obtaining a data lock for each function
    let data = ctx.data.lock();

    let socket_state = match data.get::<keys::LavalinkSocketState>() {
        Some(socket_state) => socket_state,
        None => return Err("keys::LavalinkSocketState is not present in Context::data"),
    };
    
    let socket_state = socket_state.lock()
        .expect("could not get lock on socket state");

    match socket_state.stats.clone() {
        Some(stats) => Ok(stats),
        None => {
            Err("no socket stats are available yet")
        }
    }
}

pub fn stats(ctx: &mut Context, msg: &Message, _: Args) -> Result<(), String> {
    let socket_stats = get_socket_stats(ctx); // as well as looking cleaner this should reduce the scope of the locks

    let _ = msg.channel_id.say(
        &format!("lavalink node:```\n{:?}\n```", socket_stats)
    );

    Ok(())
}
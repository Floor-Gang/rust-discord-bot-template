use crate::bot::utils::reply;
use crate::services::Config;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use crate::services::ConnectionPool;

#[group()]
#[commands(ping, db_test, prefix)]
pub struct Commands;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    reply(&ctx, &msg, &String::from("Pong!")).await;
    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    if let Err(why) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|embed| {
                embed.title("Prefix");
                embed.description(format!("My prefix is: `{}`", &config.prefix));
                embed.color(0xffa500)
            });
            m
        })
        .await
    {
        println!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why
        );
    };

    Ok(())
}

#[command]
async fn db_test(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let rows = sqlx::query!("SELECT test FROM test")
        .fetch(pool)
        .await
        .unwrap();

    reply(&ctx, &msg, &format!("{}", rows[0].test)).await; // This won't compile as the table is not existent
    Ok(())
}

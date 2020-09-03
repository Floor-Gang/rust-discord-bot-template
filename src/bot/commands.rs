use serenity::{
    prelude::*,
    framework::standard::{
        CommandResult,
        macros::{
            command, group
        }
    },
    model::{
        channel::Message
    }
};
use crate::bot::utils::{reply};
use crate::bot::DataBase;

#[group()]
#[commands(ping, db_test)]
pub struct Commands;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let test = "fortnite";
    println!("{}", test);

    reply(&ctx, &msg, &String::from("Pong!")).await;
    Ok(())
}

#[command]
async fn db_test(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let rows = db.query("SELECT test FROM test", &[]).await.unwrap();

    reply(&ctx, &msg, &rows[0].get(0)).await;
    Ok(())
}
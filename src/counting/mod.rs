pub mod commands;

use crate::models;
use crate::AppData;
use mongodb::bson::doc;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_message(ctx: &Context, message: Message) {
    if !message.author.bot {
        if let Some(channel) = counting_channel(ctx, message.channel_id).await {
            if !message.content.starts_with("*") {
                if message.author.id.to_string()
                    == channel.last_count_user.unwrap_or("".to_string())
                {
                    message
                        .react(&ctx.http, ReactionType::Unicode("👎".to_string()))
                        .await;
                    message.channel_id.send_message(&ctx.http, |msg| {
                        msg.reference_message(&message)
                            .allowed_mentions(|mentions| {
                                mentions.replied_user(true)
                            })
                            .content(format!("*Whoops!!! Wait until another person can go, {}*\n**Restarting at 1**", message.author.mention()))
                    }).await;
                    restart_channel(ctx, message.channel_id).await;
                } else {
                    if message.content == (channel.current_number + 1).to_string() {
                        message
                            .react(&ctx.http, ReactionType::Unicode("✅".to_string()))
                            .await;
                        next_number(ctx, &message).await;
                    } else {
                        message
                            .react(&ctx.http, ReactionType::Unicode("👎".to_string()))
                            .await;
                        message
                            .channel_id
                            .send_message(&ctx.http, |msg| {
                                msg.reference_message(&message)
                                    .allowed_mentions(|mentions| mentions.replied_user(true))
                                    .content(format!(
                                        "*Whoops!!! Wrong number, {}*\nRestarting at **1**",
                                        message.author.mention()
                                    ))
                            })
                            .await;
                        restart_channel(ctx, message.channel_id).await;
                    }
                }
            }
        }
    }
}

async fn restart_channel(ctx: &Context, channel_id: ChannelId) {
    let mut data_lock = ctx.data.write().await;
    let app_data = data_lock
        .get_mut::<AppData>()
        .expect("AppData not provided");
    let db = &mut app_data.database;

    db.collection::<models::Channel>("channels")
        .update_one(
            doc! {
                "id": channel_id.to_string()
            },
            doc! {
                "$set": {
                    "current_number": 0,
                },
                "$unset": {
                    "last_count_user": "",
                    "last_count_time": "",
                    "last_count_message_id": "",
                }
            },
            None,
        )
        .await
        .expect("Error finding channel");
}

async fn next_number(ctx: &Context, message: &Message) {
    let mut data_lock = ctx.data.write().await;
    let app_data = data_lock
        .get_mut::<AppData>()
        .expect("AppData not provided");
    let db = &mut app_data.database;

    db.collection::<models::Channel>("channels")
        .update_one(
            doc! {
                "id": message.channel_id.to_string()
            },
            doc! {
                "$inc": {
                    "current_number": 1,
                },
                "$set": {
                    "last_count_user": message.author.id.to_string(),
                    "last_count_message_id": message.id.to_string(),
                    "last_count_time": message.timestamp.timestamp(),
                }
            },
            None,
        )
        .await
        .expect("Error finding channel");
}

async fn counting_channel(ctx: &Context, channel_id: ChannelId) -> Option<models::Channel> {
    let mut data_lock = ctx.data.write().await;
    let app_data = data_lock
        .get_mut::<AppData>()
        .expect("AppData not provided");
    let db = &mut app_data.database;

    db.collection::<models::Channel>("channels")
        .find_one(
            doc! {
                "id": channel_id.to_string()
            },
            None,
        )
        .await
        .expect("Error finding channel")
}

async fn setup_channel(ctx: &Context, channel: &PartialChannel) -> bool {
    let mut data_lock = ctx.data.write().await;
    let app_data = data_lock
        .get_mut::<AppData>()
        .expect("AppData not provided");
    let db = &mut app_data.database;

    let channels = db.collection::<models::Channel>("channels");
    if let Some(_) = channels
        .find_one(
            doc! {
                "id": channel.id.to_string()
            },
            None,
        )
        .await
        .expect("Error finding channel")
    {
        return false;
    } else {
        channels
            .insert_one(
                models::Channel {
                    id: channel.id.to_string(),
                    current_number: 0,
                    last_count_user: None,
                    last_count_time: None,
                    last_count_message_id: None,
                },
                None,
            )
            .await
            .unwrap();
        return true;
    }
}

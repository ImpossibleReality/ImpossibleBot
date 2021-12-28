pub mod commands;

use std::fmt::{Display, Formatter};
use crate::models;
use crate::AppData;
use mongodb::bson::doc;
use serenity::model::prelude::*;
use serenity::prelude::*;

struct EmojiRef {
    animated: bool,
    id: EmojiId,
    name: &'static str
}

impl Display for EmojiRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}:{}:{}>", if self.animated { "a" } else { "" }, self.name, self.id)
    }
}

impl From<&EmojiRef> for ReactionType {
    fn from(emoji: &EmojiRef) -> Self {
        ReactionType::Custom {
            animated: emoji.animated,
            id: emoji.id,
            name: Some(String::from(emoji.name))
        }
    }
}

impl From<&EmojiRef> for EmojiIdentifier {
    fn from(emoji: &EmojiRef) -> EmojiIdentifier {
        EmojiIdentifier {
            animated: emoji.animated,
            id: emoji.id,
            name: String::from(emoji.name)
        }
    }
}

static X_EMOJI: EmojiRef = EmojiRef {
    animated: false,
    id: EmojiId(919665575813333013),
    name: "white_x_mark"
};

pub async fn handle_message(ctx: &Context, message: Message) {
    if !message.author.bot {
        if let Some(channel) = counting_channel(ctx, message.channel_id).await {
            if !message.content.starts_with("*") {
                if message.author.id.to_string()
                    == channel.last_count_user.unwrap_or("".to_string())
                {
                    message
                        .react(&ctx.http, &X_EMOJI)
                        .await.unwrap();
                    message.channel_id.send_message(&ctx.http, |msg| {
                        msg.reference_message(&message)
                            .allowed_mentions(|mentions| {
                                mentions.replied_user(true)
                            })
                            .content(format!("*Whoops!!! Wait until another person can go, {}*", message.author.mention()))
                    }).await;
                    message.channel_id.send_message(&ctx.http, |msg| {
                        msg.content("**Restarting at `1`**")
                    }).await;
                    restart_channel(ctx, message.channel_id).await;
                } else {
                    if message.content == (channel.current_number + 1).to_string() {
                        message
                            .react(&ctx.http, '✅')
                            .await;
                        next_number(ctx, &message).await;
                    } else {
                        message
                            .react(&ctx.http, &X_EMOJI)
                            .await.unwrap();
                        message
                            .channel_id
                            .send_message(&ctx.http, |msg| {
                                msg.reference_message(&message)
                                    .allowed_mentions(|mentions| mentions.replied_user(true))
                                    .content(format!(
                                        "*Whoops!!! Wrong number, {}*\n**Restarting at `1`**",
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

async fn remove_channel(ctx: &Context, channel: &PartialChannel) -> bool {
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
        channels
            .delete_one(
                doc! {
                    "id": channel.id.to_string(),
                },
                None,
            )
            .await
            .unwrap();
        return true;
    } else {
        return false;
    }
}

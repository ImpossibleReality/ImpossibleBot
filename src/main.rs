mod commands;
mod counting;
mod database;
mod logging;
pub mod models;

use mongodb::Database;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
    prelude::*,
};
use std::collections::HashMap;

use dotenv::dotenv;
use log::info;
use serenity::client::bridge::gateway::event::ShardStageUpdateEvent;
use serenity::model::channel::{
    Channel, ChannelCategory, GuildChannel, PartialGuildChannel, Reaction, StageInstance,
};
use serenity::model::event::{
    ChannelPinsUpdateEvent, GuildMemberUpdateEvent, GuildMembersChunkEvent, InviteCreateEvent,
    InviteDeleteEvent, MessageUpdateEvent, PresenceUpdateEvent, ResumedEvent, ThreadListSyncEvent,
    ThreadMembersUpdateEvent, TypingStartEvent, VoiceServerUpdateEvent,
};
use serenity::model::gateway::Presence;
use serenity::model::guild::{
    Emoji, Guild, GuildUnavailable, Integration, Member, PartialGuild, Role, ThreadMember,
};
use serenity::model::id::{ApplicationId, ChannelId, EmojiId, IntegrationId, MessageId, RoleId};
use serenity::model::prelude::{CurrentUser, User, VoiceState};
use std::env;

pub struct AppData {
    database: Database,

}

impl TypeMapKey for AppData {
    type Value = Self;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!(
            "Connected to discord as {}#{}",
            ready.user.name, ready.user.discriminator
        );

        if let Ok(guild_id) = env::var("DEV_GUILD_ID") {
            let guild = GuildId(guild_id.parse().expect("DEV_GUILD_ID must be an integer"));
            let commands = GuildId::set_application_commands(&guild, &ctx.http, |commands| {
                commands::configure_commands(commands)
            })
            .await
            .unwrap();
        } else {
            let commands =
                ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
                    commands::configure_commands(commands)
                })
                .await
                .unwrap();
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            commands::handle_command_interaction(command, &ctx).await
        } else if let Interaction::MessageComponent(component) = interaction {
            commands::handle_component_interaction(component, &ctx).await
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        counting::handle_message(&ctx, msg).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging::logger_init();
    info!("Connecting to database...");
    let db = database::connect().await;
    info!("Connected to database");

    info!("Logging into Discord...");
    // Login with a bot token from the environment
    let token = env::var("BOT_TOKEN").expect("please provide BOT_TOKEN in your environment");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(
            env::var("APPLICATION_ID")
                .expect("Please provide APPLICATION_ID in your environment")
                .parse()
                .expect("Make sure APPLICATION_ID is a number :)"),
        )
        .await
        .expect("Error creating client");

    {
        let app_data = AppData { database: db };
        let mut data_lock = client.data.write().await;
        data_lock.insert::<AppData>(app_data);
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

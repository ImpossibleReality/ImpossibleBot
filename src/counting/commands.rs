use super::{ setup_channel, remove_channel, X_EMOJI };
use serenity::builder::CreateApplicationCommands;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    ApplicationCommandOptionType,
};
use serenity::model::prelude::*;
use serenity::model::prelude::application_command::ApplicationCommandInteractionDataOption;
use serenity::prelude::*;

pub fn configure(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|command| {
        command
            .name("counting")
            .description("Counting fun!!!")
            .create_option(|setup_command| {
                setup_command
                    .kind(ApplicationCommandOptionType::SubCommand)
                    .name("setup")
                    .description("Setup bot in a channel")
                    .create_sub_option(|option| {
                        option
                            .kind(ApplicationCommandOptionType::Channel)
                            .name("channel")
                            .channel_types(&[ChannelType::Text])
                            .required(true)
                            .description("Channel you want to setup counting in")
                    })
                    .create_sub_option(|option| {
                        option
                            .kind(ApplicationCommandOptionType::Boolean)
                            .name("quiet")
                            .required(false)
                            .description("Should the bot not send a message in the channel with the rules?")
                    })
            })
            .create_option(|remove_command| {
                remove_command
                    .kind(ApplicationCommandOptionType::SubCommand)
                    .name("remove")
                    .description("Remove bot from channel")
                    .create_sub_option(|option| {
                        option
                            .kind(ApplicationCommandOptionType::Channel)
                            .name("channel")
                            .channel_types(&[ChannelType::Text])
                            .required(true)
                            .description("Channel you want to remove bot from")
                    })
            })
    })
}

pub async fn handle(interaction: ApplicationCommandInteraction, ctx: &Context) {
    let subcommand = interaction
        .data
        .options
        .get(0)
        .expect("First option expected");

    if let Some(member) = &interaction.member {
        if let Some(perms) = &member.permissions {
            if !(perms.administrator() || perms.manage_channels()) {
                interaction
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|data| {
                                data.content(format!(
                                    "<:white_x_mark:{}> You are not an administrator!",
                                    X_EMOJI
                                ))
                                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                            })
                    })
                    .await;
                return
            }
        }
    }

    if subcommand.name == "setup".to_string() {
        setup_command(&interaction, ctx, subcommand).await;
    } else if subcommand.name == "remove".to_string() {
        remove_command(&interaction, ctx, subcommand).await
    }
}

async fn setup_command(interaction: &ApplicationCommandInteraction, ctx: &Context, subcommand: &ApplicationCommandInteractionDataOption) {
    if let ApplicationCommandInteractionDataOptionValue::Channel(channel) =
    subcommand.options.get(0).unwrap().resolved.clone().unwrap()
    {
        if setup_channel(ctx, &channel).await {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.content(format!(
                                "✅ Successfully setup channel {} for counting!",
                                channel.id.mention()
                            ))
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        })
                })
                .await;
        } else {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.content(format!(
                                "Channel {} seems to already be setup.",
                                channel.id.mention()
                            ))
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        })
                })
                .await;
        }
    }
}

async fn remove_command(interaction: &ApplicationCommandInteraction, ctx: &Context, subcommand: &ApplicationCommandInteractionDataOption) {
    if let ApplicationCommandInteractionDataOptionValue::Channel(channel) =
    subcommand.options.get(0).unwrap().resolved.clone().unwrap()
    {
        if remove_channel(ctx, &channel).await {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.content(format!(
                                "✅ Successfully removed channel {} for counting!",
                                channel.id.mention()
                            ))
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        })
                })
                .await;
        } else {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.content(format!(
                                "Channel {} seems to not have been setup in the first place!",
                                channel.id.mention()
                            ))
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        })
                })
                .await;
        }
    }
}
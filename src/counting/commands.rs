use super::setup_channel;
use serenity::builder::CreateApplicationCommands;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    ApplicationCommandOptionType,
};
use serenity::model::prelude::*;
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
            })
    })
}

pub async fn handle(interaction: ApplicationCommandInteraction, ctx: &Context) {
    dbg!(&interaction.data.options);
    let subcommand = interaction
        .data
        .options
        .get(0)
        .expect("First option expected");

    if subcommand.name == "setup".to_string() {
        if let ApplicationCommandInteractionDataOptionValue::Channel(channel) =
            subcommand.options.get(0).unwrap().resolved.clone().unwrap()
        {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                })
                .await;
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
}

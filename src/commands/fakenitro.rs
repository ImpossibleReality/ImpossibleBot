use serenity::builder::CreateApplicationCommands;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    ApplicationCommandOptionType,
};
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::prelude::application_command::ApplicationCommandInteractionDataOption;
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

pub fn configure(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|cmd| cmd.name("fakenitro").description("Get trolled!"))
}

pub async fn handle(interaction: ApplicationCommandInteraction, ctx: &Context) {
    let channel = interaction.channel_id;

    interaction.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    .content("Message sent.")
            })
    }).await;

    channel
        .send_message(&ctx.http, |msg| {
            msg.add_embed(|embed| {
                embed
                    .title("A WILD GIFT APPEARS!")
                    .thumbnail("https://i.imgur.com/w9aiD6F.png")
                    .field("Discord Nitro", "Expires in 48 hours.", false)
                    .color(Color::from_rgb(47, 49, 54))
                    .footer(|footer| {
                        footer.text(&interaction.user.name).icon_url(
                            interaction.user.avatar_url().unwrap_or(
                                "https://cdn.discordapp.com/embed/avatars/1.png".to_string(),
                            ),
                        )
                    })
            })
            .components(|comp| {
                comp.create_action_row(|row| {
                    row.create_button(|btn| {
                        btn.label("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀Accept⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀")
                            .custom_id("fakenitro-accept")
                            .style(ButtonStyle::Success)
                    })
                })
            })
        })
        .await;
}

pub async fn handle_accept(interaction: MessageComponentInteraction, ctx: &Context) {
    interaction
        .create_interaction_response(&ctx.http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| {
                    data.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        .content("HAHAHA get trolled")
                })
        })
        .await;
}

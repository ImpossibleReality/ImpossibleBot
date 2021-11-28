use crate::counting::commands as counting;

use log::warn;
use serenity::builder::CreateApplicationCommands;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::message_component::MessageComponentInteraction;
use serenity::prelude::*;

trait Configurable {
    fn configure<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(&mut Self) -> &mut Self;
}

impl Configurable for CreateApplicationCommands {
    fn configure<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(&mut Self) -> &mut Self,
    {
        func(self)
    }
}

pub fn configure_commands(
    commands: &mut CreateApplicationCommands,
) -> &mut CreateApplicationCommands {
    commands.configure(counting::configure)
}

pub async fn handle_command_interaction(interaction: ApplicationCommandInteraction, ctx: &Context) {
    match interaction.data.name.as_str() {
        "counting" => counting::handle(interaction, ctx).await,
        _ => {
            warn!("Unregistered Command: {}", interaction.data.name.as_str())
        }
    }
}

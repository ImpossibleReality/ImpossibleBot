import discord
from discord.ui import InputText, Modal
from discord.ext import commands
from gpt import generate_text

bot = discord.Bot()


class GptModal(Modal):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)

        self.add_item(
            InputText(
                label="Input Text",
                placeholder="The AI will try to add to this text while maintaining context.",
                style=discord.InputTextStyle.long,
                max_length=1000,
                min_length=5,
            )
        )

    async def callback(self, interaction: discord.Interaction):
        embed = discord.Embed(title="Generated Text", color=discord.Color.random())
        embed.add_field(name="Input", value=self.children[0].value, inline=False)
        embed.add_field(name="Output", value="Loading...", inline=False)
        await interaction.response.send_message(embeds=[embed])
        generated = await generate_text(self.children[0].value)
        print(generated)
        embed = discord.Embed(title="Generated Text", color=discord.Color.random())
        embed.add_field(name="Input", value=self.children[0].value, inline=False)
        embed.add_field(name="Output", value=generated, inline=False)
        await interaction.edit_original_message(embeds=[embed])


@bot.event
async def on_ready():
    print("Bot ready")


@bot.slash_command(guild_ids=["782428786229903380"])  # create a slash command for the supplied guilds
@commands.cooldown(2, 60, commands.BucketType.user)
async def gpt(ctx):
    """Generate text using GPT"""
    modal = GptModal(title="GPT Text Generation")
    await ctx.interaction.response.send_modal(modal)


bot.run("TOKEN")

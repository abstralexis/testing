# I think I am finding out why people say discordpy is badly
# documented now. Ugh.

import discord

intents = discord.Intents.default()
intents.message_content = True
intents.messages = True
client = discord.Client(intents=intents)

@client.event
async def on_ready():
    print(f'We have logged in as {client.user}')

@client.event
async def on_message(message):
    """
    Interact with Discord's API such that when a message is sent
    it ignores it if the author of the message is itself with a guard
    clause, then checks if the content of the message has a command.
    If the message starts with a command, the bot sends a message. 
    """
    if message.author == client.user:
        return

    if message.content.startswith('$latex'):
        query = message.content()

# Run bot with discord login token
client.run('your token here')
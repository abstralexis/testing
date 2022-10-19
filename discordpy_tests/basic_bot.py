# This is a basic example from discordpy's website, I plan
# on changing up some things to see what it does, and adding comments.
# I think I should be able to do this project fully because
# As much as people say the documentation sucks for
# discordpy, I think it is alright. Just not too technical though.

# This example requires the 'message_content' intent.

import discord  # Import the discord library

# Initialise client with intents (like permissions but not quite)
intents = discord.Intents.default() # Get defaults
intents.message_content = True      # Declare intent for message content
client = discord.Client(intents=intents)    # Init bot client

# Set the client's event listeners to do certain actions asynchronously
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

    if message.content.startswith('$hello'):
        await message.channel.send('Hello!')

# Run bot with discord login token
client.run('your token here')
import telebot
import requests
import os
from bs4 import BeautifulSoup
from secrets import token_hex

owner = int(os.environ["OWNER"])
riseup_login = os.environ["RISEUP_LOGIN"]
riseup_password = os.environ["RISEUP_PASSWORD"]
bot = telebot.TeleBot(os.environ["TOKEN"], parse_mode="html")


@bot.message_handler(commands=["start"])
def start(message):
    if message.chat.id != owner:
        bot.send_message(message.chat.id, "You are not allowed to use this bot")
        return
    bot.send_message(message.chat.id, "Hello, send me /alias to get a new alias")


@bot.message_handler(commands=["alias", "a"])
def alias(message):
    if message.chat.id != owner:
        bot.send_message(message.chat.id, "You are not allowed to use this bot")
        return

    m = bot.send_message(message.chat.id, "Generating alias...")

    # Create cookie session
    s = requests.Session()

    r = s.get("https://account.riseup.net/")
    soup = BeautifulSoup(r.text, "html.parser")
    # Find csrf token
    # <input type="hidden" name="authenticity_token" value="...">
    csrf_token = soup.find("input", {"name": "authenticity_token"})["value"]

    # Send login form
    r = s.post("https://account.riseup.net/session", data={
        "authenticity_token": csrf_token,
        "utf8": "✓",
        "username": riseup_login,
        "password": riseup_password,
        "button": "",
    },
    allow_redirects=False)

    # Get alias
    # (first argument or random string)
    alias = token_hex(3) + "-" + (message.text.split(" ", 1)[1]) if len(message.text.split(" ", 1)) > 1 else token_hex(6)

    r = s.get("https://account.riseup.net/mail/aliases")
    soup = BeautifulSoup(r.text, "html.parser")
    # Find alias csrf token
    # <input type="hidden" name="authenticity_token" value="...">
    csrf_token = soup.find("input", {"name": "authenticity_token"})["value"]

    # Send alias form
    r = s.post("https://account.riseup.net/mail/aliases", data={
        "authenticity_token": csrf_token,
        "utf8": "✓",
        "mailbox_alias[source]": f"{alias}@riseup.net",
        "button": "",
    },
    allow_redirects=False)

    # Update message
    bot.edit_message_text(f"Alias: <code>{alias}@riseup.net</code>", message.chat.id, m.message_id)


# Run bot
bot.polling()

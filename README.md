# Riseup Alias Generator

Riseup Alias Generator is a service that allows you to generate aliases in the Riseup.net email service through a Telegram bot. To use the service, you must self-host the service and run it through a Docker container.

## Prerequisites

Before running the Riseup Alias Generator, make sure you have the following:

- A self-hosted environment
- Docker installed
- Telegram account and API token
- A Riseup email account

## Getting Started

To start using the Riseup Alias Generator, follow these steps:

1. Clone the repository to your local machine:

  ```bash
  git clone <https://github.com/cofob/riseup-alias-generator.git>
  ```

2. Navigate to the cloned repository and create a .env file with the following environment variables:

  ```makefile
  OWNER=<your_telegram_id>
  TOKEN=<your_telegram_api_token>
  RISEUP_LOGIN=<your_riseup_email_address>
  RISEUP_PASSWORD=<your_riseup_email_password>
  ```

  Replace <your_telegram_id> with your Telegram ID, <your_telegram_api_token> with your Telegram API token, <your_riseup_email_address> with your Riseup email address (or username), and <your_riseup_email_password> with your Riseup email password.

3. Run the Docker container:

  ```bash
  docker run --restart always --name riseup-alias-generator --env-file .env ghcr.io/cofob/riseup-alias-generator
  ```

  Start a conversation with the Telegram bot and send the command /alias to generate a new alias.

## Usage

Once you have the Riseup Alias Generator up and running, you can use the Telegram bot to generate new aliases.

To generate a new alias, follow these steps:

1. Start a conversation with the Telegram bot.
2. Send the command /alias.
3. The bot will respond with a new alias, like `abcdefg@riseup.net`.

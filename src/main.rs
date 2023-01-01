use std::str::FromStr;

use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
	if std::env::var("RUST_LOG").is_err() {
		std::env::set_var("RUST_LOG", "info");
	}
	pretty_env_logger::init();
	log::info!("Starting riseup alias bot...");

	let bot = Bot::from_env();

	Command::repl(bot, answer).await;
}

#[derive(Clone)]
struct OptionalString(Option<String>);

impl FromStr for OptionalString {
	type Err = std::io::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.is_empty() {
			Ok(Self(None))
		} else {
			Ok(Self(Some(s.to_string())))
		}
	}
}

#[derive(BotCommands, Clone)]
#[command(
	rename_rule = "lowercase",
	description = "These commands are supported:"
)]
enum Command {
	#[command(description = "send start message.")]
	Start,
	#[command(description = "display this text.")]
	Help,
	#[command(description = "add alias")]
	Alias(OptionalString),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
	let admin_id: u64 = std::env::var("ADMIN_ID")
		.expect("ADMIN_ID must be set")
		.parse()
		.expect("ADMIN_ID must be a number");
	let admin_id = UserId(admin_id);
	match cmd {
		Command::Start => {
			bot.send_message(
				msg.chat.id,
				format!("Hello! This bot is bound to {}.", admin_id),
			)
			.await?;
		}
		Command::Help => {
			bot.send_message(msg.chat.id, Command::descriptions().to_string())
				.await?;
		}
		Command::Alias(alias) => {
			if msg.from().unwrap().id == admin_id {
				let alias = alias.0;
				let login_page = reqwest::get("https://account.riseup.net/")
					.await
					.expect("Failed to get login page")
					.text()
					.await
					.expect("Failed to get login page text");
				log::info!("login_page: {}", login_page);
				let parsed_login_page: xmltree::Element =
					xmltree::Element::parse(login_page.as_bytes())
						.expect("Failed to parse login page");
				// Find csrf token
				// <input type="hidden" name="authenticity_token" value="...">
				let csrf_token = parsed_login_page
					.children
					.iter()
					.find(|child| {
						child.as_element().unwrap().name == "input"
							&& child
								.as_element()
								.unwrap()
								.attributes
								.get("name")
								.unwrap()
								.as_str() == "authenticity_token"
					})
					.unwrap()
					.as_element()
					.unwrap()
					.attributes
					.get("value")
					.unwrap()
					.as_str();
				log::info!("csrf_token: {}", csrf_token);
			} else {
				bot.send_message(
					msg.chat.id,
					"You are not authorized to use this command".to_string(),
				)
				.await?;
			}
		}
	};

	Ok(())
}

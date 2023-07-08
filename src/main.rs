// tg-send - Telegram message sender
// Copyright © luxagen, 2023-present

#![allow(unreachable_code)]

use std::env;

use tokio;
use telegram_bot::*;
use std::io::Read;

fn slurp_stdin() -> String
{
	let mut buf = String::new();

	std::io::BufReader::new(std::io::stdin()).read_to_string(&mut buf).unwrap();

	return buf;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
	let channel_id = env::var("TELEGRAM_CHANNEL_ID").expect("TELEGRAM_CHANNEL_ID not set").parse::<i64>().unwrap();

	let chat = ChannelId::new(channel_id);
	let text = std::borrow::Cow::Owned(slurp_stdin());
	Api::new(token).send(chat.text(text)).await.unwrap();

	Ok(())
}

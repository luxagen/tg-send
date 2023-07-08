#![allow(unreachable_code)]

use std::env;

extern crate tokio;
use futures::StreamExt;
use telegram_bot::*;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
	let id = env::var("TELEGRAM_CHANNEL_ID").expect("TELEGRAM_CHANNEL_ID not set");

	let api = Api::new(token);

	let chat = ChannelId::new(id.parse::<i64>().unwrap());

	let mut buf = String::new();
	let s = std::io::BufReader::new(std::io::stdin()).read_to_string(&mut buf);
	let text = std::borrow::Cow::Owned(buf);
	let t = api.send(chat.text(text));

	t.await.unwrap();

	Ok(())
}
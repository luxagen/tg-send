// tg-send - Telegram message sender
// Copyright © luxagen, 2023-present

#![allow(unreachable_code)]

use std::env;

use tokio;
use telegram_bot::*;
use std::io::Read;

#[derive(clap::Subcommand)]
enum Commands
{
	SendText { chatid: i64 },
}

#[derive(clap::Parser)]
#[clap(author,about,long_about=None)]
#[command(arg_required_else_help = true)]
struct Args
{
	#[arg(long,short,help="Commands will be performed via this Telegram bot token")]
	token: Option<String>,

	#[arg(long,short,help="Wait this number of milliseconds for a response from Telegram, otherwise wait indefinitely")]
	wait: Option<u64>,

	#[command(subcommand,help="Send a text message via stdin")]
	command: Commands,
}

const EVK_TBT: &str = "TELEGRAM_BOT_TOKEN";

impl Args
{
	fn parse() -> Self
	{
		let mut s = <Self as clap::Parser>::parse();

		let mut errors = Vec::<String>::new();

		use colored::*;

		if s.token.is_none()
		{
			match env::var(EVK_TBT)
			{
				Err(_) => errors.push(format!("No bot token provided [{}] and {} is not set","-t".yellow(),EVK_TBT.green())),
				Ok(t) => s.token = Some(t), // TODO move this code to main() ?
			}
		}

		if !errors.is_empty()
		{
			for e in errors
			{
				eprintln!("{} {e}","error:".bright_red().bold());
			}

			std::process::exit(2);
		}

		s
	}
}

lazy_static::lazy_static!
{
	static ref ARGS: Args = Args::parse();
}

fn slurp_stdin() -> String
{
	let mut buf = String::new();

	match std::io::BufReader::new(std::io::stdin()).read_to_string(&mut buf)
	{
		Err(_) => panic!(),
		Ok(_) => (),
	}

	buf.truncate(4096);
	buf
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	let api = Api::new(ARGS.token.as_ref().unwrap());

	match ARGS.command
	{
		Commands::SendText { chatid } =>
		{
			let chat = ChannelId::new(chatid);
			let text = std::borrow::Cow::Owned(slurp_stdin());

			let wait = match ARGS.wait
			{
				None => core::time::Duration::from_secs(20),
				Some(x) => core::time::Duration::from_millis(x),
			};

			let future = api.send_timeout(chat.text(text),wait);

			match future.await
			{
				Err(e) => eprintln!("error: {e}"),
				_ => std::process::exit(0),
			}
		},
	}

	Ok(())
}

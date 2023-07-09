#![allow(unreachable_code)]

use std::env;

extern crate tokio;
//use futures::StreamExt;
use telegram_bot::*;
use std::io::Read;

//lazy_static::lazy_static!
//{
//	static ref args: Args = Args::parse();
//}

#[derive(Subcommand)]
enum Commands
{
	Send { message: Option<String> },
}

#[derive(clap::Parser)]
#[clap(author,about,long_about=None)]
#[command(arg_required_else_help = true)]
struct Args
{
	#[command(subcommand)]
	command: Option<Commands>,

//	#[arg(long,short='x',help="Clear metadata from extended attributes")]
//	clear: bool,
//	#[arg(long,short,help="Suppress stdout logging")]
//	quiet: bool,
//	#[arg(long,short='Q',help="Force stdout logging (ignore .rk.quiet files)")]
//	unquiet: bool,
//	#[arg(long,short,help="Log files to stdout in `md5deep -zl` format")]
//	export: bool,
//	#[arg(long,short,help="Print status statistics to stderr")]
//	count: bool,
//	#[arg(long,short,help="Print paths relative to each argument")]
//	relative: bool,
//	#[arg(long,short,help="Initialise files that are missing metadata")]
//	init: bool,
//	#[arg(long,short,help="Update metadata for changed files")]
//	update: bool,
//	#[arg(long,short,help="Verify initialised files against metadata")]
//	verify: bool,
//	#[arg(long="verify-ro",short='V',help="As --verify, but don\'t update metadata")]
//	readonly: bool,
//
//	#[arg(required=false,help="Filesystem trees on which to operate")]
//	tree: Vec<String>,
}

fn SlurpStdin() -> String
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
	let text = std::borrow::Cow::Owned(SlurpStdin());
	Api::new(token).send(chat.text(text)).await.unwrap();

	Ok(())
}

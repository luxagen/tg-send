# tg-send: a small Rust program for sending Telegram messages

Send messages to a group/channel via the Bot API from the command line; it's super simple to create a bot token via Botfather and get going. It was developed for easy monitoring of server automation, and Telegram channels are a great way to keep on top of regular jobs without having to resort to "no news is good news" &mdash; a very bad idea because you can confuse "didn't run" with "everything worked".

In fact, as a general way to pipe text into a Telegram message, this is also useful for non-automated jobs. Imagine your server or workstation notifying you remotely when a long-running job finishes!
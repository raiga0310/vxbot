use regex::Regex;
use serenity::{
    async_trait,
    framework::{
        standard::{help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions},
        StandardFramework,
    },
    model::prelude::{Message, UserId},
    prelude::{Context, EventHandler, GatewayIntents},
    Client, Result,
};
use std::{collections::HashSet, env};

#[tokio::main]
async fn main() {
    let token = load_token();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("vx!"))
        .help(&HELP);
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[help]
#[individual_command_tip = "vxbotのヘルプです"]
#[strikethrough_commands_tip_in_guild = ""]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let Some((username, hash)) = match_url(&msg.content) else {
            return;
        };
        let reply = format!("https://vxtwitter.com/{}/status/{}\n", username, hash);
        check_msg(msg.reply(&_ctx.http, reply).await);
    }
}

fn load_token() -> String {
    //load from .secret
    dotenv::from_filename(".secret").ok();
    env::var("TOKEN").expect("Expected a token in the environment")
}

fn check_msg(result: Result<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

// 正規表現マッチングを行う関数
fn match_url(content: &str) -> Option<(String, String)> {
    let regex = Regex::new(
        r"https:\/\/(x|twitter)\.com\/(?<username>[a-zA-Z0-9_]{1,16})\/status\/(?<hash>[0-9]+)",
    )
    .unwrap();

    regex
        .captures(content)
        .map(|caps| (caps["username"].to_string(), caps["hash"].to_string()))
}

// テスト関数
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_url() {
        let content = "text https://twitter.com/user123/status/12345678 text";
        let (username, hash) = match_url(content).unwrap();
        assert_eq!(username, "user123");
        assert_eq!(hash, "12345678");
    }
}

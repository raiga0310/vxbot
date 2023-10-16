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

        //if msg have "https://twitter.com", replace all "https://twitter.com" or "https://x.com" to "https://vxtwitter.com" and reply
        if !(msg.content.contains("https://twitter.com") || msg.content.contains("https://x.com")) {
            return;
        }

        let mut reply = msg
            .content
            .replace("https://twitter.com", "https://vxtwitter.com");
        reply = reply.replace("https://x.com", "https://vxtwitter.com");
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

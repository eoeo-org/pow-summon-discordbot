use std::env;

use serenity::all::ActivityData;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelType;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(
        &self,
        ctx: Context,
        old_state: Option<VoiceState>,
        new_state: VoiceState,
    ) {
        if let Some(old_state) = old_state {
            if old_state.channel_id == new_state.channel_id {
                return;
            }
        }
        let Some(channel_id) = new_state.channel_id else {
            return;
        };
        let Some(member) = new_state.member else {
            return;
        };
        if member.user.bot {
            return;
        }
        let Some(voice_channel) = ctx
            .http
            .get_channel(channel_id)
            .await
            .ok()
            .and_then(|channel| channel.guild())
            .filter(|guild_channel| guild_channel.kind == ChannelType::Voice)
        else {
            return;
        };
        if let Some(afk_metadata) = voice_channel
            .guild(&ctx.cache)
            .and_then(|guild| guild.afk_metadata.clone())
        {
            if channel_id == afk_metadata.afk_channel_id {
                return;
            }
        }
        let Ok(members) = voice_channel.members(&ctx.cache) else {
            return;
        };
        if members.iter().any(|member| member.user.bot) {
            return;
        }
        let mut non_bot_members = members;
        non_bot_members.retain(|member| !member.user.bot);
        if non_bot_members.len() != 2 {
            return;
        }
        if let Err(why) = channel_id
            .say(&ctx.http, "<@939494577574924339> join")
            .await
        {
            println!("Error sending message: {:?}", why)
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    let git_describe = option_env!("GIT_DESCRIBE").unwrap_or("");
    println!(
        "{pkg_name} - {git_describe}",
        pkg_name = env!("CARGO_PKG_NAME")
    );
    let token = env::var("DISCORD_TOKEN").expect("token");
    run(token, git_describe).await;
}

async fn run(token: String, activity: &str) {
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .activity(ActivityData::custom(activity))
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why)
    }
}

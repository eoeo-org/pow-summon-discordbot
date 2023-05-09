use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
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
        if old_state.is_some() {
            return;
        }
        let Some(channel_id) = new_state.channel_id else {
            return
        };
        if let Some(member) = new_state.member {
            if member.user.bot {
                return;
            }

            if let Ok(channel) = ctx.http.get_channel(channel_id.into()).await {
                if let Some(guild_channel) = channel.guild() {
                    if let Ok(members) = guild_channel.members(&ctx.cache).await {
                        if members.len() == 1 {
                            if let Err(why) = channel_id
                                .say(&ctx.http, "<@939494577574924339> join")
                                .await
                            {
                                println!("Error sending message: {:?}", why)
                            }
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why)
    }
}

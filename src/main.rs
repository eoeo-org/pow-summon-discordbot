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
        if let Some(old_state) = old_state {
            if old_state.channel_id == new_state.channel_id {
                return;
            }
        }
        let Some(channel_id) = new_state.channel_id else {
            return
        };
        let Some(member)= new_state.member else{return };
        if member.user.bot {
            return;
        }
        let Some(guild_channel) = ctx.http.get_channel(channel_id.into()).await.ok().and_then(|channel| channel.guild()) else {return };
        if let Some(afk_channel_id) = guild_channel
            .guild(&ctx.cache)
            .and_then(|guild| guild.afk_channel_id)
        {
            if channel_id == afk_channel_id {
                return;
            }
        }
        let Ok(members) = guild_channel.members(&ctx.cache).await else{return}  ;
        if members.len() != 1 {
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

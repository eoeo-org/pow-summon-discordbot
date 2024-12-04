use std::env;

#[tokio::main]
async fn main() {
    let git_describe = option_env!("GIT_DESCRIBE").unwrap_or("");
    println!(
        "{pkg_name} - {git_describe}",
        pkg_name = env!("CARGO_PKG_NAME")
    );
    let token = env::var("DISCORD_TOKEN").expect("token");
    pow_summon_discordbot::run(token, git_describe).await;
}

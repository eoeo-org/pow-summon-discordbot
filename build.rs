fn main() {
    match get_git_describe_result() {
        Ok(result) => println!("cargo::rustc-env=GIT_DESCRIBE={result}"),
        Err(e) => println!("cargo::warning={}", e),
    }
}

fn get_git_describe_result() -> Result<String, Box<dyn std::error::Error>> {
    let repo = gix::discover(".")?;
    let mut describe = repo
        .head_commit()?
        .describe()
        .names(gix::commit::describe::SelectRef::AllTags);
    let mut format = describe.format()?;
    if repo.is_dirty()? {
        format.dirty_suffix = Some("dirty".into());
    }
    let formatted = format.long(true).to_string();

    Ok(formatted)
}

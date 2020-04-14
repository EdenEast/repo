pub fn init() -> String {
    let completion = include_str!("completion.zsh");
    let work = include_str!("work.zsh");
    format!("{}\n{}", completion, work)
}

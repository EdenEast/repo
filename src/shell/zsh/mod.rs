pub fn init(fzf: bool) -> String {
    let completion = include_str!("completion.zsh");
    let work = if fzf {
        include_str!("fzf.zsh")
    } else {
        include_str!("work.zsh")
    };

    format!("{}\n{}", completion, work)
}

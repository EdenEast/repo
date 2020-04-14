pub fn init(fzf: bool) -> String {
    let completion = include_str!("completion.fish");
    let work = if fzf {
        include_str!("fzf.fish")
    } else {
        include_str!("work.fish")
    };

    format!("{}\n{}", completion, work)
}

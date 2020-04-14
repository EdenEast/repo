pub fn init(fzf: bool) -> String {
    let completion = include_str!("completion.bash");
    let work = if fzf {
        include_str!("fzf.bash")
    } else {
        include_str!("work.bash")
    };

    format!("{}\n{}", completion, work)
}

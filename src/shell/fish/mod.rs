pub fn init() -> String {
    let completion = include_str!("completion.fish");
    let work = include_str!("work.fish");
    format!("{}\n{}", completion, work)
}

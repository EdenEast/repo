pub fn init() -> String {
    let completion = include_str!("completion.bash");
    let work = include_str!("work.bash");
    format!("{}\n{}", completion, work)
}

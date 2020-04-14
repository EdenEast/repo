pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

pub fn init(shell: Shell, fzf: bool) -> String {
    match shell {
        Shell::Bash => bash::init(fzf),
        Shell::Fish => fish::init(fzf),
        Shell::Zsh => zsh::init(fzf),
    }
}

mod bash;
mod fish;
mod zsh;

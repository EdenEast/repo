pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

pub fn init(shell: Shell) -> String {
    match shell {
        Shell::Bash => bash::init(),
        Shell::Fish => fish::init(),
        Shell::Zsh => zsh::init(),
    }
}

mod bash;
mod fish;
mod zsh;

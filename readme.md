# Repo

[![Continuous Integration][build-image]][build-link]
[![Dependabot][dependabot-image]][dependabot-link]
![Apachie 2.0][license-image]

Repo is a command line repository management tool written in rust. Track your remote repositories
with repo and manage your workspace.

![demo](./docs/resources/demo.gif)

## Installation

Currently repo is only available from source. To this end make sure that you have a rust environment
setup. If you are new to rust and want more information on setting up a development environment
check the [rust book's][rust-setup] setup chapter.

```bash
# Install using cargo
cargo install --git https://github.com/edeneast/repo
```

This will install `repo` in your default cargo path `$CARGO_HOME/bin/repo`. Once `repo` is in your
path, the `work` helper function and completion needs to be sourced in order to let `repo` change your shell's cwd. The work command has [fzf] integration. If you have `fzf` installed then add `--fzf` option to `init`.

#### Bash

```bash
# ~/.bashrc
[ -x "$(command -v repo)" ] && eval "$(repo init bash)"

# With `fzf` integration
[ -x "$(command -v repo)" ] && {
    [ -x "$(command -v fzf)"] && eval "$(repo init bash --fzf)" || eval "$(repo init bash)"
}
```

#### Zsh

```zsh
# ~/.zshrc
[ -x "$(command -v repo)" ] && eval "$(repo init zsh)"

# With `fzf` integration
[ -x "$(command -v repo)" ] && {
    [ -x "$(command -v fzf)"] && eval "$(repo init zsh --fzf)" || eval "$(repo init zsh)"
}
```

#### Fish

```sh
# ~/.config/fish/config.conf
test -x $(command -v repo) && repo init fish | source

# With `fzf` integration
test -x $(command -v repo) && {
    test -x $(command -v fzf) && repo init fish --fzf | source || repo init fish | source
}
```

[rust-setup]: https://doc.rust-lang.org/book/ch01-01-installation.html
[fzf]: https://github.com/junegunn/fzf

## Quick Start

```sh
# Track a new remote repository with repo
repo add edeneast/repo

# Add your fork of a project
repo add edeneast/cargo --remote 'upstream,rust-lang/cargo' --path 'forks'

# Update workspace by pulling from remote
repo update

# Execute command on all repos
repo foreach 'git remote update --prune'

# Run the work command to cd into the repository folder in the workspace
work repo
```

## Development

`repo` is currently under development and subject to change before a `v1.0` release. Have an idea
for repo? Open an issue or fork the project and create a pull request.

## Licence

Repo is licenced under [Apache][apache-2.0] Licence (Version 2.0)

See [LICENSE](./LICENSE) file for more details.

[//]: # (badges)

[build-image]: https://github.com/EdenEast/repo/workflows/Continuous%20Integration/badge.svg
[build-link]: https://github.com/EdenEast/repo/actions?query=workflow%3A%22Continuous+Integration%22+branch%3Amaster
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[dependabot-image]: https://api.dependabot.com/badges/status?host=github&repo=EdenEast/repo
[dependabot-link]: https://dependabot.com

[//]: # (general)

[apache-2.0]: https://apache.org/licenses/LICENSE-2.0


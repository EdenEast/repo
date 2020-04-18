# Repo

[![Continuous Integration][build-image]][build-link]
[![Dependabot][dependabot-image]][dependabot-link]
[![Crates.io][crates-io-image]][crates-io-link]
![Apachie 2.0][license-image]

Repo is a command line repository management tool written in rust. Track your remote repositories
with repo and manage your workspace.

![demo](./docs/resources/demo.gif)

## Table of Contents

<details>
<summary>Click here to show</summary>

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [Dotfiles](#dotfiles)
- [Components](#Components)
  - [Repository](#repository)
  - [Tag](#tag)
  - [Remote](#remote)
- [Development](#development)
- [Licence](#licence)

</details>

## Installation

Install the latest released version of repo from crates.io with cargo. If you don't have a rust
environment setup check the [rust book's][rust-setup] setup chapter.

```bash
cargo install repo-cli
```

This will install `repo` in your default cargo path `$CARGO_HOME/bin/repo`. Once `repo` is in your
path, the `work` helper function and completion needs to be sourced in order to let `repo` change
your shell's cwd. The work command has [fzf] integration. If you have `fzf` installed then add
`--fzf` option to `init`.

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

# Create a tag that will repositories into a folk folder
repo tag add fork --path fork --tag fork

# Add your fork of a project
repo add edeneast/cargo --remote 'upstream,rust-lang/cargo' --path 'forks'

# Update workspace by pulling from remote
repo update

# Execute command on all repos. This gets the behind/ahead of a branch relative to its origin
repo foreach 'b=$(git rev-parse --abbrev-ref HEAD) ; git rev-list --left-right --count origin/$b...$b'

# Check if all forks can be fast-forward merged with upstream
repo foreach -t fork 'git merge-base --is-ancestor upstream/master master ; echo $?'

# Run the work command to cd into the repository folder in the workspace
work repo

# Not sure about something... Check the help
repo help
```

## Usage

## Dotfiles

Repo stores it's information in a config folder. Repo calls these locations caches. There are two
different caches repo uses, global and local.

|Location Type|Default Path|Environment Override|
|-------------|------------|--------------------|
|Global | $XDG_CONFIG_HOME/repo | $REPO_CONFIG_PATH|
|Local | $XDG_DATA_HOME/repo | $REPO_LOCAL_PATH|

Depending on your operating system the definition of [XDG_CONFIG_HOME][config-dir] and
[XDG_DATA_HOME][local-data-dir] will be different. Check the links to see the default for your os.

Just like how git allows you to have global and local configuration, repo allows the same
flexibility. This means that you can commit your global configuration into your versioned dotfiles
and save any local machine only changes. The local cache will override anything in the global cache.
This means that you can define things like the `root` workspace path in your local config and will
override the one set globally.

Using the configuration's include and exclude tag filters in your local config you can make sure
that one repo's with a tag can be operated on, or vice versa you can exclude any repos that contain
a tag. This comes in useful when you use your personal dotfiles on a work machine. Maybe you want to
use repo to manage your internal work's repo and you want to filter out your personal repos so they
are not accessible on your work machine. You can add a `personal` tag to all of your personal repos
and exclude that tag in your local work's configuration cache. This would filter out all of your
personal repos on your work machine making sure that you cant pull anything on your work machine.

#### Example

```bash
# personal computer
repo tag add personal # Add a marker tag to denote that this is a personal repo
repo add my-github/project --tag personal # Defaults to the global config cache
dotfiles add $REPO_CONFIG_PATH # Commit global config to dotfiles
dotfiles commit -m "Add global repo config cachce"

# work machine
repo add https://internal-host.com/org/work --local # Save work to local machine config cache
repo config exclude personal --local # Set repo to exclude repos with personal tag on work machine
repo list # Will only contain the work repo as our personal project is excluded
```

[config-dir]: https://docs.rs/dirs/2.0.2/dirs/fn.config_dir.html
[local-data-dir]: https://docs.rs/dirs/2.0.2/dirs/fn.data_local_dir.html

## Components

### Repository

A repository represents a project that is hosted on some remote. A repository contains a bunch of
optional values that can be set.

| Name    | Type       | Description                                                                           |
| ------  | -----      | ------------                                                                          |
| path    | Path       | The path relative to the workspace root, the repository location in the workspace     |
| clone   | String     | The command that will be executed on the after clone hook                             |
| work    | String     | The command that will be executed on the after work hook                              |
| cli     | bool       | A flag to determine if repo should execute git from command line or libgit2           |
| tags    | TagList    | The list of tag names associated with the repository                                  |
| remotes | RemoteList | The list of remotes for this repository. Note convention that origin is first in list |

#### Example config file

```toml
# ~/.config/repo/repository/repo.toml
name = 'repo'
path = 'app'
work = 'echo after work hook'
clone = 'echo after clone hook'
tags = [
  'personal',
  'rust',
]

[[remotes]]
name = 'origin'
url = 'https://github.com/edeneast/repo'

[[remotes]]
name = 'upstream'
url = 'https://github.com/upstream-fork/repo'
```

### Tag

A tag represents values that can be applied to multiple repositories. If you want to reuse
configuration values on multiple repositories then create a tag. A tag can also be just for marking
a project. For example if the project is a personal project you can create an empty tag and
associate it with all your personal projects. You can then filter out personal tags in the config
file on machines where you want want personal projects like a work machine.

| Name     | Type   |  Description                                                                      |
| ----     | ----   |  -----------                                                                      |
| path     | Path   | The path relative to the workspace root, the repository location in the workspace |
| clone    | String | The command that will be executed on the after clone hook                         |
| work     | String | The command that will be executed on the after work hook                          |
| cli      | bool   | A flag to determine if repo should execute git from command line or libgit2       |
| priority | Number | The order in which tags are applied. The lower the number the higher the priority |

#### Example

```toml
# ~/.config/repo/tag/rust.toml
name = 'rust'
path = 'rust'
clone = 'cargo fetch'
work = 'cargo check'
priority = 20
```

### Config

As discussed in the [dotfiles](#dotfiles) section, repo stores it's configuration in two main
configuration cache locations. There is the global and local config caches. In these cache folder
locations there is a `config.toml` file.

| Name             | Default      | Description                                                                        |
| ----             | ----         | -----------                                                                        |
| root             | `$HOME/repo` | The path relative to the workspace root, the repository location in the workspace. |
| cli              | `false`      | A flag to determine if repo should execute git from command line or libgit2.       |
| default_host     | `github.com` | The default host to use if a query is just 'user/repo'                             |
| default_scheme   | `https`      | The scheme type of the generated url: [`http, https, git, ssh`]                    |
| default_ssh_user | `git`        | Default ssh user when generating a url with ssh scheme.                            |
| shell            | `bash -c`    | The shell that all external command line calls will use.                           |
| include          | empty        | A list of tags. Repositories that have these tags will be shown and operated on    |
| exclude          | empty        | A list of tags. Repositories that have these tags will be excluded from operations |

#### Example

```toml
# ~/.config/repo/config.toml
root = '~/dev/workspace'
cli = false
default_host = 'github.com'
default_scheme = 'ssh'
default_ssh_user = 'git'
shell = [
    'bash',
    '-c',
]
include = ['personal']
exclude = ['work']
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
[crates-io-image]: https://img.shields.io/crates/v/repo-cli
[crates-io-link]: https://crates.io/crates/repo-cli

[//]: # (general)

[apache-2.0]: https://apache.org/licenses/LICENSE-2.0


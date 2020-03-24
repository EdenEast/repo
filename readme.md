# Repo

Repo is a command line repository management tool written in rust.

## Installation

Currently repo is only available from source. To this end make sure that you have a rust environment
setup. If you are new to rust and want more information on setting up a development environment
check the [rust book's][rust-setup] setup chapter.

```bash
# Install using cargo
cargo install --git https://github.com/edeneast/repo
```

This will install `repo` in your default cargo path `$CARGO_HOME/bin/repo`. Once `repo` is in your
path, the `work` helper function needs to be sourced in order to let `repo` change your shell's cwd.

```bash
# ~/.bashrc
[ -x "$(command -v repo)" ] && eval "$(repo script bash)"
```

[rust-setup]: https://doc.rust-lang.org/book/ch01-01-installation.html


## Development

`repo` is currently under development and subject to change before a `v1.0` release. Have an idea
for repo? Open an issue or fork the project and create a pull request.

## Licence

Repo is licenced under [Apache][apache-2.0] Licence (Version 2.0)

See [LICENSE](./LICENSE) file for more details.

[apache-2.0]: https://apache.org/licenses/LICENSE-2.0


#!/bin/bash
# Script base was taken from 'sharkdp/fd' found here 'https://github.com/sharkdp/fd/blob/v7.5.0/doc/screencast.sh'
# When I saw this I was very excited as I dont have to constantly restart recording because I messed up typing
#
# Usage: This script is padded to terminalizer:
# `terminalizer record --command ./docs/screencast.sh demo`

# Designed to be executed via svg-term from the fd root directory:
# svg-term --command="bash doc/screencast.sh" --out doc/screencast.svg --padding=10
set -e
set -u

PROMPT="λ"

enter() {
    INPUT=$1
    DELAY=1

    prompt
    sleep "$DELAY"
    type "$INPUT"
    sleep 0.5
    printf '%b' "\\n"
    eval "$INPUT"
    type "\\n"
}

prompt() {
  printf '%b ' $PROMPT | pv -q
}

type() {
    # printf '%b' "$1" | pv -qL $((10+(-2 + RANDOM%5)))
    printf '%b' "$1" | pv -qL $((12+(-2 + RANDOM%5)))
}

main() {
    IFS='%'

    enter "repo --help"
    enter "repo tag add --path app app"
    enter "repo tag add --clone 'cargo fetch' rust"
    enter "repo add --tag app --tag repo --work 'pwd' edeneast/repo"
    enter "repo add --clone 'pwd' rust-lang/mdbook"
    enter "repo add clap-rs/clap"
    enter "repo config root"
    enter "repo config root ~/custom/workspaceroot"
    enter "repo config cli true"
    enter "RUST_LOG=trace repo update"
    enter "repo foreach 'git rev-parse --abbrev-ref HEAD'"
    enter "eval \"$(repo init bash -f)\""
    enter "work"
    enter "rp"
    enter "pwd ; ls"

    prompt
    sleep 3
    echo ""

    unset IFS
}

main

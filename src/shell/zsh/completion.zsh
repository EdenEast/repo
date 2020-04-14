autoload -U is-at-least


_repo() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_repo_commands" \
"*::: :->repo" \
&& ret=0
    case $state in
    (repo)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:repo-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" \
'*-t+[Add a tag to repository]: :_repo_tags' \
'*--tag=[Add a tag to repository]: :_repo_tags' \
'-p+[Override the default path of the repository in the workspace.]' \
'--path=[Override the default path of the repository in the workspace.]' \
'-c+[Execute command after being cloned by the update command]' \
'--clone=[Execute command after being cloned by the update command]' \
'-w+[Execute command after calling the work command]' \
'--work=[Execute command after calling the work command]' \
'*-r+[Add an additional remote]' \
'*--remote=[Add an additional remote]' \
'-l[Write repository to local cache]' \
'--local[Write repository to local cache]' \
'-f[Override repository if it is already tracked by repo]' \
'--force[Override repository if it is already tracked by repo]' \
'-u[Flag repository to interact with git through the command line]' \
'--cli[Flag repository to interact with git through the command line]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':URL -- A url link to the repository remote origin.:_files' \
'::NAME -- Name of the repository:_files' \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" \
'(-g --global)-l[Interact with local config.]' \
'(-g --global)--local[Interact with local config.]' \
'(-l --local)-g[Interact with global config]' \
'(-l --local)--global[Interact with global config]' \
'-r[Remove tag instead of adding]' \
'--rm[Remove tag instead of adding]' \
'-e[Open cache file in $EDITOR]' \
'--edit[Open cache file in $EDITOR]' \
'-s[List all config options and values]' \
'--list[List all config options and values]' \
'-n[List only config option names]' \
'--name-only[List only config option names]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::NAME -- Name of configuration option:_repo_config_values' \
'::VALUE -- Value to be set to the configuration option provided:_files' \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" \
'*-t+[Add tag to repository]: :_repo_tags' \
'*--tag=[Add tag to repository]: :_repo_tags' \
'-p+[Override the default path of an attached repository in the workspace.]' \
'--path=[Override the default path of an attached repository in the workspace.]' \
'*-r+[Add an additional remote]' \
'*--remote=[Add an additional remote]' \
'(-g --global)-l[Change repository to be a stored in the local cache]' \
'(-g --global)--local[Change repository to be a stored in the local cache]' \
'(-l --local)-g[Change repository to be a stored in the global cache]' \
'(-l --local)--global[Change repository to be a stored in the global cache]' \
'-e[Open cache file in $EDITOR]' \
'--edit[Open cache file in $EDITOR]' \
'-u[Flag repository to interact with git through the command line]' \
'--cli[Flag repository to interact with git through the command line]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::NAME -- Name of the repository to be edited:_repo_repositories' \
&& ret=0
;;
(foreach)
_arguments "${_arguments_options[@]}" \
'*-t+[Perform operation on only repositories that contain tag]: :_repo_tags' \
'*--tag=[Perform operation on only repositories that contain tag]: :_repo_tags' \
'(-a --all -g --global)-l[Perform operation on only local repositories]' \
'(-a --all -g --global)--local[Perform operation on only local repositories]' \
'(-l --local -a --all)-g[Perform operation on only global repositories]' \
'(-l --local -a --all)--global[Perform operation on only global repositories]' \
'(-l --local -g --global)-a[Perform operation on all repositories, global and local]' \
'(-l --local -g --global)--all[Perform operation on all repositories, global and local]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':CMD -- Shell command to be executed:_repo_repositories' \
&& ret=0
;;
(init)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':SHELL -- Name of the shell the shell function will generate:(bash zsh fish)' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
'-l[Show only local repositories]' \
'--local[Show only local repositories]' \
'-g[Show only global repositories]' \
'--global[Show only global repositories]' \
'(-l --local -g --global)-a[Show all repositories regardless of config filters]' \
'(-l --local -g --global)--all[Show all repositories regardless of config filters]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" \
'-f[Force removal of tracked repository.]' \
'--force[Force removal of tracked repository.]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':NAME -- Name of repository:__repo__repositories' \
&& ret=0
;;
(tag)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_repo__tag_commands" \
"*::: :->tag" \
&& ret=0
case $state in
    (tag)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:repo-tag-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" \
'-p+[Override the default path of an attached repository in the workspace.]' \
'--path=[Override the default path of an attached repository in the workspace.]' \
'-c+[Execute command after being cloned by the update command]' \
'--clone=[Execute command after being cloned by the update command]' \
'-w+[Execute command after calling the work command]' \
'--work=[Execute command after calling the work command]' \
'-l[Write repository to local cache]' \
'--local[Write repository to local cache]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':NAME -- Name of the tag:_repo_tags' \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" \
'-p+[Override the default path of an attached repository in the workspace.]' \
'--path=[Override the default path of an attached repository in the workspace.]' \
'(-g --global)-l[Change tag to be a stored in the local cache]' \
'(-g --global)--local[Change tag to be a stored in the local cache]' \
'(-l --local)-g[Change tag to be a stored in the global cache]' \
'(-l --local)--global[Change tag to be a stored in the global cache]' \
'-e[Open cache file in $EDITOR]' \
'--edit[Open cache file in $EDITOR]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::NAME -- Name of the repository to be edited:_files' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
'-l[Show only local tags]' \
'--local[Show only local tags]' \
'-g[Show only global tags]' \
'--global[Show only global tags]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" \
'-f[Force removal of tag.]' \
'--force[Force removal of tag.]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':NAME -- Name of tag:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(update)
_arguments "${_arguments_options[@]}" \
'*-t+[Perform operation on only repositories that contain tag]: :_repo_tags' \
'*--tag=[Perform operation on only repositories that contain tag]: :_repo_tags' \
'(-a --all -g --global)-l[Perform operation on only local repositories]' \
'(-a --all -g --global)--local[Perform operation on only local repositories]' \
'(-l --local -a --all)-g[Perform operation on only global repositories]' \
'(-l --local -a --all)--global[Perform operation on only global repositories]' \
'(-l --local -g --global)-a[Perform operation on all repositories, global and local]' \
'(-l --local -g --global)--all[Perform operation on all repositories, global and local]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(work)
_arguments "${_arguments_options[@]}" \
'-q[Only change directory to repository in workspace]' \
'--quick[Only change directory to repository in workspace]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':NAME -- Name of the tracked repository to be worked on:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

# (( $+functions[_repo_commands] )) ||
_repo_commands() {
    local commands; commands=(
        "add:Add a repository to be tracked by repo" \
"config:Get or set configuration options" \
"edit:Edit a repository tracked by repo" \
"foreach:Execute command for every tracked repository" \
"init:Prints the shell function used to integrate repo with shell" \
"list:List repositories tracked by repo" \
"remove:Remove a repository tracked by repo" \
"tag:Manage tags" \
"update:Update tracked repositories in repo with their remotes" \
"work:Generate work command for a repostory" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'repo commands' commands "$@"
}
(( $+functions[_repo__add_commands] )) ||
_repo__add_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo add commands' commands "$@"
}
(( $+functions[_repo__tag__add_commands] )) ||
_repo__tag__add_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo tag add commands' commands "$@"
}
(( $+functions[_repo__config_commands] )) ||
_repo__config_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo config commands' commands "$@"
}
(( $+functions[_repo__edit_commands] )) ||
_repo__edit_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo edit commands' commands "$@"
}
(( $+functions[_repo__tag__edit_commands] )) ||
_repo__tag__edit_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo tag edit commands' commands "$@"
}
(( $+functions[_repo__foreach_commands] )) ||
_repo__foreach_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo foreach commands' commands "$@"
}
(( $+functions[_repo__help_commands] )) ||
_repo__help_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo help commands' commands "$@"
}
(( $+functions[_repo__tag__help_commands] )) ||
_repo__tag__help_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo tag help commands' commands "$@"
}
(( $+functions[_repo__init_commands] )) ||
_repo__init_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo init commands' commands "$@"
}
(( $+functions[_repo__list_commands] )) ||
_repo__list_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo list commands' commands "$@"
}
(( $+functions[_repo__tag__list_commands] )) ||
_repo__tag__list_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo tag list commands' commands "$@"
}
(( $+functions[_repo__remove_commands] )) ||
_repo__remove_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo remove commands' commands "$@"
}
(( $+functions[_repo__tag__remove_commands] )) ||
_repo__tag__remove_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo tag remove commands' commands "$@"
}
(( $+functions[_repo__tag_commands] )) ||
_repo__tag_commands() {
    local commands; commands=(
        "add:Add a tag to repo" \
"edit:Edit a tag stored in repo" \
"list:List tags stored in repo" \
"remove:Remove a tag from repo" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'repo tag commands' commands "$@"
}
(( $+functions[_repo__update_commands] )) ||
_repo__update_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo update commands' commands "$@"
}
(( $+functions[_repo__work_commands] )) ||
_repo__work_commands() {
    local commands; commands=(

    )
    _describe -t commands 'repo work commands' commands "$@"
}

(( $+functions[_repo_repositories] )) ||
_repo_repositories() {
  local repos; repos=(${(f)"$(_call_program list)"})
  _describe -t repos 'repos' repos "$@"
}

(( $+functions[_repo_tags] )) ||
_repo_tags() {
  local tags; tags=(${(f)"$(_call_program tag list)"})
  _describe -t tags 'tags' tags "$@"
}

(( $+functions[_repo_config_values] )) ||
_repo_config_values() {
  local values; values=(${(f)"$(_call_program config --list --name-only)"})
  _describe -t values 'values' values "$@"
}

compdef _repo repo;

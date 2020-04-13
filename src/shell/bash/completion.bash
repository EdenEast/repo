function __repo_repositories()
{
    local projects=()
    while read line; do
        projects+=($line)
    done < <(repo list)
    echo ${projects[@]}
}

function __repo_tags()
{
    local tags=()
    while read line; do
        tags+=($line)
    done < <(repo tag list)
    echo ${tags[@]}
}

_repo() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    __repo_comp()
    {
        local cur_="${3-$cur}"

        case "$cur_" in
            --*=)
                ;;
            *)
                local c i=0 IFS=$' \t\n'
                for c in $1; do
                    c="$c${4-}"
                    if [[ $c == "$cur_"* ]]; then
                        case $c in
                            --*=*|*.) ;;
                            *) c="$c " ;;
                        esac
                        COMPREPLY[i++]="${2-}$c"
                    fi
                done
                ;;
        esac
    }

    __find_on_cmdline() {
        local word subcommand c=1
        while [ $c -lt $COMP_CWORD ]; do
            word="${COMP_WORDS[c]}"
            for subcommand in $1; do
                if [ "$subcommand" = "$word" ]; then
                    echo "$subcommand"
                    return
                fi
            done
            ((c++))
        done
    }

    __repo_config_values()
    {
        local values=()
        while read line; do
            values+=($line)
        done < <(repo config --list --name-only)
        echo ${values[@]}
    }

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            repo)
                cmd="repo"
                ;;

            add)
                cmd+="__add"
                ;;
            config)
                cmd+="__config"
                ;;
            edit)
                cmd+="__edit"
                ;;
            foreach)
                cmd+="__foreach"
                ;;
            help)
                cmd+="__help"
                ;;
            init)
                cmd+="__init"
                ;;
            list)
                cmd+="__list"
                ;;
            remove)
                cmd+="__remove"
                ;;
            tag)
                cmd+="__tag"
                ;;
            update)
                cmd+="__update"
                ;;
            work)
                cmd+="__work"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        repo)
            opts="-h -V --help --version add config edit foreach init list remove tag update work help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi

            case "${prev}" in
                *) COMPREPLY=() ;;
            esac

            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;

        repo__add)
            # Check if the prev argument was a tag, if so then complete the list of current tags
            case "$prev" in
                --tag) __repo_comp "$(__repo_tags)" ; return 0 ;;
            esac

            # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--local --force --cli --help --version --tag --path --clone --work --remote" ; return 0 ;;
            esac
            return 0
            ;;

        repo__config)
            local values="$(__repo_config_values)"
            local value="$(__find_on_cmdline "$values")"
            local ops="--local --global --rm --edit --list --name-only --help --version"
            case "$value,$cur" in
                *,--*) __repo_comp "$ops" ; return 0 ;;
                ,--*) __repo_comp "$ops" ; return 0 ;;
                ,*) __repo_comp "$values" ; return 0 ;;
            esac
            return 0
            ;;

        repo__edit)
            # Check if the prev argument was a tag, if so then complete the list of current tags
            case "$prev" in
                --tag) __repo_comp "$(__repo_tags)" ; return 0 ;;
            esac

            # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--local --global --edit --cli --help --version --tag --path --remote" ; return 0 ;;
            esac
            __repo_comp "$(__repo_repositories)"
            return 0
            ;;

        repo__foreach)
            # Check if the prev argument was a tag, if so then complete the list of current tags
            case "$prev" in
                --tag) __repo_comp "$(__repo_tags)" ; return 0 ;;
            esac

            # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--local --global --all --help --version --tag" ; return 0 ;;
            esac
            return 0
            ;;

        repo__help)
            opts="-h -V --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *) COMPREPLY=() ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;

        repo__init)
            local shells="bash zsh fish"
            local value="$(__find_on--help --version_cmdline "$values")"
            local ops="--help --version"
            case "$value,$cur" in
                *,--*) __repo_comp "$ops" ; return 0 ;;
                ,--*) __repo_comp "$ops" ; return 0 ;;
                ,*) __repo_comp "$shells" ; return 0 ;;
            esac
            __repo_comp "$shells"
            return 0
            ;;

        repo__list)
            # Check if the prev argument was a tag, if so then complete the list of current tags
            case "$prev" in
                --tag) __repo_comp "$(__repo_tags)" ; return 0 ;;
            esac

            # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--local --global --all --help --version" ; return 0 ;;
            esac
            __repo_comp "$(__repo_repositories)"
            return 0
            ;;

        repo__remove)
            # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--force --help --version" ; return 0 ;;
            esac
            __repo_comp "$(__repo_repositories)"
            return 0
            ;;

        repo__tag)
            opts=" -h -V --help --version add edit list remove help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *) COMPREPLY=() ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;

        repo__tag__add)
             # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--local --help --version --path --clone --work" ; return 0 ;;
            esac
            return 0
            ;;

        repo__tag__edit)
             # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--local --global --edit --help --version --path" ; return 0 ;;
            esac
            __repo_comp "$(__repo_tags)"
            return 0
            ;;

        repo__tag__help)
            opts=" -h -V --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *) COMPREPLY=() ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;

        repo__tag__list)
             # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--local --global --help --version" ; return 0 ;;
            esac
            return 0
            ;;

        repo__tag__remove)
             # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--force --help --version" ; return 0 ;;
            esac
            __repo_comp "$(__repo_tags)"
            return 0
            ;;

        repo__update)
             # Check if the prev argument was a tag, if so then complete the list of current tags
            case "$prev" in
                --tag) __repo_comp "$(__repo_tags)" ; return 0 ;;
            esac

            # Check if the current argument is '--', if so then complete options for add
            case "$cur" in
                --*) __repo_comp "--local --global --all --help --version --tag" ; return 0 ;;
            esac
            return 0
            ;;
    esac

    unset __repo_comp
    unset __find_on_cmdline
    unset __repo_config_values
}

complete -F _repo -o bashdefault -o default repo

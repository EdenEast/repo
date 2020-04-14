function work()
{
  local PROJECT="$(repo list | fzf --cycle --query=$1 --preview-window=top:50% --no-mouse)"
  local SCRIPT="$(repo work $PROJECT $2)"

  case $(uname -s) in
    MINGW*|MSYS*) SCRIPT="cd $(echo "/${SCRIPT:3}" | sed -e 's/\\/\//g' -e 's/://')" ;;
  esac
  [ $? -eq 0 ] && eval "$SCRIPT" || printf "$SCRIPT"
}

function _work()
{
  COMPREPLY=($(compgen -W "$(__repo_repositories)" -- ${COMP_WORDS[1]}))
}

complete -F _work work

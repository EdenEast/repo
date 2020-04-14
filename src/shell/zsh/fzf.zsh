unction work()
{
  PROJECT="$(repo list | fzf --cycle --query=$1 --preview-window=top:50% --no-mouse)"
  SCRIPT="$(repo work $PROJECT $2)"
  case $(uname -s) in
    MINGW*|MSYS*) SCRIPT="cd $(echo "/${SCRIPT:3}" | sed -e 's/\\/\//g' -e 's/://')" ;;
  esac
  [ $? -eq 0 ] && eval "$SCRIPT" || printf "$SCRIPT"
}

function _work()
{
  local repos
  repo list | while read line; do
    repos+=( $line )
  done
  _describe -t repos 'repository names' repos
}

compdef _work work

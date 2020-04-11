function work()
{
  local SCRIPT="$(repo work $@)"
  case $(uname -s) in
    MINGW*|MSYS*) SCRIPT="cd $(echo "/${SCRIPT:3}" | sed -e 's/\\/\//g' -e 's/://')" ;;
  esac
  [ $? -eq 0 ] && eval "$SCRIPT" || printf "$SCRIPT"
}

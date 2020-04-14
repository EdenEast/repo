function __fish_repo_use_internal
  if test $argv[1] -eq 0
    echo $argv[2] | source
  else
    printf "$argv[2]\n"
  end
end

function work
  set -l PROJECT (repo list | fzf --cycle --preview-window=top:50% --no-mouse --query "$1")
  set -l script (repo work $PROJECT $2)
  __fish_repo_use_internal $status $script
end

complete -c work -f -xa "(repo list)"

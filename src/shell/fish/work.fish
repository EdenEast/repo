function __fish_repo_use_internal
  if test $argv[1] -eq 0
    echo $argv[2] | source
  else
    printf "$argv[2]\n"
  end
end

function work
  set -l script (repo work $argv)
  __fish_repo_use_internal $status $script
end

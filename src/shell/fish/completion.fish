for x in add edit foreach update
    complete -c repo -x -n "__fish_seen_subcommand_from $x" -s t -l tag -a "(repo tag list)"
end

for x in edit remove work
    complete -c repo -x -n "__fish_seen_subcommand_from $x" -a "(repo list)"
end

complete -c repo -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c repo -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
complete -c repo -n "__fish_use_subcommand" -f -a "add" -d 'Add a repository to be tracked by repo'
complete -c repo -n "__fish_use_subcommand" -f -a "config" -d 'Get or set configuration options'
complete -c repo -n "__fish_use_subcommand" -f -a "edit" -d 'Edit a repository tracked by repo'
complete -c repo -n "__fish_use_subcommand" -f -a "foreach" -d 'Execute command for every tracked repository'
complete -c repo -n "__fish_use_subcommand" -f -a "init" -d 'Prints the shell function used to integrate repo with shell'
complete -c repo -n "__fish_use_subcommand" -f -a "inspect" -d 'Inspect a repository and view its properties'
complete -c repo -n "__fish_use_subcommand" -f -a "list" -d 'List repositories tracked by repo'
complete -c repo -n "__fish_use_subcommand" -f -a "remove" -d 'Remove a repository tracked by repo'
complete -c repo -n "__fish_use_subcommand" -f -a "tag" -d 'Manage tags'
complete -c repo -n "__fish_use_subcommand" -f -a "update" -d 'Update tracked repositories in repo with their remotes'
complete -c repo -n "__fish_use_subcommand" -f -a "work" -d 'Generate work command for a repostory'
complete -c repo -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'

complete -c repo -n "__fish_seen_subcommand_from add" -s t -l tag -d 'Add a tag to repository'
complete -c repo -n "__fish_seen_subcommand_from add" -s p -l path -d 'Override the default path of the repository in the workspace.'
complete -c repo -n "__fish_seen_subcommand_from add" -s c -l clone -d 'Execute command after being cloned by the update command'
complete -c repo -n "__fish_seen_subcommand_from add" -s w -l work -d 'Execute command after calling the work command'
complete -c repo -n "__fish_seen_subcommand_from add" -s r -l remote -d 'Add an additional remote'
complete -c repo -n "__fish_seen_subcommand_from add" -s l -l local -d 'Write repository to local cache'
complete -c repo -n "__fish_seen_subcommand_from add" -s f -l force -d 'Override repository if it is already tracked by repo'
complete -c repo -n "__fish_seen_subcommand_from add" -s u -l cli -d 'Flag repository to interact with git through the command line'

complete -c repo -n "__fish_seen_subcommand_from config" -s l -l local -d 'Interact with local config.'
complete -c repo -n "__fish_seen_subcommand_from config" -s g -l global -d 'Interact with global config'
complete -c repo -n "__fish_seen_subcommand_from config" -s r -l rm -d 'Remove tag instead of adding'
complete -c repo -n "__fish_seen_subcommand_from config" -s e -l edit -d 'Open cache file in $EDITOR'
complete -c repo -n "__fish_seen_subcommand_from config" -s s -l list -d 'List all config options and values'
complete -c repo -n "__fish_seen_subcommand_from config" -s n -l name-only -d 'List only config option names'

complete -c repo -n "__fish_seen_subcommand_from edit" -s t -l tag -d 'Add tag to repository'
complete -c repo -n "__fish_seen_subcommand_from edit" -s p -l path -d 'Override the default path of an attached repository in the workspace.'
complete -c repo -n "__fish_seen_subcommand_from edit" -s r -l remote -d 'Add an additional remote'
complete -c repo -n "__fish_seen_subcommand_from edit" -s l -l local -d 'Change repository to be a stored in the local cache'
complete -c repo -n "__fish_seen_subcommand_from edit" -s g -l global -d 'Change repository to be a stored in the global cache'
complete -c repo -n "__fish_seen_subcommand_from edit" -s e -l edit -d 'Open cache file in $EDITOR'
complete -c repo -n "__fish_seen_subcommand_from edit" -s u -l cli -d 'Flag repository to interact with git through the command line'

complete -c repo -n "__fish_seen_subcommand_from foreach" -s t -l tag -d 'Perform operation on only repositories that contain tag'
complete -c repo -n "__fish_seen_subcommand_from foreach" -s l -l local -d 'Perform operation on only local repositories'
complete -c repo -n "__fish_seen_subcommand_from foreach" -s g -l global -d 'Perform operation on only global repositories'
complete -c repo -n "__fish_seen_subcommand_from foreach" -s a -l all -d 'Perform operation on all repositories, global and local'

complete -c repo -n "__fish_seen_subcommand_from inspect" -s f -l format -d 'Define the output format of the inspection'
complete -c repo -n "__fish_seen_subcommand_from inspect" -x -a "bash ron toml"

complete -c repo -n "__fish_seen_subcommand_from list" -s l -l local -d 'Show only local repositories'
complete -c repo -n "__fish_seen_subcommand_from list" -s g -l global -d 'Show only global repositories'
complete -c repo -n "__fish_seen_subcommand_from list" -s a -l all -d 'Show all repositories regardless of config filters'

complete -c repo -n "__fish_seen_subcommand_from remove" -s f -l force -d 'Force removal of tracked repository.'

complete -c repo -n "__fish_seen_subcommand_from tag" -f -a "add" -d 'Add a tag to repo'
complete -c repo -n "__fish_seen_subcommand_from tag" -f -a "edit" -d 'Edit a tag stored in repo'
complete -c repo -n "__fish_seen_subcommand_from tag" -f -a "list" -d 'List tags stored in repo'
complete -c repo -n "__fish_seen_subcommand_from tag" -f -a "remove" -d 'Remove a tag from repo'

complete -c repo -n "__fish_seen_subcommand_from add" -s p -l path -d 'Override the default path of an attached repository in the workspace.'
complete -c repo -n "__fish_seen_subcommand_from add" -s c -l clone -d 'Execute command after being cloned by the update command'
complete -c repo -n "__fish_seen_subcommand_from add" -s w -l work -d 'Execute command after calling the work command'
complete -c repo -n "__fish_seen_subcommand_from add" -s l -l local -d 'Write repository to local cache'

complete -c repo -n "__fish_seen_subcommand_from edit" -s p -l path -d 'Override the default path of an attached repository in the workspace.'
complete -c repo -n "__fish_seen_subcommand_from edit" -s l -l local -d 'Change tag to be a stored in the local cache'
complete -c repo -n "__fish_seen_subcommand_from edit" -s g -l global -d 'Change tag to be a stored in the global cache'
complete -c repo -n "__fish_seen_subcommand_from edit" -s e -l edit -d 'Open cache file in $EDITOR'

complete -c repo -n "__fish_seen_subcommand_from list" -s l -l local -d 'Show only local tags'
complete -c repo -n "__fish_seen_subcommand_from list" -s g -l global -d 'Show only global tags'

complete -c repo -n "__fish_seen_subcommand_from remove" -s f -l force -d 'Force removal of tag.'

complete -c repo -n "__fish_seen_subcommand_from update" -s t -l tag -d 'Perform operation on only repositories that contain tag'
complete -c repo -n "__fish_seen_subcommand_from update" -s l -l local -d 'Perform operation on only local repositories'
complete -c repo -n "__fish_seen_subcommand_from update" -s g -l global -d 'Perform operation on only global repositories'
complete -c repo -n "__fish_seen_subcommand_from update" -s a -l all -d 'Perform operation on all repositories, global and local'

complete -c repo -n "__fish_seen_subcommand_from work" -s q -l quick -d 'Only change directory to repository in workspace'
complete -c repo -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c repo -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'

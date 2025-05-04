# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_roxide_global_optspecs
	string join \n r/recursive l/list i/interactive= p/pattern= f/force= v/verbose d/dir c/check h/help V/version
end

function __fish_roxide_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_roxide_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_roxide_using_subcommand
	set -l cmd (__fish_roxide_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c roxide -n "__fish_roxide_needs_command" -s i -l interactive -d 'whether to prompt before removals' -r -f -a "never\t'Never prompt'
once\t'Prompt once before removing more than three files or when removing recursivly'
always\t'Prompt before every removal'
prompt-protected\t'Prompt only on write-protected files'"
complete -c roxide -n "__fish_roxide_needs_command" -s p -l pattern -d 'remove files matching the pattern. revert will not work on patterns, provide -rp for recursive remove' -r
complete -c roxide -n "__fish_roxide_needs_command" -s f -l force -d 'Forces deletion without moving files to the trash directory' -r -F
complete -c roxide -n "__fish_roxide_needs_command" -s r -l recursive -d 'Remove directories and their contents recursively'
complete -c roxide -n "__fish_roxide_needs_command" -s l -l list -d 'list items which will be affected, (dry run)'
complete -c roxide -n "__fish_roxide_needs_command" -s v -l verbose -d 'Enable verbose output'
complete -c roxide -n "__fish_roxide_needs_command" -s d -l dir -d 'remove empty directories'
complete -c roxide -n "__fish_roxide_needs_command" -s c -l check -d 'Will check health of roxide in user env'
complete -c roxide -n "__fish_roxide_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c roxide -n "__fish_roxide_needs_command" -s V -l version -d 'Print version'
complete -c roxide -n "__fish_roxide_needs_command" -a "revert" -d 'revert the previous remove'
complete -c roxide -n "__fish_roxide_needs_command" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c roxide -n "__fish_roxide_using_subcommand revert" -s h -l help -d 'Print help'
complete -c roxide -n "__fish_roxide_using_subcommand help; and not __fish_seen_subcommand_from revert help" -f -a "revert" -d 'revert the previous remove'
complete -c roxide -n "__fish_roxide_using_subcommand help; and not __fish_seen_subcommand_from revert help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'

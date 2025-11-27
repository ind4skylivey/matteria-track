# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_materiatrack_global_optspecs
	string join \n c/config= f/format= v/verbose h/help V/version
end

function __fish_materiatrack_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_materiatrack_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_materiatrack_using_subcommand
	set -l cmd (__fish_materiatrack_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c materiatrack -n "__fish_materiatrack_needs_command" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_needs_command" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_needs_command" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "track" -d '⚔️ Start tracking time on a task'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "t" -d '⚔️ Start tracking time on a task'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "finish" -d '✓ Finish the current tracking session'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "f" -d '✓ Finish the current tracking session'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "status" -d '💎 Show current tracking status'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "s" -d '💎 Show current tracking status'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "list" -d '✨ List tracked entries'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "l" -d '✨ List tracked entries'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "project" -d '🏆 Manage projects'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "task" -d '⭐ Manage tasks'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "stats" -d '📊 Show time statistics'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "statusbar" -d '🖥️ Output for DWM/i3 statusbar'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "dashboard" -d '🎨 Launch interactive TUI dashboard'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "ui" -d '🎨 Launch interactive TUI dashboard'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "config" -d '⚙️ Manage configuration'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "completions" -d 'Generate shell completions (bash, zsh, fish)'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "import" -d '📤 Import data from Zeit or other trackers'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "export" -d '📥 Export data to various formats'
complete -c materiatrack -n "__fish_materiatrack_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand track" -s p -l project -d 'Project name' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand track" -s t -l task -d 'Task name' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand track" -l begin -d 'Start time offset (e.g., -0:15 for 15 minutes ago)' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand track" -s n -l notes -d 'Optional notes for this tracking session' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand track" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand track" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand track" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand track" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand track" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand t" -s p -l project -d 'Project name' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand t" -s t -l task -d 'Task name' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand t" -l begin -d 'Start time offset (e.g., -0:15 for 15 minutes ago)' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand t" -s n -l notes -d 'Optional notes for this tracking session' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand t" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand t" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand t" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand t" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand t" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand finish" -s t -l task -d 'Switch to a different task when finishing' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand finish" -l begin -d 'Adjust the start time' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand finish" -l end -d 'Set the end time offset' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand finish" -s n -l notes -d 'Add notes to the entry' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand finish" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand finish" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand finish" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand finish" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand finish" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand f" -s t -l task -d 'Switch to a different task when finishing' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand f" -l begin -d 'Adjust the start time' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand f" -l end -d 'Set the end time offset' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand f" -s n -l notes -d 'Add notes to the entry' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand f" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand f" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand f" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand f" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand f" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand status" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand status" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand status" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand status" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand status" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand s" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand s" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand s" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand s" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand s" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand list" -l since -d 'Show entries since this datetime (ISO8601 or relative)' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand list" -s n -l limit -d 'Maximum number of entries to show' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand list" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand list" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand list" -l only-projects-and-tasks -d 'Only show projects and tasks (no entries)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand list" -l total -d 'Show total time in output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand list" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand list" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand list" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand l" -l since -d 'Show entries since this datetime (ISO8601 or relative)' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand l" -s n -l limit -d 'Maximum number of entries to show' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand l" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand l" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand l" -l only-projects-and-tasks -d 'Only show projects and tasks (no entries)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand l" -l total -d 'Show total time in output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand l" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand l" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand l" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -f -a "add" -d 'Add a new project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -f -a "list" -d 'List all projects'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -f -a "update" -d 'Update a project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -f -a "remove" -d 'Remove a project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and not __fish_seen_subcommand_from add list update remove help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from add" -s C -l color -d 'Project color (hex code)' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from add" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from add" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from add" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from add" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from list" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from list" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from list" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from update" -l new-name -d 'New project name' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from update" -s C -l color -d 'New color' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from update" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from update" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from update" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from update" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from update" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from remove" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from remove" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from remove" -l force -d 'Force removal without confirmation'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from remove" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from remove" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from remove" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from help" -f -a "add" -d 'Add a new project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all projects'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from help" -f -a "update" -d 'Update a project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from help" -f -a "remove" -d 'Remove a project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand project; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -f -a "add" -d 'Add a new task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -f -a "list" -d 'List all tasks'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -f -a "update" -d 'Update a task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -f -a "remove" -d 'Remove a task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and not __fish_seen_subcommand_from add list update remove help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from add" -s p -l project -d 'Project to add task to' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from add" -s g -l git-repo -d 'Git repository path for this task' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from add" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from add" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from add" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from add" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from list" -s p -l project -d 'Filter by project' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from list" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from list" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from list" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from list" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from update" -s p -l project -d 'Project the task belongs to' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from update" -l new-name -d 'New task name' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from update" -s g -l git-repo -d 'New git repository path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from update" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from update" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from update" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from update" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from update" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from remove" -s p -l project -d 'Project the task belongs to' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from remove" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from remove" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from remove" -l force -d 'Force removal without confirmation'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from remove" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from remove" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from remove" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "add" -d 'Add a new task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all tasks'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "update" -d 'Update a task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "remove" -d 'Remove a task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -l since -d 'Show stats since this date' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -l today -d 'Show stats for today only'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -l week -d 'Show stats for this week'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -l month -d 'Show stats for this month'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -l by-project -d 'Group stats by project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -l by-task -d 'Group stats by task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand stats" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand statusbar" -l icon -d 'Icon prefix' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand statusbar" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand statusbar" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand statusbar" -s s -l short -d 'Use short format'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand statusbar" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand statusbar" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand statusbar" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand dashboard" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand dashboard" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand dashboard" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand dashboard" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand dashboard" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand ui" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand ui" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand ui" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand ui" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand ui" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -f -a "show" -d 'Show current configuration'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -f -a "edit" -d 'Edit configuration file'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -f -a "reset" -d 'Reset configuration to defaults'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -f -a "set" -d 'Set a configuration value'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -f -a "path" -d 'Show configuration file path'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and not __fish_seen_subcommand_from show edit reset set path help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from show" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from show" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from show" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from show" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from show" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from edit" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from edit" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from edit" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from edit" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from edit" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from reset" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from reset" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from reset" -l force -d 'Force reset without confirmation'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from reset" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from reset" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from reset" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from set" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from set" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from set" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from set" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from set" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from path" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from path" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from path" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from path" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from path" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "show" -d 'Show current configuration'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "edit" -d 'Edit configuration file'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "reset" -d 'Reset configuration to defaults'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "set" -d 'Set a configuration value'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "path" -d 'Show configuration file path'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand config; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand completions" -s o -l out-dir -d 'Output directory (defaults to ./completions)' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand completions" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand completions" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand completions" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand completions" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand completions" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand import" -l zeit -d 'Import from Zeit database' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand import" -l json -d 'Import from JSON file' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand import" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand import" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand import" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand import" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand import" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand export" -s F -l export-format -d 'Export format (json, csv)' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand export" -s o -l output -d 'Output file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand export" -l since -d 'Export entries since this date' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand export" -s c -l config -d 'Configuration file path' -r
complete -c materiatrack -n "__fish_materiatrack_using_subcommand export" -s f -l format -d 'Output format' -r -f -a "pretty\t''
json\t''
plain\t''
statusbar\t''"
complete -c materiatrack -n "__fish_materiatrack_using_subcommand export" -s v -l verbose -d 'Verbose output'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand export" -s h -l help -d 'Print help'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand export" -s V -l version -d 'Print version'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "track" -d '⚔️ Start tracking time on a task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "finish" -d '✓ Finish the current tracking session'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "status" -d '💎 Show current tracking status'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "list" -d '✨ List tracked entries'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "project" -d '🏆 Manage projects'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "task" -d '⭐ Manage tasks'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "stats" -d '📊 Show time statistics'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "statusbar" -d '🖥️ Output for DWM/i3 statusbar'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "dashboard" -d '🎨 Launch interactive TUI dashboard'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "config" -d '⚙️ Manage configuration'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "completions" -d 'Generate shell completions (bash, zsh, fish)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "import" -d '📤 Import data from Zeit or other trackers'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "export" -d '📥 Export data to various formats'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and not __fish_seen_subcommand_from track finish status list project task stats statusbar dashboard config completions import export help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from project" -f -a "add" -d 'Add a new project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from project" -f -a "list" -d 'List all projects'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from project" -f -a "update" -d 'Update a project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from project" -f -a "remove" -d 'Remove a project'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "add" -d 'Add a new task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "list" -d 'List all tasks'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "update" -d 'Update a task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "remove" -d 'Remove a task'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "show" -d 'Show current configuration'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "edit" -d 'Edit configuration file'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "reset" -d 'Reset configuration to defaults'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "set" -d 'Set a configuration value'
complete -c materiatrack -n "__fish_materiatrack_using_subcommand help; and __fish_seen_subcommand_from config" -f -a "path" -d 'Show configuration file path'

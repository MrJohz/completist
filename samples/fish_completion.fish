# The Spec:
#
# myprog foo
# myprog foo subfoo --option 'x'|'y'
# myprog foo subfoo 'filename.txt'
# myprog foo 'on'|'off'
#
# myprog bar
# myprog bar mitzvah
#
# myprog --my-opt 'open'|'closed'


function __fish_at_level
  # set useful variables
  set cmd (commandline -opc)
  set subcmd_index (contains --index '' $argv)
  if [ $subcmd_index -eq 1 ]
    set subcommands # empty list
  else
    set subcommands $argv[1..(math $subcmd_index - 1)]
  end
  if [ $subcmd_index -eq (count $argv) ]
    set ignore_commands # empty list
  else
    set ignore_commands $argv[(math $subcmd_index + 1)..-1]
  end
  set subcommands_len (count $subcommands)

  # Test that the initial subcommands are always equal
  if [ (count $cmd) -le $subcommands_len ]
    return 1
  end
  for i in (seq $subcommands_len)
    if [ $subcommands[$i] != $cmd[(math $i + 1)] ]
      return 1
    end
  end

  # Test that the most recent command isn't a further subcommand
  for i in $ignore_commands
    if [ (count $cmd) -lt (math $subcommands_len + 2) ]
      return 0
    else if [ $cmd[(math $subcommands_len + 2)] = $i ]
      return 1
    end
  end

  return 0
end

# myprog [options]
complete -c 'myprog' --condition '__fish_at_level "" foo bar' -l 'my-opt' -a 'open closed' \
    --no-files --require-parameter --description 'open or close myprog'

# myprog [subcommands]
complete -c 'myprog' --condition '__fish_at_level "" foo bar' -a 'foo' \
    --no-files --description 'foo command'
complete -c 'myprog' --condition '__fish_at_level "" foo bar' -a 'bar' \
    --no-files --description 'bar command'

# myprog foo [arguments]
complete -c 'myprog' --condition '__fish_at_level foo "" subfoo' -a 'on off' \
    --no-files --description 'turn myprog foo on/off'

# myprog foo [subcommands]
complete -c 'myprog' --condition '__fish_at_level foo "" subfoo' -a 'subfoo' \
    --no-files --description 'subfoo command'

# myprog foo subfoo [arguments]
complete -c 'myprog' --condition '__fish_at_level foo subfoo ""' \
    --require-parameter

# myprog foo subfoo [options]
complete -c 'myprog' --condition '__fish_at_level foo subfoo ""' -l 'option' -a 'x y' \
    --no-files --require-parameter --description 'set subfoo to x or y'

# myprog bar [subcommands]
complete -c 'myprog' --condition '__fish_at_level bar "" mitzvah' -a 'mitzvah' \
    --no-files --description 'do mitzvah'

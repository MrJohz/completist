function __at_level
  set cmd (commandline -opc)
  set subcommands $argv
  set subcommands_len (count $subcommands)
  if [ (count $cmd) -eq $subcommands_len ]
    for i in (seq $subcommands_len)
      if [ $subcommands[$i] != $cmd[$i] ]
        return 1
      end
    end
    return 0
  end

  return 1
end

# myprog options
complete -c 'myprog' --condition '__at_level myprog' -l 'my-opt' -a 'open closed' \
    --no-files --require-parameter --description 'open or close myprog'

# myprog subcommands
complete -c 'myprog' --condition '__at_level myprog' -a 'foo' \
    --no-files --description 'foo command'
complete -c 'myprog' --condition '__at_level myprog' -a 'bar' \
    --no-files --description 'bar command'

# myprog foo arguments
complete -c 'myprog' --condition '__at_level myprog foo' -a 'on off' \
    --no-files --description 'turn myprog foo on/off'

# myprog foo subcommands
complete -c 'myprog' --condition '__at_level myprog foo' -a 'subfoo' \
    --no-files --description 'subfoo command'

# myprog foo subfoo arguments
complete -c 'myprog' --condition '__at_level myprog foo subfoo' \
    --require-parameter

# myprog foo subfoo options
complete -c 'myprog' --condition '__at_level myprog foo subfoo' -l 'option' -a 'x y' \
    --no-files --require-parameter --description 'set subfoo to x or y'

# myprog bar subcommands
complete -c 'myprog' --condition '__at_level myprog bar' -a 'mitzvah' \
    --no-files --description 'do mitzvah'


# myprog foo
# myprog foo subfoo --option 'x'|'y'
# myprog foo subfoo 'filename.txt'
# myprog foo 'on'|'off'

# myprog bar
# myprog bar mitzvah

# myprog --my-opt 'open'|'closed'

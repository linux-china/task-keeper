#compdef sq
#autload

local subcmds=("add" "list" "edit")

while read -r line ; do
   if [[ ! $line == Available* ]] ;
   then
      trimed=${line##+([[:space:]])}
      subcmds+=(${trimed/[[:space:]]*/})
   fi
done < <(just -f $HOME/.tk/snippets.just --list)

_describe 'command' subcmds

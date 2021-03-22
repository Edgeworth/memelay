#!/bin/sh
find $(pwd -P) -regextype posix-extended -regex '.*\.(asm|c|cc|cmake|conf|cpp|css|h|hpp|html|java|js|md|py|rb|rs|sh|tex|toml)'

# find . -type f -name '*.*' -exec grep -Iq . {} \; -print | sed 's/^.*\.//' | sort -u

#!/bin/bash -e

rm -rf "$HOME/.forc/git/checkouts"

echo "installing forc"
(cd "$HOME/wrk/fuel/code/sway/forc" && cargo install --path .)

# for folder in project{1..30}; do
# 	mkdir "$folder" &> /dev/null
# done

# for folder in project{1..30}; do
# 	(cd "$folder" && forc init) &> /dev/null
# done

echo "building"
parallel -j40 "cd {} && forc build" ::: project{1..10} #|& grep -vFf lines_to_ignore.txt

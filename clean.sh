#!/bin/bash

# this script cleans up the local project folder to
# free it of the quite large amount of stuff that
# lives in target and node_modules folders. use this
# when you're done working for a while and want to
# kind of store the project without deleting your
# local repo

cargo clean
find . -name "node_modules" -type d -prune | xargs du -chs


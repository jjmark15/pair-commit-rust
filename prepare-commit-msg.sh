#!/bin/sh

COMMIT_MSG_FILE=$1
# shellcheck disable=SC2034
COMMIT_SOURCE=$2
# shellcheck disable=SC2034
SHA1=$3

binary_path="$HOME/.local/bin/pair-commit-tool"

if test ! -f "$binary_path"; then
  echo "Error: pair-commit-tool binary not found"
  exit 1
fi

output=$($binary_path message)
exit_code=$?

if [ $exit_code -eq 0 ]; then
  echo "" >>"$COMMIT_MSG_FILE"
  echo "$output" >>"$COMMIT_MSG_FILE"
fi

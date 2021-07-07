#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlite3)" ]; then
  echo >&2 "Error: `sqlite3` is not installed."
  exit 1
fi

mkdir db && cd db
touch games.db
sqlite3 games.db

>&2 echo "Sqlite database initialized!"
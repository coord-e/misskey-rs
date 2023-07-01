#!/bin/bash

set -eu

cd "$(dirname "$0")"

# Retry the setup several times in case of failure as a workaround of #3
readonly MAX_RETRY=5

count=0
while :; do
  docker-compose build
  docker-compose run --rm web yarn run init || docker-compose run --rm web pnpm run init
  docker-compose up -d web
  if docker-compose run --rm get-token; then
    break
  fi

  if [ $(( count++ )) -ge $MAX_RETRY ]; then
    echo "Too many fails" 2>1
    exit 1
  else
    echo "Retrying... (count: $count)" 2>1
    docker-compose down -v
  fi
done

#!/bin/bash

cd "$(dirname "$0")"

docker-compose build
docker-compose run --rm web yarn run init
docker-compose up -d web
docker-compose run --rm get-token

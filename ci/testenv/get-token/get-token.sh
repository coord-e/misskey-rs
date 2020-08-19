#!/bin/sh

until nc -z web 3000; do
  sleep 1
done

echo -n "::set-output name=admin_token::"

curl -fsSL -XPOST -H 'Content-Type: application/json'    \
	--data '{"username":"admin","password":"admin"}' \
	http://web:3000/api/admin/accounts/create | jq -r .token

echo -n "::set-output name=user_token::"

curl -fsSL -XPOST -H 'Content-Type: application/json'    \
	--data '{"username":"testuser","password":"testuser"}' \
	http://web:3000/api/signup | jq -r .token

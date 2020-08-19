#!/bin/sh

until nc -z web 3000; do
  sleep 1
done

curl -fsSL -XPOST -H 'Content-Type: application/json'    \
	--data '{"username":"testuser","password":"testuser"}' \
	http://web:3000/api/admin/accounts/create | jq -r .token

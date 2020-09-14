#!/bin/sh

until nc -z web 3000; do
  sleep 1
done

ADMIN_TOKEN=$(
  curl -fsS -XPOST -H 'Content-Type: application/json' \
    --data '{"username":"admin","password":"admin"}'   \
    http://web:3000/api/admin/accounts/create | jq -r .token
)

USER_DATA=$(
  curl -fsS -XPOST -H 'Content-Type: application/json'     \
    --data '{"username":"testuser","password":"testuser"}' \
    http://web:3000/api/signup
)

USER_TOKEN=$(echo "$USER_DATA" | jq -r .token)
USER_ID=$(echo "$USER_DATA" | jq -r .id)

curl -fsS -XPOST -H 'Content-Type: application/json'            \
  --data '{"i":"'"${ADMIN_TOKEN}"'","userId":"'"${USER_ID}"'"}' \
  http://web:3000/api/admin/moderators/add || exit 1

echo "::set-output name=admin_token::$ADMIN_TOKEN"
echo "::set-output name=user_token::$USER_TOKEN"

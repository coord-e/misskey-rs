#!/bin/sh

until nc -z web 3000; do
  sleep 1
done

ADMIN_DATA=$(
  curl -fsS -XPOST -H 'Content-Type: application/json' \
    --data '{"username":"admin","password":"admin"}'   \
    http://web:3000/api/admin/accounts/create
)

ADMIN_TOKEN=$(echo "$ADMIN_DATA" | jq -r .token)
ADMIN_ID=$(echo "$ADMIN_DATA" | jq -r .id)

USER_DATA=$(
  curl -fsS -XPOST -H 'Content-Type: application/json'     \
    --data '{"username":"testuser","password":"testuser"}' \
    http://web:3000/api/signup
)

USER_TOKEN=$(echo "$USER_DATA" | jq -r .token)
USER_ID=$(echo "$USER_DATA" | jq -r .id)

# endpoint `admin/moderators/add` is removed in Misskey v13.0.0
curl -fsS -XPOST -H 'Content-Type: application/json'            \
  --data '{"i":"'"${ADMIN_TOKEN}"'","userId":"'"${USER_ID}"'"}' \
  http://web:3000/api/admin/moderators/add ||
(
  # create and assign moderator role
  ROLE_ID=$(
    curl -fsS -XPOST -H 'Content-Type: application/json' \
      --data '{
        "i": "'"${ADMIN_TOKEN}"'",
        "name": "Moderator",
        "description": "",
        "color": "",
        "iconUrl": null,
        "target": "manual",
        "condFormula": {},
        "isPublic": false,
        "isModerator": true,
        "isAdministrator": false,
        "asBadge": false,
        "canEditMembersByModerator": false,
        "policies": {
          "pinLimit": {
            "value": 1000,
            "priority": 0,
            "useDefault": false
          },
          "clipLimit": {
            "value": 1000,
            "priority": 0,
            "useDefault": false
          },
          "antennaLimit": {
            "value": 1000,
            "priority": 0,
            "useDefault": false
          },
          "userListLimit": {
            "value": 1000,
            "priority": 0,
            "useDefault": false
          }
        }
      }' \
    http://web:3000/api/admin/roles/create | jq -r .id \
  ) &&
  curl -fsS -XPOST -H 'Content-Type: application/json' \
    --data '{"i":"'"${ADMIN_TOKEN}"'", "roleId": "'"${ROLE_ID}"'", "userId":"'"${ADMIN_ID}"'"}' \
    http://web:3000/api/admin/roles/assign &&
  curl -fsS -XPOST -H 'Content-Type: application/json' \
    --data '{"i":"'"${ADMIN_TOKEN}"'", "roleId": "'"${ROLE_ID}"'", "userId":"'"${USER_ID}"'"}' \
    http://web:3000/api/admin/roles/assign
) ||
exit 1

echo "::set-output name=admin_token::$ADMIN_TOKEN"
echo "::set-output name=user_token::$USER_TOKEN"

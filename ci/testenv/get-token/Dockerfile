FROM debian:buster-slim

RUN apt-get update -y
RUN apt-get install -y --no-install-recommends wait-for-it jq curl
COPY get-token.sh /get-token.sh

CMD ["wait-for-it", "web:3000", "--quiet", "--timeout=60", "--", "/get-token.sh"]

FROM alpine:3.12

RUN apk add --no-cache jq curl
COPY get-token.sh /get-token.sh

CMD ["/get-token.sh"]

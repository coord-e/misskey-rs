ARG MISSKEY_VERSION
FROM misskey/misskey:${MISSKEY_VERSION}
ARG MISSKEY_ID
COPY default.yml /misskey/.config/default.yml
RUN echo "id: '${MISSKEY_ID}'" >> /misskey/.config/default.yml

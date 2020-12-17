ARG MISSKEY_IMAGE
FROM ${MISSKEY_IMAGE}
ARG MISSKEY_ID
COPY default.yml /misskey/.config/default.yml
RUN echo "id: '${MISSKEY_ID}'" >> /misskey/.config/default.yml

ARG MISSKEY_IMAGE
FROM ${MISSKEY_IMAGE}
ARG MISSKEY_ID
COPY default.yml /misskey/.config/default.yml
USER root
RUN echo "id: '${MISSKEY_ID}'" >> /misskey/.config/default.yml
# Disable rate limit
ENV NODE_ENV=development

ARG MISSKEY_VERSION
FROM misskey/misskey:${MISSKEY_VERSION}
ARG MISSKEY_ID
COPY default.yml /misskey/.config/default.yml
RUN echo "id: '${MISSKEY_ID}'" >> /misskey/.config/default.yml
# for node-gyp
RUN apk add python make gcc g++
# we need typeorm#6417
RUN yarn add typeorm@0.2.29

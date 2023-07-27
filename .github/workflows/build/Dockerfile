# syntax=docker/dockerfile:1
ARG BINARY_NAME=pow-summon-discordbot

FROM --platform=$BUILDPLATFORM busybox:latest AS env
ARG TARGETPLATFORM
ARG BINARY_NAME
RUN --mount=type=bind,source=artifact,target=artifact \
    if [ "$TARGETPLATFORM" = 'linux/amd64' ]; then\
      cp artifact/x86_64-unknown-linux-musl/"$BINARY_NAME" .\
    ;elif [ "$TARGETPLATFORM" = 'linux/arm64' ]; then\
      cp artifact/aarch64-unknown-linux-musl/"$BINARY_NAME" .\
    ;fi

FROM scratch
ARG BINARY_NAME
COPY --from=env --chmod=755 "$BINARY_NAME" binary
ENTRYPOINT ["./binary"]

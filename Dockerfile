FROM rust:bookworm AS build

WORKDIR /usr/src/nettopbox
COPY . .

RUN cargo install --path .

FROM debian:12.1-slim

VOLUME [ "/config" ]

RUN apt-get update && apt-get install -y \
        ffmpeg \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/local/cargo/bin/nettopbox /usr/local/bin/nettopbox

ENTRYPOINT [ "nettopbox" ]
CMD [ "/config/config.yaml" ]

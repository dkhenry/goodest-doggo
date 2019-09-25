FROM rustlang/rust:nightly as builder

ENV ROCKET_ENV prod
ENV ROCKET_DATABASES '{ doggers = { url = "mysql://mysql_user:mysql_password@localhost:3306/doggers" } }'

# muslc is required in order to build the rust image.
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN rustup target add x86_64-unknown-linux-musl

# Sets the environment variable for the cargo build command that follows.
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.10

RUN apk --no-cache add ca-certificates 

# Copy build files over
COPY --from=builder /target/x86_64-unknown-linux-musl/release/doggo-api /usr/local/bin
COPY --from=builder doggo-api/Rocket.toml /usr/local/lib/doggo/
COPY --from=builder doggo-api/templates /usr/local/lib/doggo/templates

WORKDIR /usr/local/lib/doggo/
EXPOSE 8000

CMD ["/usr/local/bin/doggo"]

FROM rustlang/rust:nightly as builder

ENV ROCKET_ENV prod
ENV ROCKET_DATABASES '{ doggers = { url = "mysql://mysql_user:mysql_password@localhost:3306/doggers" } }'

# muslc is required in order to build the rust image.
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN rustup target add x86_64-unknown-linux-gnu

# Change default shell to bash - necessary for next command
SHELL ["/bin/bash", "-c"]

# Add deterministic secret key to config so pods have same secret key and cookies resolve correctly.
RUN echo secret_key = \"$(openssl rand -base64 32)\" >> Rocket.toml
RUN cat Rocket.toml

# Sets the environment variable for the cargo build command that follows.
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo fetch --target x86_64-unknown-linux-gnu
RUN cargo build --target x86_64-unknown-linux-gnu --release

FROM centos

# Copy build files over
COPY --from=builder /target/x86_64-unknown-linux-gnu/release/doggo-api /usr/local/bin/doggo-api
COPY --from=builder Rocket.toml.docker /usr/local/lib/doggo/
COPY --from=builder doggo-api/templates /usr/local/lib/doggo/templates

WORKDIR /usr/local/lib/doggo/
EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/doggo-api"]

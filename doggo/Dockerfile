FROM rust:slim

ENV ROCKET_ENV prod
ENV ROCKET_DATABASES '{ doggers = { url = "mysql://mysql_user:mysql_password@35.247.67.76:3306/doggers" } }'

COPY target/release/doggo /usr/local/bin/
COPY Rocket.toml /usr/local/lib/doggo/
COPY templates /usr/local/lib/doggo/templates

WORKDIR /usr/local/lib/doggo/
EXPOSE 8000

CMD ["/usr/local/bin/doggo"]

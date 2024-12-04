FROM rustlang/rust:nightly AS builder
WORKDIR /app
COPY src ./src
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo install --path .

FROM rustlang/rust:nightly AS runner
COPY migrations /migrations
RUN cargo install diesel_cli --no-default-features --features postgres
RUN apt update && apt upgrade -y && apt install -y libpq-dev libc6
COPY --from=builder /usr/local/cargo/bin/request-mirror /usr/local/bin/request-mirror
COPY ./templates /templates
COPY .env.docker /.env
COPY Rocket.toml /
ENV ROCKET_PROFILE=docker
EXPOSE 80
ENTRYPOINT diesel migration run --database-url $DATABASE_URL --migration-dir /migrations && request-mirror
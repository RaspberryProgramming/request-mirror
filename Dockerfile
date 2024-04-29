FROM rustlang/rust:nightly as builder
WORKDIR ./app
COPY . .
RUN cargo install --path .

FROM rustlang/rust:nightly as runner
COPY migrations /migrations
RUN cargo install diesel_cli --no-default-features --features postgres
RUN apt update && apt install -y libpq-dev libc6
COPY --from=builder /usr/local/cargo/bin/request-mirror /usr/local/bin/request-mirror
COPY ./templates /templates
COPY .env.docker /.env
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=production
EXPOSE 8000
ENTRYPOINT diesel migration run --migration-dir /migrations && request-mirror
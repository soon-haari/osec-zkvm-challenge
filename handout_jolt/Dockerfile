FROM rust:slim AS build

RUN apt-get update && apt-get install -y --no-install-recommends git patch libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*
RUN useradd -m -u 10001 app && mkdir -p /app && chown -R app:app /app
WORKDIR /app
USER app
COPY ./chall .
RUN cargo build -r
# download toolchains and compile guest
RUN RUST_LOG=info cargo run -r

FROM pwn.red/jail
COPY --from=build / /srv
COPY ./flag.txt /srv/flag.txt
RUN cp /srv/app/target/release/chall /srv/app/run
RUN chmod -R 0755 /srv/home
ENV JAIL_MEM=2000M JAIL_ENV_HOME=/home/app JAIL_ENV_RAYON_NUM_THREADS=1 JAIL_TIME=1000 JAIL_CPU=1000
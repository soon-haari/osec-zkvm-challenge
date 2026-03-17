FROM rust:slim-bookworm AS build

RUN apt-get update && apt-get install -y --no-install-recommends git patch && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY ./chall .
RUN rustup target add riscv32im-unknown-none-elf
# build and compile guest
RUN mv `cargo run -r` /app/guest.elf

FROM pwn.red/jail
COPY --from=build /bin /srv/bin
COPY --from=build /usr /srv/usr
COPY --from=build /lib /srv/lib
COPY --from=build /lib64 /srv/lib64
COPY --from=build /app/target/release/chall /srv/app/run
COPY --from=build /app/guest.elf /srv/app/guest
COPY ./flag.txt /srv/flag.txt
ENV JAIL_MEM=2000M JAIL_ENV_RAYON_NUM_THREADS=1 JAIL_TIME=1000 JAIL_CPU=1000 JAIL_ENV_ELF_PATH=/app/guest
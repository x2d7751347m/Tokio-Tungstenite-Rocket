#
# Stage 1 (Build)
#

FROM rust:1.70-slim-buster AS build

WORKDIR /tokio_tungstenite_rocket

COPY . .

RUN apt-get update 
RUN apt install -y cmake && apt-get install -y build-essential gdb && apt install -y pkg-config libssl-dev
# && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

#
# Stage 2 (Run)
#

FROM debian:bookworm-slim

RUN apt-get update  
RUN apt-get install -y build-essential gdb && apt install -y pkg-config libssl-dev && apt-get install -y wget
# && rm -rf /var/lib/apt/lists/*

# RUN mkdir /opt
WORKDIR /opt
# Download a supported openssl version. e.g., openssl-1.1.1u.tar.gz
RUN wget https://www.openssl.org/source/openssl-1.1.1u.tar.gz
RUN tar -zxvf openssl-1.1.1u.tar.gz
WORKDIR /opt/openssl-1.1.1u
RUN ./config && make && make test

RUN mkdir /opt/lib
RUN mv /opt/openssl-1.1.1u/libcrypto.so.1.1 /opt/lib/
RUN mv /opt/openssl-1.1.1u/libssl.so.1.1 /opt/lib/
ENV LD_LIBRARY_PATH=/opt/lib:$LD_LIBRARY_PATH

WORKDIR /tokio_tungstenite_rocket

COPY --from=build /tokio_tungstenite_rocket/target/release/tokio_tungstenite_rocket ./tokio_tungstenite_rocket
COPY --from=build /tokio_tungstenite_rocket/Rocket.toml .

EXPOSE 8000

# And away we go...
CMD [ "./tokio_tungstenite_rocket" ]

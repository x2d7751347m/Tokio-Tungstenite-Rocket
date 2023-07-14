#
# Stage 1 (Build)
#
FROM rust:1.70-slim-buster AS build

WORKDIR /example-api

COPY . .

RUN apt-get update && apt install -y cmake 
RUN apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

#
# Stage 2 (Run)
#

FROM debian:bullseye-slim

WORKDIR /example-api

RUN apt-get update && apt install -y cmake 
RUN apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /example-api/target/release/example-api ./example-api

EXPOSE 80

# And away we go...
CMD [ "./example-api" ]

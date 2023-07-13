#
# Stage 1 (Build)
#

FROM rust:1.70-slim-buster AS build

WORKDIR /pararium

COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

#
# Stage 2 (Run)
#

FROM debian:bullseye-slim

WORKDIR /pararium

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /pararium/target/release/pararium ./pararium

EXPOSE 80

# And away we go...
CMD [ "./pararium" ]

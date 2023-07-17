#
# Stage 1 (Build)
#

FROM rust:1.70-slim-buster AS build

WORKDIR /okapi_example

COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

#
# Stage 2 (Run)
#

FROM debian:bullseye-slim

WORKDIR /okapi_example

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /okapi_example/target/release/okapi_example ./okapi_example

EXPOSE 80

# And away we go...
CMD [ "./okapi_example" ]

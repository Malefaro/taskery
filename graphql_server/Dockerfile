FROM rust

RUN USER=root cargo new --bin graphql_server
RUN mkdir auth_service
COPY ./auth_service ./auth_service
WORKDIR ./graphql_server
COPY ./graphql_server/Cargo.toml ./Cargo.toml
RUN rustup component add rustfmt
RUN cargo build --release
RUN mkdir /cargo
RUN cargo install diesel_cli --root /cargo
RUN rm src/*.rs

ADD ./graphql_server ./

RUN rm ./target/release/deps/graphql_server*
RUN cargo build --release
ENV DATABASE_URL $POSTGRES_URL
CMD DATABASE_URL=$POSTGRES_URL /cargo/bin/diesel migration run; ./target/release/graphql_server

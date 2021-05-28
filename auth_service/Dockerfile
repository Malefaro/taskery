FROM rust

RUN USER=root cargo new --bin auth_service
WORKDIR ./auth_service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
RUN rustup component add rustfmt

ADD . ./

RUN rm ./target/release/deps/auth_service*
RUN cargo build --release
CMD [ "target/release/auth_service" ]

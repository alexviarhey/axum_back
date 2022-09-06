# Stage 1. Generate a recipe.json file for dependencies
FROM rust:latest as planner
WORKDIR /usr/src/dca_api
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2. Build our dependencies
FROM rust:latest as cacher
WORKDIR /usr/src/dca_api
RUN cargo install cargo-chef
COPY --from=planner /usr/src/dca_api/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3. Use the offitial docker image
FROM rust:latest as builder

ENV USER=rust
ENV UID=1001

# Create user
RUN adduser \
	--disabled-password \
	--gecos "" \
	--home "/nonexistent" \
	--shell "/sbin/nologin" \
	--no-create-home \
	--uid "${UID}" \
	"${USER}"


# Set working dir
WORKDIR /usr/src/dca_api

# Copy files from current dir to docker working dir
COPY . .

# Copy deps from cacher stage
COPY --from=cacher /usr/src/dca_api/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release

# Stage 4. Run productin image
FROM gcr.io/distroless/cc-debian11

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY --from=builder /usr/src/dca_api/target/release/axum_back /usr/src/dca_api/axum_back

WORKDIR /usr/src/dca_api

USER rust:rust

# Run applicatin
CMD ["/usr/src/dca-api/axum_back"]

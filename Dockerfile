ARG APP_PATH=/usr/src/dca_api

# Stage 1. Generate a recipe.json file for dependencies
FROM rust:latest as planner
WORKDIR ${APP_PATH}
RUN cargo install cargo-chef
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2. Build our dependencies 
FROM rust:latest as cacher
WORKDIR ${APP_PATH}
RUN cargo install cargo-chef
COPY --from=planner ${APP_PATH}/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3. Use the offitial docker image
FROM rust:latest as build

# Create user
RUN adduser \
	--disabled-password \
	--gecos "" \
	--home "noneexistent" \
	--no-create-home \
	--uid "1001" \
	"rust" \

# Set working dir
WORKDIR ${APP_PATH}

# Copy files from current dir to docker working dir
COPY . .

# Copy deps from cacher stage
COPY --from=cacher ${APP_PATH}/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release

# Stage 4. Run productin image
FROM gcr.io/distroless/cc-debian11

COPY --from=build ${APP_PATH}/target/release/axum_back ${APP_PATH}/axum_back

WORKDIR ${APP_PATH}

# Run applicatin
CMD ["./axum_back"]

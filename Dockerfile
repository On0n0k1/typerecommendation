FROM rust:latest AS builder

RUN USER=root cargo new --bin typeahead-backend-on0n0k1

RUN pwd

WORKDIR /typeahead-backend-on0n0k1

COPY ./Cargo.toml ./Cargo.toml

# Install required dependencies first
# --no-default-features will exclude code and dependencies that load from a .env file
RUN cargo build --release

# Remove auto-generated source files
RUN rm src/*.rs target/release/deps/typeahead_backend_on0n0k1*

# Include source code
ADD . ./

RUN cargo test

# Release will exclude excessive debug log messages
# --no-default-features will exclude useless dependencies for Docker runtime
RUN cargo build --release

FROM debian:buster-slim


ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*


# EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

# Not required, will use "0.0.0.0" if HOST isn't set
# ENV HOST="127.0.0.1"

# This will export the PORT environment variable to your application.
# It has 12345 as default value, but when running the container we might pass
# any other port. You shouldn't change this unless you really know what you are doing.
ENV PORT 12345

# This will export the SUGGESTION_NUMBER environment variable to your application.
# It has 10 as default value, but when running the container we might pass
# any other value
ENV SUGGESTION_NUMBER 10

# Avoid changing this too; it will expose the port so
# other containers can connect to your app.
EXPOSE $PORT

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# Copy the executable
COPY --from=builder /typeahead-backend-on0n0k1/target/release/typeahead-backend-on0n0k1 ${APP}/typeahead-backend-on0n0k1

# Include the json file
COPY --from=builder /typeahead-backend-on0n0k1/names.json ${APP}/names.json

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

# Run the executable
CMD ["./typeahead-backend-on0n0k1"]
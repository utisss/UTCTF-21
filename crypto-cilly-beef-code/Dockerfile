### Global args
ARG BIN_NAME=cilly_beef_code
ARG PORT=3000
ARG APP=/usr/app
ARG APP_USER=appuser

### Stage 1 (build)
FROM alpine:latest as builder
ARG BIN_NAME

RUN apk update \
    && apk add rustup build-base upx

RUN rustup-init -y --default-toolchain nightly --profile minimal \
	&& source $HOME/.cargo/env

RUN USER=root $HOME/.cargo/bin/cargo new --bin $BIN_NAME
WORKDIR ./$BIN_NAME

COPY ./Cargo.toml ./Cargo.toml

RUN $HOME/.cargo/bin/cargo build --release \
	&& rm ./target/release/deps/$BIN_NAME* src/*.rs
#RUN $HOME/.cargo/bin/cargo build \
#	&& rm ./target/debug/deps/$BIN_NAME* src/*.rs

COPY ./src ./src

RUN $HOME/.cargo/bin/cargo build --release \
	&& upx ./target/release/$BIN_NAME
#RUN $HOME/.cargo/bin/cargo build \
#	&& upx ./target/debug/$BIN_NAME

### Stage 2 (setup prod env)
FROM alpine:latest as pre-squash
ARG BIN_NAME
ARG PORT
ARG APP
ARG APP_USER

RUN apk update \
    && apk add libgcc

RUN adduser --disabled-password --system --no-create-home --shell /sbin/nologin $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /$BIN_NAME/target/release/$BIN_NAME ${APP}/$BIN_NAME
#COPY --from=builder /$BIN_NAME/target/debug/$BIN_NAME ${APP}/$BIN_NAME

RUN chown -R $APP_USER: ${APP}


# Configure remaining container settings
ENV BIN_NAME $BIN_NAME
ENV PORT $PORT

EXPOSE $PORT/tcp

USER $APP_USER
WORKDIR ${APP}

HEALTHCHECK CMD ./$BIN_NAME HEALTHCHECK

CMD ./$BIN_NAME

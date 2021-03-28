FROM rust:1.51.0-alpine as builder
WORKDIR /app-build
COPY ./Cargo.toml ./Cargo.toml
COPY ./src/ ./src/ 
COPY ./public/ ./public/ 
RUN \
  apk add --no-cache musl-dev openssl-dev && \
  cargo build --release \
 && echo "#!/bin/sh" > run.sh \
 && bin=$(find ./target/release -maxdepth 1 -perm -111 -type f| head -n 1) \
 && echo ./${bin##*/} >> run.sh \
 && chmod 755 run.sh

FROM alpine:3.13
ENV APP_USER=appuser
RUN addgroup -S $APP_USER && adduser -S $APP_USER -G $APP_USER
WORKDIR "/app/public"
COPY --from=builder --chown=$APP_USER /app-build/public/* ./
WORKDIR "/app"
COPY --from=builder --chown=$APP_USER /app-build/public/* /app-build/target/release/* /app-build/run.sh ./
USER $APP_USER
ENV PORT 8080
EXPOSE 8080
CMD ["./run.sh"]
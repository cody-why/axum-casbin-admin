FROM debian:buster-slim

ENV TZ Asia/Shanghai

WORKDIR /app

COPY ./config/application.conf /app/config/application.conf
COPY ./config/log4rs.yaml /app/config/log4rs.yaml
COPY ./config/auth_model.conf /app/config/auth_model.conf
COPY ./config/errors.json /app/config/errors.json
COPY ./target/release/axum-admin /app/

CMD ["./axum-admin"]
FROM alpine:latest
EXPOSE 80
ENV APP_DIR=/iss
COPY ./dist $APP_DIR
RUN chmod +x $APP_DIR/server/botw-iss

WORKDIR $APP_DIR

ENV ISS_LOG=info                    \
    ISS_LOG_COLOR=false             \
    ISS_MANUAL_PATH=/iss/manual     \
    ISS_LEGACY_PATH=/iss/legacy

ENTRYPOINT ["/iss/server/botw-iss"]

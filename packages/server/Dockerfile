FROM alpine:latest
RUN apk add libstdc++ libgcc

EXPOSE 80
ENV APP_DIR=/iss
COPY ./dist $APP_DIR
RUN chmod +x $APP_DIR/server-bin

WORKDIR $APP_DIR

ENTRYPOINT ["/iss/server-bin"]

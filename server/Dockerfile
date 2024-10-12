# FROM golang:alpine AS build
# COPY . /build/
# WORKDIR /build
# RUN ls -a
# RUN go mod tidy
# RUN go mod download
# RUN go build -o server
#


FROM ubuntu:latest
EXPOSE 80
ENV APP_DIR=/opt/app
COPY ./dist $APP_DIR
# COPY --from=build /build/server $APP_DIR/server
RUN chmod +x $APP_DIR/server/server
# RUN chmod +x $APP_DIR/server/test.sh

WORKDIR $APP_DIR

ENV ISS_LOG=info                    \
    ISS_LOG_COLOR=false             \
    ISS_MANUAL_PATH=/opt/iss/manual \
    ISS_LEGACY_PATH=/opt/iss/legacy

# RUN stat /bin/sh
# RUN stat ./server/test.sh

# ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["./server/server"]

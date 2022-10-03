FROM alpine
RUN apk add --no-cache cargo
WORKDIR /app
COPY . .
RUN cargo build
CMD ["cargo", "run"]

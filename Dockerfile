FROM rust:latest AS build
WORKDIR /build
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli
RUN cargo install trunk
COPY . .
RUN trunk build --release

FROM nginx:alpine-slim AS final
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /build/dist /usr/share/nginx/html
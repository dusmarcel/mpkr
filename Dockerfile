FROM rust:1-bookworm AS build
WORKDIR /build

RUN apt-get update \
    && apt-get install -y --no-install-recommends nodejs npm \
    && npm install -g @tailwindcss/cli \
    && rustup target add wasm32-unknown-unknown \
    && cargo install trunk \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY package.json package-lock.json ./
RUN npm ci --no-fund --no-audit

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY index.html styles.css ./

RUN trunk build --release

FROM nginx:alpine-slim AS final
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /build/dist /usr/share/nginx/html

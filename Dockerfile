# syntax=docker/dockerfile:1

FROM node:26-alpine AS deps
WORKDIR /app
RUN npm install -g pnpm@10.32.1
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

FROM deps AS build
COPY . .
RUN pnpm build

FROM caddy:2-alpine AS runtime
COPY deploy/caddy/Caddyfile /etc/caddy/Caddyfile
COPY --from=build /app/dist /srv
EXPOSE 80

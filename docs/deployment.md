# Deployment

LangIndex deploys as a static site served by Caddy. The production target is
Stephen's Ubuntu VM at `langindex.dev`.

Do not deploy, change DNS, or modify production host configuration without
Stephen's explicit approval.

## Production Shape

- Astro builds static files into `dist/`.
- Pagefind indexes the built site.
- The Docker image copies `dist/` into a Caddy runtime image.
- Caddy serves the static files for the configured site address.
- No production secrets are required for v1.

## Local Image Build

```sh
docker build -t langindex:local .
```

## Local Container Verification

```sh
docker run --rm -p 8080:80 -e SITE_ADDRESS=:80 langindex:local
```

Then check:

```sh
curl -I http://localhost:8080/
curl -I http://localhost:8080/languages/
```

## Compose

The default Compose file is local-safe and serves the site on host port `8080`:

```sh
docker compose up --build
```

For production, set `SITE_ADDRESS=langindex.dev` and publish ports `80` and
`443` from the Caddy container through the VM firewall and reverse-proxy plan
Stephen approves.

## Caddy Validation

If Caddy is installed locally:

```sh
SITE_ADDRESS=:80 caddy validate --config deploy/caddy/Caddyfile
```

Without a local Caddy binary, validate with Docker:

```sh
docker run --rm \
  -v "$PWD/deploy/caddy/Caddyfile:/etc/caddy/Caddyfile:ro" \
  -e SITE_ADDRESS=:80 \
  caddy:2-alpine \
  caddy validate --config /etc/caddy/Caddyfile
```

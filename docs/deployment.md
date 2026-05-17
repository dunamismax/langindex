# Deployment

LangIndex deploys as a Rust site service behind Caddy. The production target is
Stephen's Ubuntu VM at `langindex.dev`.

Do not deploy, change DNS, or modify production host configuration without
Stephen's explicit approval.

## Production Shape

- Cargo builds one release binary: `langindex-site`.
- The binary serves Axum routes, Leptos SSR HTML, embedded CSS/favicon assets,
  generated JSON/RSS/sitemap/robots outputs, and `/healthz`.
- Git-authored content is copied into the image at `/app/src/content` and
  validated on service startup.
- Docker Compose runs the service on loopback port `8090` by default.
- Host Caddy terminates TLS and reverse-proxies `langindex.dev` to
  `127.0.0.1:8090`.
- No production secrets are required.

## Release Build

```sh
just fmt
just check
just test
just build
```

The release binary is written to `target/release/langindex-site`.

## Local Image Build

```sh
docker build -t langindex:local .
```

## Local Container Verification

```sh
docker run --rm -p 8080:3000 langindex:local
```

Then check:

```sh
curl -fsS http://127.0.0.1:8080/healthz
curl -I http://127.0.0.1:8080/
curl -I http://127.0.0.1:8080/languages/
curl -I http://127.0.0.1:8080/rss.xml
```

## Compose

The default Compose file is local-safe and binds the service to loopback port
`8090`:

```sh
docker compose up --build -d
docker compose ps
curl -fsS http://127.0.0.1:8090/healthz
```

Use a different host port only if `8090` is already occupied:

```sh
LANGINDEX_HOST_PORT=8091 docker compose up --build -d
```

## Host Caddy

The reusable host snippet lives at `deploy/caddy/langindex.host.caddy`:

```caddy
www.langindex.dev {
	redir https://langindex.dev{uri} permanent
}

langindex.dev {
	encode zstd gzip
	reverse_proxy 127.0.0.1:8090

	header {
		X-Content-Type-Options "nosniff"
		Strict-Transport-Security "max-age=31536000; includeSubDomains"
		Referrer-Policy "strict-origin-when-cross-origin"
		X-XSS-Protection "0"
		Permissions-Policy "interest-cohort=()"
		-Server
	}
}
```

Validate before reloading Caddy on any host:

```sh
caddy validate --config /etc/caddy/Caddyfile
```

## Rollback

The Git repository is the source of truth. To roll back:

1. Check out the previous known-good commit.
2. Run `docker compose up --build -d`.
3. Confirm `docker compose ps` reports a healthy service.
4. Confirm `curl -fsS http://127.0.0.1:8090/healthz` returns `ok`.
5. Leave the host Caddy config unchanged unless the upstream port changed.

## Backup And Restore Notes

Host-level state worth backing up:

- `/etc/caddy/Caddyfile`
- Caddy-managed certificates and account state under Caddy's configured data
  directory
- the production checkout or deploy directory, if it contains uncommitted local
  changes

Restore from a clean VM by cloning the repository, checking out the intended
commit, running `docker compose up --build -d`, restoring the shared Caddyfile
site block, validating Caddy, and reloading the Caddy service.

## Local Caddy Validation

If Caddy is installed locally:

```sh
SITE_ADDRESS=localhost:8080 LANGINDEX_UPSTREAM=127.0.0.1:8090 \
  caddy validate --config deploy/caddy/Caddyfile
```

Without a local Caddy binary, validate with Docker:

```sh
docker run --rm \
  -v "$PWD/deploy/caddy/Caddyfile:/etc/caddy/Caddyfile:ro" \
  -e SITE_ADDRESS=localhost:8080 \
  -e LANGINDEX_UPSTREAM=127.0.0.1:8090 \
  caddy:2-alpine \
  caddy validate --config /etc/caddy/Caddyfile
```

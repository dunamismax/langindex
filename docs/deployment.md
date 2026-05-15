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

The default Compose file is local-safe and serves the site on loopback port
`8090`:

```sh
docker compose up --build
```

For production on Stephen's shared Caddy VM, keep the container bound to
loopback and let the host Caddyfile terminate TLS. The reusable host snippet
lives at `deploy/caddy/langindex.host.caddy`:

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

Use a different `LANGINDEX_HOST_PORT` only if `8090` is already occupied.

## Backup And Restore Notes

The Git repository is the source of truth for site source, content, and
deployment wiring. The production container is rebuildable from a clean
checkout and does not require runtime secrets.

Host-level state worth backing up:

- `/etc/caddy/Caddyfile`
- Caddy-managed certificates and account state under Caddy's configured data
  directory
- the production checkout or deploy directory, if it contains uncommitted local
  changes

Restore from a clean VM by cloning the repository, checking out the intended
commit, running `docker compose up --build -d`, restoring the shared Caddyfile
site block, validating Caddy, and reloading the Caddy service.

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

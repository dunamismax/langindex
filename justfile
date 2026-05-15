set shell := ["sh", "-cu"]

fmt:
  pnpm fmt

check:
  pnpm check

test:
  pnpm test

test-smoke:
  pnpm test:smoke

validate-sources:
  pnpm validate:sources

check-links-internal:
  pnpm check:links:internal

check-links-external:
  pnpm check:links:external

build:
  pnpm build

dev:
  pnpm dev

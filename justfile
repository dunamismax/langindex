set shell := ["sh", "-cu"]

fmt:
  pnpm fmt

check:
  pnpm check

test:
  pnpm test

validate-sources:
  pnpm validate:sources

build:
  pnpm build

dev:
  pnpm dev

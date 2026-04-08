#!/usr/bin/env bash
set -euo pipefail

PORT=2959

exec trunk serve --port "$PORT" "$@"

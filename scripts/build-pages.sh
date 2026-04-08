#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Building pages/ ==="
cd "$PROJECT_DIR"
mkdir -p pages
touch pages/.nojekyll
trunk build --release --public-url /web-sw-cor24-basic/
rsync -a --delete --exclude='.nojekyll' dist/ pages/

echo "=== Done ==="
echo "Pages built in: $PROJECT_DIR/pages/"
echo "To deploy: git add pages/ && git commit && git push"

#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Building pages/ ==="
cd "$PROJECT_DIR"

# Take the same exclusive dist/ lock that scripts/serve.sh uses so this
# build cannot race a running dev server (which would corrupt dist/.stage
# mid-pipeline and produce empty/SRI-blocked artifacts).
mkdir -p target
LOCK="$PROJECT_DIR/target/.trunk-dist.lock"
if ! mkdir "$LOCK" 2>/dev/null; then
  HOLDER="$(cat "$LOCK/pid" 2>/dev/null || echo unknown)"
  if [ "$HOLDER" != "unknown" ] && ! kill -0 "$HOLDER" 2>/dev/null; then
    echo "build-pages.sh: removing stale lock from pid $HOLDER" >&2
    rm -rf "$LOCK"
    mkdir "$LOCK"
  else
    echo "build-pages.sh: another trunk process (pid $HOLDER) holds $LOCK — refusing to share dist/" >&2
    exit 1
  fi
fi
echo $$ > "$LOCK/pid"
trap 'rm -rf "$LOCK"' EXIT INT TERM

mkdir -p pages
touch pages/.nojekyll
trunk build --release --public-url /web-sw-cor24-basic/
rsync -a --delete --exclude='.nojekyll' dist/ pages/

echo "=== Done ==="
echo "Pages built in: $PROJECT_DIR/pages/"
echo "To deploy: git add pages/ && git commit && git push"

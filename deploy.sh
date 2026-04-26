#!/usr/bin/env bash
# Idempotent deploy script for serve.uber.space/ewn/.
# Run from the repo root, on the serve host, as the `serve` user.
#
# Steps:
#   1. Compile the Rust release binary (mako-cli).
#   2. Install Bun deps and build the React frontend with BASE_PATH=/ewn/.
#   3. Sync the dist/ output to the Apache document root for /ewn/.
#   4. Initialise the markt/ directory if it does not yet exist.
#   5. Reload + restart the systemd user unit for the API server.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

WEB_ROOT="/var/www/virtual/serve/html/ewn"
SERVICE="ewn.service"
SYSTEMD_USER_DIR="$HOME/.config/systemd/user"

echo "==> 1/5  cargo build --release --bin mako-cli"
cargo build --release --bin mako-cli

echo "==> 2/5  bun install + vite build (BASE_PATH=/ewn/)"
cd mako-ui
bun install
BASE_PATH=/ewn/ bun run build
cd "$REPO_ROOT"

echo "==> 3/5  rsync mako-ui/dist/ → $WEB_ROOT/"
# Guard: refuse to delete-sync into a symlink (would wipe the shared html/ root).
if [ -L "$WEB_ROOT" ]; then
	echo "ERROR: $WEB_ROOT is a symlink. Refusing to rsync --delete." >&2
	exit 1
fi
mkdir -p "$WEB_ROOT"
rsync -a --delete mako-ui/dist/ "$WEB_ROOT/"

# SPA fallback so deep links like /ewn/markt/lieferant resolve to index.html.
cat > "$WEB_ROOT/.htaccess" <<'EOF'
RewriteEngine On
RewriteBase /ewn/
RewriteRule ^index\.html$ - [L]
RewriteCond %{REQUEST_FILENAME} !-f
RewriteCond %{REQUEST_FILENAME} !-d
RewriteCond %{REQUEST_URI} !^/ewn/api
RewriteRule . /ewn/index.html [L]
EOF

echo "==> 4/5  initialise markt/ if missing"
if [ ! -d mako-ui/markt ]; then
	./target/release/mako-cli init mako-ui/markt
fi

echo "==> 5/5  install + reload systemd unit, restart $SERVICE"
mkdir -p "$SYSTEMD_USER_DIR"
# Render the unit with the absolute repo path baked in.
sed "s|@REPO_ROOT@|$REPO_ROOT|g" ewn.service.in > "$SYSTEMD_USER_DIR/$SERVICE"
systemctl --user daemon-reload
systemctl --user enable "$SERVICE" >/dev/null
systemctl --user restart "$SERVICE"
sleep 1
systemctl --user status "$SERVICE" --no-pager | head -10

echo
echo "Done. Visit https://serve.uber.space/ewn/"

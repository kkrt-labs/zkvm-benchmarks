#!/usr/bin/env bash
set -euo pipefail

echo "Setting up keth (uv project) for Stwo proving..."

# Defaults (can be overridden via env)
: "${KETH_REPO:=https://github.com/kkrt-labs/keth}"
: "${KETH_DIR:=keth}"

# Basic tool checks
if ! command -v git >/dev/null 2>&1; then
	echo "git is required but not found. Please install git." >&2
	exit 1
fi

if ! command -v uv >/dev/null 2>&1; then
	echo "uv is required but not found. Install with:"
	echo "  curl -LsSf https://astral.sh/uv/install.sh | sh" >&2
	exit 1
fi

# Clone or update keth
if [ ! -d "$KETH_DIR/.git" ]; then
	echo "Cloning keth into '$KETH_DIR'..."
	git clone "$KETH_REPO" "$KETH_DIR"
else
	echo "keth already present in '$KETH_DIR'. Pulling latest..."
	git -C "$KETH_DIR" pull --ff-only || {
		echo "Warning: Could not pull latest keth. Continuing with existing checkout." >&2
	}
fi

# Sync uv environment inside keth
echo "Installing keth dependencies with uv..."
(
	cd "$KETH_DIR"
	# This creates/updates .venv based on pyproject and lockfile
	uv sync
)

echo "keth setup complete. You can now run:"
echo "  (cd $KETH_DIR && uv run compile <file.cairo> --proof-mode --output-path <out.json>)"
echo "  (cd $KETH_DIR && uv run prove-cairo --compiled-program <out.json> --arguments-file <input.json>)"

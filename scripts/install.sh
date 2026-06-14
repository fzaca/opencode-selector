#!/usr/bin/env bash
set -euo pipefail

# opencode-selector installer
# Usage: curl -fsSL https://raw.githubusercontent.com/fzaca/opencode-selector/master/scripts/install.sh | bash

REPO="fzaca/opencode-selector"
BIN_NAME="opcs"

# --- Detect OS/Arch ---
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux) ;;
  *)
    echo "Unsupported OS: $OS (Linux only)"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64)  ARCH_SUFFIX="x86_64-linux" ;;
  aarch64|arm64) ARCH_SUFFIX="aarch64-linux" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

# --- Resolve install dir ---
if [ -w /usr/local/bin ]; then
  INSTALL_DIR="/usr/local/bin"
elif [ -w "$HOME/.local/bin" ]; then
  INSTALL_DIR="$HOME/.local/bin"
else
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

# Ensure install dir is on PATH
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    echo "Warning: $INSTALL_DIR is not in your PATH."
    echo "Add it by running:  export PATH=\"\$PATH:$INSTALL_DIR\""
    echo "You can add that line to ~/.bashrc or ~/.zshrc."
    ;;
esac

# --- Fetch latest release ---
echo "Fetching latest release from $REPO..."
LATEST_URL="https://api.github.com/repos/$REPO/releases/latest"

if ! command -v curl &>/dev/null; then
  echo "Error: curl is required. Install it first."
  exit 1
fi

RELEASE_JSON="$(curl -fsSL "$LATEST_URL")"
TAG="$(echo "$RELEASE_JSON" | grep '"tag_name"' | cut -d'"' -f4)"
ASSET_NAME="opcs-${ARCH_SUFFIX}"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$TAG/$ASSET_NAME"

echo "Found version: $TAG"
echo "Downloading $ASSET_NAME..."

curl -fsSL "$DOWNLOAD_URL" -o "/tmp/$BIN_NAME"
chmod +x "/tmp/$BIN_NAME"
mv "/tmp/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"

echo ""
echo "✓ opencode-selector $TAG installed to $INSTALL_DIR/$BIN_NAME"
echo ""
echo "Run 'opcs' to start the session selector."
echo "Run 'opcs upgrade' to update to the latest version."

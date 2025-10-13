#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

# --- Configuration ---

# Enables or disables dry-run mode (no real publishing)
DRY_RUN=false

# Check if first argument is --dry-run
if [ "$1" == "--dry-run" ]; then
  DRY_RUN=true
  echo -e "\033[33m[INFO]\033[0m Running in DRY-RUN mode (no crates will actually be published)"
fi

# Ordered list of crates to publish
PUBLISH_ORDER=(
  "unen_app"
  "unen_event_derive"
  "unen_event"
  "unen_logging"
  "unen_net"
  "unen_window"
  "unen_render"
  "unen_winit"
  "unen"
)


WORKSPACE_ROOT=$(pwd)

# --- Functions ---

# Publishes a single crate in order
publish_crate() {
  local crate_name=$1
  local crate_path="$WORKSPACE_ROOT/crates/$crate_name"

  echo -e "\n\033[1;34m=== Processing crate: $crate_name ===\033[0m"

  # Ensure crate directory exists
  if [ ! -d "$crate_path" ]; then
    echo -e "\033[31m[ERROR]\033[0m Crate directory not found: $crate_path"
    exit 1
  fi

  cd "$crate_path"

  # Run tests before publishing
  echo -e "\033[36m[STEP]\033[0m Running tests for $crate_name..."
  cargo test --release

  # Publish (dry-run or real)
  if [ "$DRY_RUN" = true ]; then
    echo -e "\033[33m[DRY-RUN]\033[0m Simulating publish for $crate_name..."
    cargo publish --dry-run
  else
    echo -e "\033[32m[PUBLISH]\033[0m Publishing $crate_name to crates.io..."
    cargo publish

    echo -e "\033[36m[WAIT]\033[0m Waiting 30s for crates.io propagation..."
    sleep 30
  fi

  # Return to workspace root
  cd "$WORKSPACE_ROOT"

  echo -e "\033[32m[DONE]\033[0m Finished processing $crate_name"
}

# --- Main Execution ---

echo -e "\033[1;35m=======================================\033[0m"
echo -e "\033[1;35m Starting UnnamedEngine publication... \033[0m"
echo -e "\033[1;35m=======================================\033[0m\n"

echo -e "\033[36m[ORDER]\033[0m Crate publish order: ${PUBLISH_ORDER[*]}"

# Loop through and publish each crate in order
for crate in "${PUBLISH_ORDER[@]}"; do
  publish_crate "$crate"
done

echo -e "\n\033[1;32m✅ Successfully published all UnnamedEngine crates!\033[0m"

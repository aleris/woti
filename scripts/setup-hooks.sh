#!/bin/bash

# Setup script to configure git hooks for this repository
# Run this once after cloning the repo

set -e

REPO_ROOT=$(git rev-parse --show-toplevel)

echo "Configuring git to use .githooks directory..."
git config core.hooksPath .githooks

echo "Making hooks executable..."
chmod +x "$REPO_ROOT/.githooks/"*

echo "Done! Git hooks are now configured."
echo ""
echo "Hooks enabled:"
echo "  - post-commit: Auto-increments version on each commit"
echo "    - Minor bump by default (0.1.0 → 0.2.0)"
echo "    - Patch bump if commit starts with 'fix' or '[fix]' (0.1.0 → 0.1.1)"

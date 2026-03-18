#!/bin/bash
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
TAG="v${VERSION}"

echo -e "${YELLOW}🚀 Releasing woti ${TAG}${NC}"

# Pre-flight checks
BRANCH=$(git branch --show-current)
[ "$BRANCH" != "main" ] && echo -e "${RED}Error: Must be on main branch${NC}" && exit 1
[ -n "$(git status --porcelain)" ] && echo -e "${RED}Error: Uncommitted changes${NC}" && exit 1
git rev-parse "$TAG" >/dev/null 2>&1 && echo -e "${RED}Error: Tag ${TAG} already exists${NC}" && exit 1

# Confirm
echo "This will build, tag ${TAG}, and publish to GitHub."
read -p "Continue? (Y/n) " -n 1 -r
echo
[[ $REPLY =~ ^[Nn]$ ]] && exit 1

# Build
echo -e "${YELLOW}🔨 Building release...${NC}"
cargo build --release

BINARY="target/release/woti"
[ ! -f "$BINARY" ] && echo -e "${RED}Error: Binary not found at ${BINARY}${NC}" && exit 1
echo -e "${GREEN}✓ Built: ${BINARY}${NC}"

# Create tarball
ARCHIVE="woti-${TAG}-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).tar.gz"
tar -czf "$ARCHIVE" -C target/release woti
echo -e "${GREEN}✓ Packaged: ${ARCHIVE}${NC}"

# Tag and release
git tag -a "$TAG" -m "Release ${TAG}"
git push origin "$TAG"

gh release create "$TAG" \
    --title "Release ${TAG}" \
    --generate-notes \
    "$ARCHIVE"

rm -f "$ARCHIVE"

echo -e "${GREEN}✅ Released ${TAG}${NC}"

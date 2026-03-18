#!/bin/bash
set -e

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

echo "This will tag ${TAG} and push to GitHub. CI will build and publish the release."
read -p "Continue? (Y/n) " -n 1 -r
echo
[[ $REPLY =~ ^[Nn]$ ]] && exit 1

git tag -a "$TAG" -m "Release ${TAG}"
git push origin "$TAG"

echo -e "${GREEN}✅ Tag ${TAG} pushed — CI will build and publish the release.${NC}"

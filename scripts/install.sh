#!/bin/sh
set -e

REPO="aleris/woti"
BINARY="woti"

usage() {
    printf "Usage: install.sh [--version VERSION]\n"
    printf "\n"
    printf "Install %s from GitHub releases.\n" "$BINARY"
    printf "\n"
    printf "Options:\n"
    printf "  --version VERSION  Install a specific version (e.g., v0.4.0)\n"
    printf "  --help             Show this help message\n"
    exit 0
}

error() {
    printf "\033[0;31mError: %s\033[0m\n" "$1" >&2
    exit 1
}

info() {
    printf "\033[0;32m%s\033[0m\n" "$1"
}

VERSION=""
while [ $# -gt 0 ]; do
    case "$1" in
        --version)
            shift
            VERSION="$1"
            ;;
        --help)
            usage
            ;;
        *)
            error "Unknown option: $1"
            ;;
    esac
    shift
done

command -v curl >/dev/null 2>&1 || error "curl is required but not installed."
command -v tar >/dev/null 2>&1 || error "tar is required but not installed."

detect_platform() {
    local os arch

    os="$(uname -s)"
    case "$os" in
        Linux)  os="linux" ;;
        Darwin) os="darwin" ;;
        *)      error "Unsupported OS: $os. Supported: Linux, macOS" ;;
    esac

    arch="$(uname -m)"
    case "$arch" in
        x86_64|amd64)  arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)             error "Unsupported architecture: $arch. Supported: x86_64, aarch64/arm64" ;;
    esac

    echo "${os} ${arch}"
}

fetch_latest_version() {
    local url response tag
    url="https://api.github.com/repos/${REPO}/releases/latest"
    response="$(curl -fsSL "$url" 2>/dev/null)" || error "Failed to fetch latest release from ${url}"
    tag="$(echo "$response" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/')"
    [ -z "$tag" ] && error "Could not parse latest release tag from GitHub API"
    echo "$tag"
}

platform="$(detect_platform)"
os="$(echo "$platform" | cut -d' ' -f1)"
arch="$(echo "$platform" | cut -d' ' -f2)"

if [ -z "$VERSION" ]; then
    printf "Fetching latest version... "
    VERSION="$(fetch_latest_version)"
    printf "%s\n" "$VERSION"
fi

archive="${BINARY}-${VERSION}-${os}-${arch}.tar.gz"
url="https://github.com/${REPO}/releases/download/${VERSION}/${archive}"

printf "Downloading %s...\n" "$archive"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

curl -fSL --progress-bar "$url" -o "${tmpdir}/${archive}" || error "Download failed: ${url}\nCheck that version ${VERSION} exists and has a binary for ${os}-${arch}."

tar -xzf "${tmpdir}/${archive}" -C "$tmpdir" || error "Failed to extract archive"
[ -f "${tmpdir}/${BINARY}" ] || error "Binary not found in archive"
chmod +x "${tmpdir}/${BINARY}"

if [ "$(id -u)" = "0" ]; then
    install_dir="/usr/local/bin"
else
    install_dir="${HOME}/.local/bin"
fi
mkdir -p "$install_dir"

mv "${tmpdir}/${BINARY}" "${install_dir}/${BINARY}"

info "Installed ${BINARY} ${VERSION} to ${install_dir}/${BINARY}"

case ":${PATH}:" in
    *":${install_dir}:"*) ;;
    *)
        printf "\n"
        printf "\033[1;33mNote:\033[0m %s is not in your PATH.\n" "$install_dir"
        printf "Add it by appending this to your shell profile:\n"
        printf "\n"
        printf "  export PATH=\"%s:\$PATH\"\n" "$install_dir"
        printf "\n"
        ;;
esac

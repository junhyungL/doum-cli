#!/bin/bash
# Doum CLI installation script for Linux and macOS

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="junhyungL/doum-cli"
BINARY_NAME="doum"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
detect_platform() {
    local os="$(uname -s)"
    local arch="$(uname -m)"
    
    case "$os" in
        Linux*)
            OS="linux"
            ;;
        Darwin*)
            OS="macos"
            ;;
        *)
            echo -e "${RED}Unsupported operating system: $os${NC}"
            exit 1
            ;;
    esac
    
    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            echo -e "${RED}Unsupported architecture: $arch${NC}"
            exit 1
            ;;
    esac
    
    # Special handling for macOS
    if [ "$OS" = "macos" ]; then
        PLATFORM="macos-${ARCH}"
    else
        PLATFORM="linux-${ARCH}"
    fi
}

# Get latest release version
get_latest_version() {
    echo -e "${YELLOW}Fetching latest version...${NC}"
    VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        echo -e "${RED}Failed to fetch latest version${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}Latest version: ${VERSION}${NC}"
}

# Download and install
install_doum() {
    local asset_name="${BINARY_NAME}-${PLATFORM}"
    local download_url="https://github.com/${REPO}/releases/download/${VERSION}/${asset_name}.tar.gz"
    local tmp_dir=$(mktemp -d)
    
    echo -e "${YELLOW}Downloading ${asset_name}...${NC}"
    
    if ! curl -L -o "${tmp_dir}/${asset_name}.tar.gz" "$download_url"; then
        echo -e "${RED}Failed to download ${asset_name}${NC}"
        rm -rf "$tmp_dir"
        exit 1
    fi
    
    echo -e "${YELLOW}Extracting...${NC}"
    tar -xzf "${tmp_dir}/${asset_name}.tar.gz" -C "$tmp_dir"
    
    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
    
    echo -e "${YELLOW}Installing to ${INSTALL_DIR}...${NC}"
    mv "${tmp_dir}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    
    # Cleanup
    rm -rf "$tmp_dir"
    
    echo -e "${GREEN}Doum CLI installed successfully!${NC}"
}

# Check if install directory is in PATH
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo ""
        echo -e "${YELLOW}Warning: ${INSTALL_DIR} is not in your PATH${NC}"
        echo ""
        echo "Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo ""
        echo -e "${GREEN}export PATH=\"\$PATH:${INSTALL_DIR}\"${NC}"
        echo ""
    fi
}

# Verify installation
verify_installation() {
    if command -v "$BINARY_NAME" &> /dev/null; then
        echo ""
        echo -e "${GREEN}Verification successful!${NC}"
        echo "Run '${BINARY_NAME} --help' to get started."
    else
        echo ""
        echo -e "${YELLOW}Installation complete but '${BINARY_NAME}' command not found in PATH.${NC}"
        echo "You may need to restart your shell or add ${INSTALL_DIR} to your PATH."
    fi
}

# Main
main() {
    echo ""
    echo "====================================="
    echo "   Doum CLI Installation Script"
    echo "====================================="
    echo ""
    
    detect_platform
    echo -e "Platform: ${GREEN}${OS} (${ARCH})${NC}"
    echo ""
    
    get_latest_version
    echo ""
    
    install_doum
    echo ""
    
    check_path
    verify_installation

    echo -e "${GREEN}Done!${NC}"
}

main

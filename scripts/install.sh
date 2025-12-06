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

# Detect OS
detect_os() {
    local os="$(uname -s)"
    
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
}

# Get architecture
get_architecture() {
    local arch="$(uname -m)"
    
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
    local platform="${OS}-${ARCH}"
    local asset_name="${BINARY_NAME}-${platform}"
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

# Add to PATH
add_to_path() {
    if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
        echo -e "${GREEN}${INSTALL_DIR} is already in PATH${NC}"
        return
    fi
    
    echo -e "${YELLOW}Adding ${INSTALL_DIR} to PATH...${NC}"
    
    # Detect shell and add to appropriate config file
    local shell_config=""
    
    if [ -n "$BASH_VERSION" ]; then
        # macOS prefers .bash_profile, Linux prefers .bashrc
        if [ "$OS" = "macos" ]; then
            if [ -f "$HOME/.bash_profile" ]; then
                shell_config="$HOME/.bash_profile"
            elif [ -f "$HOME/.bashrc" ]; then
                shell_config="$HOME/.bashrc"
            else
                shell_config="$HOME/.bash_profile"
                touch "$shell_config"
            fi
        else
            if [ -f "$HOME/.bashrc" ]; then
                shell_config="$HOME/.bashrc"
            elif [ -f "$HOME/.bash_profile" ]; then
                shell_config="$HOME/.bash_profile"
            fi
        fi
    elif [ -n "$ZSH_VERSION" ] || [[ "$SHELL" == */zsh ]]; then
        if [ -f "$HOME/.zshrc" ]; then
            shell_config="$HOME/.zshrc"
        else
            shell_config="$HOME/.zshrc"
            touch "$shell_config"
        fi
    else
        # Try to detect shell from SHELL environment variable
        case "$SHELL" in
            */bash)
                if [ "$OS" = "macos" ]; then
                    if [ -f "$HOME/.bash_profile" ]; then
                        shell_config="$HOME/.bash_profile"
                    else
                        shell_config="$HOME/.bash_profile"
                        touch "$shell_config"
                    fi
                else
                    if [ -f "$HOME/.bashrc" ]; then
                        shell_config="$HOME/.bashrc"
                    elif [ -f "$HOME/.bash_profile" ]; then
                        shell_config="$HOME/.bash_profile"
                    fi
                fi
                ;;
            */zsh)
                if [ -f "$HOME/.zshrc" ]; then
                    shell_config="$HOME/.zshrc"
                else
                    shell_config="$HOME/.zshrc"
                    touch "$shell_config"
                fi
                ;;
        esac
    fi
    
    if [ -n "$shell_config" ]; then
        # Check if PATH export already exists in the file
        if ! grep -q "export PATH=\"\$PATH:${INSTALL_DIR}\"" "$shell_config" 2>/dev/null; then
            echo "" >> "$shell_config"
            echo "# Added by Doum CLI installer" >> "$shell_config"
            echo "export PATH=\"\$PATH:${INSTALL_DIR}\"" >> "$shell_config"
            echo -e "${GREEN}Added to ${shell_config}${NC}"
            echo -e "${YELLOW}Please restart your terminal or run: source ${shell_config}${NC}"
        else
            echo -e "${GREEN}PATH entry already exists in ${shell_config}${NC}"
        fi
    else
        echo -e "${YELLOW}Could not detect shell configuration file.${NC}"
        echo ""
        echo "Please manually add the following line to your shell profile:"
        echo -e "${GREEN}export PATH=\"\$PATH:${INSTALL_DIR}\"${NC}"
        echo ""
    fi
}

# Verify installation
verify_installation() {
    local doum_path="${INSTALL_DIR}/${BINARY_NAME}"
    
    if [ -f "$doum_path" ] && [ -x "$doum_path" ]; then
        echo -e "${GREEN}Verification successful!${NC}"
        echo -e "${YELLOW}Run 'doum --help' to get started.${NC}"
        echo -e "${YELLOW}Note: You may need to restart your terminal if 'doum' command is not found.${NC}"
    else
        echo -e "${RED}Installation verification failed!${NC}"
        exit 1
    fi
}

# Main
main() {
    echo ""
    echo "====================================="
    echo "   Doum CLI Installation Script"
    echo "====================================="
    echo ""
    
    detect_os
    get_architecture
    echo -e "Platform: ${GREEN}${OS}${NC}"
    echo -e "Architecture: ${GREEN}${ARCH}${NC}"
    echo ""
    
    get_latest_version
    echo ""
    
    install_doum
    echo ""
    
    add_to_path
    echo ""
    
    verify_installation
    echo ""
    
    echo -e "${GREEN}Done!${NC}"
}

main

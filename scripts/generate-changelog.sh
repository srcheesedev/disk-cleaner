#!/bin/bash

# Changelog Generation Script
# Uses git-cliff to generate and update CHANGELOG.md

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}📝 Changelog Generator${NC}"
echo "======================"

# Check if git-cliff is installed
if ! command -v git-cliff &> /dev/null; then
    echo -e "${RED}❌ git-cliff not found${NC}"
    echo -e "${YELLOW}💡 Installing git-cliff...${NC}"
    
    # Try to install git-cliff
    if command -v cargo &> /dev/null; then
        cargo install git-cliff
    elif command -v brew &> /dev/null; then
        brew install git-cliff
    else
        echo -e "${RED}❌ Please install git-cliff manually:${NC}"
        echo "   cargo install git-cliff"
        echo "   # or"
        echo "   brew install git-cliff"
        exit 1
    fi
fi

# Backup current changelog
if [ -f "CHANGELOG.md" ]; then
    echo -e "${YELLOW}📋 Backing up current CHANGELOG.md...${NC}"
    cp CHANGELOG.md CHANGELOG.md.backup
fi

# Generate changelog
echo -e "${BLUE}🔄 Generating changelog...${NC}"

case "${1:-full}" in
    "latest")
        echo -e "${GREEN}📝 Generating latest release changelog...${NC}"
        git cliff --latest --output CHANGELOG.md
        ;;
    "unreleased")
        echo -e "${GREEN}📝 Generating unreleased changes...${NC}"
        git cliff --unreleased --output CHANGELOG.md
        ;;
    "full"|*)
        echo -e "${GREEN}📝 Generating full changelog...${NC}"
        git cliff --output CHANGELOG.md
        ;;
esac

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Changelog generated successfully!${NC}"
    echo -e "${BLUE}📄 Preview of CHANGELOG.md:${NC}"
    echo "===================="
    head -n 20 CHANGELOG.md
    echo "..."
    echo "===================="
    
    # Clean up backup if successful
    if [ -f "CHANGELOG.md.backup" ]; then
        rm CHANGELOG.md.backup
    fi
else
    echo -e "${RED}❌ Failed to generate changelog${NC}"
    # Restore backup if it exists
    if [ -f "CHANGELOG.md.backup" ]; then
        echo -e "${YELLOW}🔄 Restoring backup...${NC}"
        mv CHANGELOG.md.backup CHANGELOG.md
    fi
    exit 1
fi

echo -e "${GREEN}🎉 Done!${NC}"
#!/bin/bash
set -e

# ANSI color codes for better output formatting
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print banner
echo -e "${BLUE}=====================================${NC}"
echo -e "${BLUE}   Uniswap V4 Changelog Generator    ${NC}"
echo -e "${BLUE}=====================================${NC}"
echo

# Function to check dependencies
check_dependency() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${YELLOW}Warning: $1 is not installed.${NC}"
        return 1
    fi
    return 0
}

# Function to install git-cliff
install_git_cliff() {
    echo -e "${YELLOW}Attempting to install git-cliff...${NC}"
    
    # Check if cargo is available
    if ! check_dependency "cargo"; then
        echo -e "${RED}Error: Rust's Cargo is not installed. Please install Rust first from https://rustup.rs/${NC}"
        exit 1
    fi
    
    # Install git-cliff
    echo -e "${BLUE}Installing git-cliff via cargo...${NC}"
    cargo install git-cliff
    
    # Check if installation was successful
    if [ $? -ne 0 ]; then
        echo -e "${RED}Failed to install git-cliff. Please install it manually using 'cargo install git-cliff'.${NC}"
        echo -e "${RED}For more information, visit: https://github.com/orhun/git-cliff${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}git-cliff installed successfully!${NC}"
}

# Check if git-cliff is installed
if ! check_dependency "git-cliff"; then
    echo -e "${YELLOW}git-cliff is required to generate the changelog.${NC}"
    read -p "Do you want to install git-cliff now? [Y/n] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
        install_git_cliff
    else
        echo -e "${RED}Aborted. Please install git-cliff manually.${NC}"
        exit 1
    fi
fi

# Check if cliff.toml exists
if [ ! -f "cliff.toml" ]; then
    echo -e "${RED}Error: cliff.toml configuration file not found.${NC}"
    echo -e "${YELLOW}Creating a default cliff.toml file...${NC}"
    
    cat > cliff.toml << 'EOF'
# Configuration file for git-cliff
# See https://github.com/orhun/git-cliff for more details

[changelog]
# Changelog header
header = """
# Changelog

All notable changes to Uniswap V4 Core will be documented in this file.
"""
# Template for the changelog body
body = """
{% if version %}\
## [{{ version }}]{% if previous_version %} - _compared to [{{ previous_version }}]({{ repository_url }}/compare/{{ previous_version }}...{{ version }})_{% endif %} - {{ timestamp | date(format="%Y-%m-%d") }}
{% else %}\
## [Unreleased]
{% endif %}\
{% for group, commits in commits | group_by(attribute="group") %}
### {{ group | upper_first }}
{% for commit in commits %}
- {% if commit.breaking %}**BREAKING**: {% endif %}{{ commit.message | upper_first }}{% if commit.id %} ([{{ commit.id | truncate(length=7, end="") }}]({{ repository_url }}/commit/{{ commit.id }})){% endif %}
{% endfor %}
{% endfor %}\
"""
# Remove the leading and trailing whitespace from the template
trim = true
# Changelog footer
footer = """
<!-- generated by git-cliff -->
"""

[git]
# Allow using regular expressions to group commits
conventional_commits = true
# Regex for parsing and grouping commits
commit_parsers = [
  { message = "^feat", group = "Features" },
  { message = "^fix", group = "Bug Fixes" },
  { message = "^doc", group = "Documentation" },
  { message = "^perf", group = "Performance" },
  { message = "^refactor", group = "Refactor" },
  { message = "^style", group = "Styling" },
  { message = "^test", group = "Testing" },
  { message = "^chore\\(release\\): prepare for", skip = true },
  { message = "^chore", group = "Miscellaneous Tasks" },
  { body = ".*security", group = "Security" },
]
# Filter out the commits that are not matched by commit parsers
filter_commits = false
# Sort the tags chronologically
date_order = false
# Sort the commits inside sections by oldest/newest order
sort_commits = "oldest"
EOF

    echo -e "${GREEN}Default cliff.toml created.${NC}"
fi

echo -e "${BLUE}Updating .changelog file...${NC}"

# Generate changelog
git-cliff --config cliff.toml --output .changelog

# Check if generation was successful
if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to generate changelog.${NC}"
    exit 1
fi

echo -e "${GREEN}Changelog updated successfully!${NC}"
echo -e "${BLUE}To view the changelog, use '${YELLOW}cat .changelog${BLUE}' or open it in your editor.${NC}"

# Optional: Show a preview of the changelog
read -p "Do you want to see a preview of the changelog? [Y/n] " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
    echo -e "${BLUE}====== Changelog Preview ======${NC}"
    echo
    head -n 20 .changelog
    echo -e "${YELLOW}...(truncated)...${NC}"
    echo
    echo -e "${BLUE}To see the full changelog, use '${YELLOW}cat .changelog${BLUE}'${NC}"
fi

exit 0 
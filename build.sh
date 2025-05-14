#!/bin/bash
# Script to build the pigment project

# Check if colors.rs exists, if not, regenerate it
if [ ! -f "generated/colors.rs" ]; then
    echo "Generating colors.rs file..."
    PIGMENT_REGEN=1 cargo build
else
    # Normal build
    cargo build
fi

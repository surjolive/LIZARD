#!/bin/bash
set -e

INSTALL_DIR="$HOME/.local/bin"

echo "LIZARD Uninstaller"
echo "=================="
echo ""

# Remove executables
if [ -d "$INSTALL_DIR" ]; then
    if [ -f "$INSTALL_DIR/lz" ] || [ -f "$INSTALL_DIR/lizard" ]; then
        echo "Removing LIZARD executables from $INSTALL_DIR..."
        rm -f "$INSTALL_DIR/lz" "$INSTALL_DIR/lizard"
        echo "✓ Removed LIZARD executables"
    else
        echo "⚠ LIZARD executables not found in $INSTALL_DIR"
    fi
else
    echo "⚠ Installation directory not found at $INSTALL_DIR"
fi

# Try to remove from PATH in shell configs
for shell_config in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
    if [ -f "$shell_config" ]; then
        if grep -q "LIZARD\|$INSTALL_DIR" "$shell_config" 2>/dev/null; then
            sed -i.bak "/LIZARD\|$INSTALL_DIR/d" "$shell_config"
            echo "✓ Removed LIZARD from $shell_config"
        fi
    fi
done

echo ""
echo "LIZARD has been successfully uninstalled."
echo "Please open a new terminal window for changes to take effect."

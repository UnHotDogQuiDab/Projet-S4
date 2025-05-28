#!/bin/bash
set -e

echo "🔧 Installing waves_found..."

INSTALL_DIR="/usr/local/share/waves_found"
BIN_PATH="/usr/local/bin/waves_found"


echo "📦 Building project in release mode..."
cargo build --release

echo "📁 Copying files to $INSTALL_DIR"
sudo mkdir -p "$INSTALL_DIR"
sudo cp -r src/test_files "$INSTALL_DIR"

echo "🔐 Setting permissions so all users can read and write test_files..."
sudo chmod -R a+rw "$INSTALL_DIR/test_files"

echo "🚀 Installing executable to $BIN_PATH"
sudo cp target/release/projet-s4 "$BIN_PATH"


echo "✅ Creating launcher script..."
sudo tee "$BIN_PATH" > /dev/null <<EOF
#!/bin/bash
exec "$INSTALL_DIR/projet-s4" "\$@"
EOF

sudo chmod +x "$BIN_PATH"

echo "✅ waves_found has been installed. You can now run it using: waves_found"

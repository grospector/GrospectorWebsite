#!/bin/bash

# Bitcoin Wealth Comparison - Development Setup Script

set -e

echo "🦀 Setting up Bitcoin Wealth Comparison development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust is installed"

# Check if wasm32-unknown-unknown target is added
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "📦 Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
else
    echo "✅ wasm32-unknown-unknown target is already installed"
fi

# Install Trunk if not already installed
if ! command -v trunk &> /dev/null; then
    echo "📦 Installing Trunk..."
    cargo install trunk
else
    echo "✅ Trunk is already installed"
fi

# Install wasm-bindgen-cli if not already installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "📦 Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
else
    echo "✅ wasm-bindgen-cli is already installed"
fi

# Build the project
echo "🔨 Building the project..."
cargo build

echo "🎉 Setup complete!"
echo ""
echo "To start the development server, run:"
echo "  trunk serve"
echo ""
echo "To build for production, run:"
echo "  trunk build --release"
echo ""
echo "The app will be available at: http://localhost:8080"
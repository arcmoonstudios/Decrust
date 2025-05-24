#!/bin/bash
# CI Check Script - Run the same checks as CI locally
# This helps catch formatting and lint issues before pushing

set -e

echo "🔍 Running CI checks locally..."

echo "📦 Checking if cargo is available..."
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust."
    exit 1
fi

echo "🎨 Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "❌ Code formatting check failed!"
    echo "💡 Run 'cargo fmt --all' to fix formatting issues."
    exit 1
fi

echo "📎 Running clippy lints..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy check failed!"
    echo "💡 Fix the clippy warnings above."
    exit 1
fi

echo "🔨 Building all targets..."
if ! cargo build --all-targets; then
    echo "❌ Build failed!"
    exit 1
fi

echo "🧪 Running all tests..."
if ! cargo test --all-targets; then
    echo "❌ Tests failed!"
    exit 1
fi

echo "📖 Checking documentation..."
if ! cargo doc --all-features --no-deps; then
    echo "❌ Documentation check failed!"
    exit 1
fi

echo "✅ All CI checks passed! Ready to push."

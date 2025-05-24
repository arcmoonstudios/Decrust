@echo off
REM CI Check Script - Run the same checks as CI locally
REM This helps catch formatting and lint issues before pushing

echo 🔍 Running CI checks locally...

echo 📦 Checking if cargo is available...
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ Cargo not found. Please install Rust.
    exit /b 1
)

echo 🎨 Checking code formatting...
cargo fmt --all -- --check
if %errorlevel% neq 0 (
    echo ❌ Code formatting check failed!
    echo 💡 Run 'cargo fmt --all' to fix formatting issues.
    exit /b 1
)

echo 📎 Running clippy lints...
cargo clippy --all-targets --all-features -- -D warnings
if %errorlevel% neq 0 (
    echo ❌ Clippy check failed!
    echo 💡 Fix the clippy warnings above.
    exit /b 1
)

echo 🔨 Building all targets...
cargo build --all-targets
if %errorlevel% neq 0 (
    echo ❌ Build failed!
    exit /b 1
)

echo 🧪 Running all tests...
cargo test --all-targets
if %errorlevel% neq 0 (
    echo ❌ Tests failed!
    exit /b 1
)

echo 📖 Checking documentation...
cargo doc --all-features --no-deps
if %errorlevel% neq 0 (
    echo ❌ Documentation check failed!
    exit /b 1
)

echo ✅ All CI checks passed! Ready to push.

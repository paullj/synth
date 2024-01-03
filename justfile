_default:
  @just --list
  
[no-exit-message]
check-setup:
    @echo "🔍 Checking dependencies..."
    @rustc --version > /dev/null && echo "✅ Rust is installed." || { echo "❌ Rust is not installed. Please install it from https://www.rust-lang.org/tools/install" >&2; exit 1; }
    @rustup target list --installed | grep thumbv6m-none-eabi > /dev/null && echo "✅ thumbv6m-none-eabi target is installed." || { echo "❌ thumbv6m-none-eabi target is not installed. Please install it using 'rustup target add thumbv6m-none-eabi'" >&2; exit 1; }
    @sdl2-config --version > /dev/null && echo "✅ SDL2 is installed." || { echo "❌ SDL2 is not installed. Please install it following the instructions on https://www.libsdl.org/download-2.0.php" >&2; exit 1; }

[no-exit-message]
build: 
    @just check-setup >/dev/null
    @echo "🛠️ Building..."
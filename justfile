# Subcommands for synth_app 
mod app 'synth-app/justfile'

set dotenv-load

_default:
  @just --list

# TODO: Sort this out
# Checks required dependencies on development machine
# [no-exit-message]
# check-setup:
#   @echo "🔍 Checking dependencies..."
#   @rustc --version > /dev/null && echo "✅ Rust is installed." || { echo "❌ Rust is not installed. Please install it from https://www.rust-lang.org/tools/install" >&2; exit 1; }
#   @sdl2-config --version > /dev/null && echo "✅ SDL2 is installed." || { echo "❌ SDL2 is not installed. Please install it following the instructions on https://www.libsdl.org/download-2.0.php" >&2; exit 1; }
#   @cargo-objcopy --version > /dev/null && echo "✅ cargo-objcopy is installed." || { echo "❌ cargo-objcopy is not installed. Please install it using 'cargo install cargo-binutils'" >&2; exit 1; }

# cargo install probe-rs --features cli
# I needed to install from git? so might need to do that too
# cargo install cross
# cargo install cargo-binutils

# TODO: Check to see if rustup targets and compoents need to be installed or if the toolchain and .cargo/config are enough
# @rustup target list --installed | grep thumbv6m-none-eabi > /dev/null && echo "✅ thumbv6m-none-eabi target is installed." || { echo "❌ thumbv6m-none-eabi target is not installed. Please install it using 'rustup target add thumbv6m-none-eabi'" >&2; exit 1; }
# @rustup component list --installed | grep llvm-tools > /dev/null && echo "✅ llvm-tools is installed." || { echo "❌ llvm-tools is not installed. Please install it using 'rustup component add llvm-tools'" >&2; exit 1; }

# [no-exit-message]
# build-firmware args="":
#   @echo "🔨 Building firmware for embedded..."
#   @just check-setup > /dev/null
#   @cd synth-firmware && cargo-objcopy --target thumbv6m-none-eabi --bin midi {{args}} -- -O ihex firmware.hex

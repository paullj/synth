# Synth

<!-- TODO: Rename to something cooler -->

> **Note**: This project is still in its early stages and is not ready for use. ie it doesn't do anything yet and this repo is fairly empty.

This repo is a monorepo composed of multiple projects that are used to make a synthesizer. The aim of the project is to learn embedded, lower level programming with Rust, and electronic engineering. It is very much a work in progress, I have no idea what I'm doing.

- [`synth-firmware`](https://github.com/paullj/synth/blob/main/synth-firmware/README.md): Firmware for custom board that sends MIDI messages over USB
- [`synth-hardware`](https://github.com/paullj/synth/blob/main/synth-hardware/README.md): Hardware design files for the MIDI keyboard

## Development

This project uses [just](https://github.com/casey/just) as a task runner. Once installed run `just` in the repo root to see the available tasks. You will also need to install [Rust](https://www.rust-lang.org/tools/install) and a few other dependencies, you can run `just check-setup` to check if you have everything installed.

# Component-C Compiler

## Development Roadmap

STAGE 1: `comcc` written in Rust, compiled by `rustc`

STAGE 2: `comcc` written in Component-C, compiled by Rust-based `comcc`

STAGE 3: `comcc` written in Component-C, compiled by Component-C-based `comcc`

## Targets

Target Quads: `architecture-kernel-os-abi`

### Stage 1 Plans

Architectures: x86_64, Aarch64

Kernels: Linux, NT, XNU, None

OSes: Linux, Windows, macOS, None

ABIs: GNU/MUSL, MSVC/GNU, Darwin, UEFI, None

Total List:
 - x86_64-linux-linux-gnu
 - x86_64-linux-linux-musl
 - x86_64-nt-windows-msvc
 - x86_64-nt-windows-gnu
 - x86_64-xnu-macos-darwin
 - x86_64-none-none-uefi
 - x86_64-none-none-none
 - aarch64-linux-linux-gnu
 - aarch64-linux-linux-musl
 - aarch:64-nt-windows-msvc
 - aarch64-nt-windows-gnu
 - aarch64-xnu-macos-darwin
 - aarch64-none-none-uefi
 - aarch64-none-none-none

## Rust Dependencies

Rust-base `comcc` uses Inkwell, to act as a safe interface with the LLVM backend, and `clap` for command line argument processing.

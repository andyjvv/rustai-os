#!/bin/bash
# build.sh - Script para compilar y ejecutar RustAI-OS

set -e

# Verificar dependencias
command -v rustup >/dev/null 2>&1 || { echo "Necesitas instalar rustup"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "Necesitas instalar cargo"; exit 1; }
command -v qemu-system-x86_64 >/dev/null 2>&1 || { echo "Necesitas instalar QEMU"; exit 1; }

echo "Configurando entorno Rust..."
rustup override set nightly
rustup component add rust-src --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly
cargo install bootimage

echo "Compilando RustAI-OS..."
cargo build --release

echo "Creando imagen booteable..."
cargo bootimage --release

echo "Iniciando QEMU..."
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rustai_os/release/bootimage-rustai-os.bin -m 128M -serial stdio -display sdl

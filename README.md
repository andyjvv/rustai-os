# RustAI-OS

Sistema operativo especializado en Inteligencia Artificial escrito en Rust.

## Características

- Núcleo minimalista diseñado para operaciones de IA
- Soporte para aceleración de hardware de tensores
- API para modelos de IA
- Red optimizada para comunicación de datos de IA
- Memoria gestionada para cargas de trabajo intensivas

## Requisitos

- Rust nightly
- QEMU para pruebas de emulación
- cargo-bootimage

## Compilación

```bash
cargo install cargo-bootimage
cargo bootimage

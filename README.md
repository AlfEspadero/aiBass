# aiBass

Sistema de IA empotrada de detección de notas a partir de sensores inerciales.
Trabajo de Fin de Grado. Alfonso Espadero García. Curso 2025/2026

## Descripción

En el presente proyecto se plantea aprovechar las vibraciones del mástil de un bajo para
determinar la nota que está sonando en cada momento. Para ello se plantea fijar un sensor de
medidas inerciales (IMU) en el instrumento, recopilando su información de salida para ser
procesada mediante una IA empotrada. A partir de esta clasificación se exploraran múltiples
aplicaciones potenciales, como por ejemplo la afinación o un captador MIDI.

## Objetivos

- Estudiar el posicionamiento del sensor y realizar una interfaz FW/SW
- Construir un dataset a partir de la recolección de muestras de las distintas
- Diseñar y entrenar una IA empotrada para la clasificación de muestras
- Implementar algún mecanismo de interfaz de usuario y representación
- Evaluar el desempeño de la IA lograda

### Tutor

Angel Jiménez Fernández

### Cotutor

Daniel Casanueva Morato

## Estado actual del firmware (Rust + Embassy)

Se ha inicializado un firmware embebido para la placa `B-L4S5I-IOT01A` con MCU `STM32L4S5VI`:

- Toolchain en Rust `edition = "2024"`.
- Runtime asíncrono con `embassy-executor`.
- HAL con `embassy-stm32` configurado para `stm32l4s5vi`.
- Runner de depuración/flasheo con `probe-rs`.

### Estructura implementada

- `src/board.rs`:
  - Inicialización de placa.
  - LED de estado en `PB14`.
  - Bus IMU por `I2C2` (`PB10/PB11`) con DMA e interrupciones.

- `src/acquisition.rs`:
  - Configuración básica de `LSM6DSL`.
  - Muestreo periódico (~100 Hz).
  - Buffer circular de muestras inerciales.
  - Envío de ventanas de señal al motor de pitch.

- `src/pitch.rs`:
  - Estimador de pitch baseline (autocorrelación normalizada, aritmética entera).
  - Mapeo a cuerdas de bajo `E1/A1/D2/G2`.
  - Estimación de desviación en cents y confianza.
  - Harness de validación sintético de arranque.

- `src/ui.rs`:
  - Estado de afinación (`flat`, `in-tune`, `sharp`, `no-pitch`).
  - Salida MVP por trazas `defmt`.

### Flujo de tareas Embassy

1. `acquisition_task`: lee IMU y publica ventanas de datos.
2. `pitch_task`: estima frecuencia + nota + error.
3. `ui_task`: publica estado de afinación en logs.

### Comandos

- Comprobar compilación:
  - `cargo check`

- Ejecutar en placa (requiere sonda compatible con probe-rs):
  - `cargo run`

- Tests de host (solo lógica pura de Rust):
  - `cargo test --target x86_64-unknown-linux-gnu`

### Nota sobre NanoEdge AI

NanoEdge AI queda como vía posterior a MVP en este repositorio: no hay aún librería C generada ni capa FFI integrada, por lo que el camino MVP actual es el pipeline nativo en Rust.

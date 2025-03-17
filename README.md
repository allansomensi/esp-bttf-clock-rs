# ESP BTTF Clock

A clock/lamp built with **ESP32**, using **Rust** and the `esp-idf` framework. It connects to Wi-Fi to synchronize time via **SNTP**.

## Features
- Synchronizes time via Wi-Fi using **SNTP** with timezone support.
- Web portal for configuration and control of settings.
- Basic error handling to ensure smooth operation.
- Handlers for **Display**, **LedStrip**, and **Wifi** components.
- HTTP server with route handling for communication.

# ⏳⚡ ESP BTTF Clock

A feature-rich **ESP32** clock and lamp built using **Rust** and the `esp-idf` framework. The device connects to Wi-Fi, synchronizes time via **SNTP**, and offers a web portal for configuration and customization.

## 🌟 Features
- ⏰ **Time Synchronization:** Automatically syncs time via SNTP.
- 🌐 **Web Portal:** User-friendly interface for configuring and controlling the clock.
- 📡 **Captive Portal:** Simplifies Wi-Fi connection by automatically redirecting to the setup page.
- 🔗 **mDNS Support:** Access the web portal easily using a hostname instead of an IP address.
- 🎨 **Color Themes:** Multiple LED color themes for personalized aesthetics.
- 🌍 **Dynamic Timezones:** Supports 500+ adjustable timezones.
- 💾 **Persistent Storage:** Utilizes NVS (Non-Volatile Storage) for saving reusable settings.
- 🔒 **Secure by Design:** Built with Rust and TypeScript, ensuring memory safety and type safety.

## 🛠️ Setup & Usage

### 📋 Prerequisites

- **Rust** toolchain (recommended: nightly version).
- **Node.js** for Captive and Web Portals.
- **Just** for utility scripts (see [Install Just](https://github.com/casey/just#installation)).
- **ESP-IDF** environment properly set up (see [ESP-RS setup](https://docs.espressif.com/projects/rust/book/installation/index.html)).
- **ESP32 D1-mini** development board.

### 🔧 Development Setup

#### 1. Clone the repository:
```bash
git clone https://github.com/allansomensi/esp-bttf-clock-rs.git
cd esp-bttf-clock-rs
```

#### 2. Build Captive and Web Portals:
```elixir
just build
```

#### 3. If you want, you can modify the environment variables in `.cargo/config.toml`:
```toml
[env]
AP_IP_ADDRESS = "192.168.71.1"
AP_SSID = "My AP SSID"
AP_PASSWORD = "My AP password" # Must be at least 8 characters long
```

#### 4. Compile and flash the firmware:
```elixir
just run  # or simply `just`
```

## 🛑 Common Issues & Troubleshooting

### ❌ No Wi-Fi 5G Support
The ESP32 D1-mini does not support 5GHz Wi-Fi networks, as it only operates on the 2.4GHz band. Ensure that your router has a 2.4GHz network enabled and connect to it.

### 🚦 Captive Portal Not Redirecting Automatically
If the login notification to connect to the network does not appear and you are not automatically redirected to the Wi-Fi setup page, manually enter the following URL in your browser:
`http://{{ip_address}}`

By default, the Access Point IP address is **192.168.71.1**.

### 🌍 Web Portal Not Accessible
- ✅ Verify that the ESP32 is properly connected to the correct Wi-Fi network.
- 🌐 If mDNS (`http://espclock.local`) is not working, first ensure that you are accessing it via **HTTP** and not HTTPS.
- 🔍 If the issue persists, check your router settings to find the assigned IP address and use it directly.

### ⏳ Time Synchronization Fails
- 📶 Ensure the ESP32 has internet access and can reach SNTP servers.
- 🔒 Check if your firewall or network settings block NTP traffic.

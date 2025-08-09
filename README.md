# AutoClic !! 🖱️

AutoClic use devcontainers to simplify the development.

## NAPI-RS

This is the core of the application.
It is the component responsible for creating the autoclicker thread.
Napi-rs is used to create a Node.js library written in Rust.

```sh
npm install -g @napi-rs/cli

napi new
```

## Electron
```sh
npm create @quick-start/electron@latest

export DISPLAY=:1.0

electron-vite dev --noSandbox

npm run dev
```

## devcontainer

### port config
```json
"forwardPorts": [6080],
"portsAttributes": {
    "6080": {
    "label": "desktop"
    }
}
```

## Getting Started

Clone and open the project in a dev container.

Navigate to the `📁 autoclic` folder, run `npm install`, then `npm run build`.

Next, go to the `📁 autoclic-app` folder, run `npm install`, then `npm run dev`.

In your browser, open the virtual desktop URL: http://localhost:6080/

## 🛠️ Cross-Compile Guide

For more details, refer to the official cross-build documentation.

---

### 🪟 Windows Target

#### ✅ Using `cargo-xwin`

Install the necessary tools:

```sh
rustup component add llvm-tools
cargo install --locked cargo-xwin
```

Example build command:

```sh
cargo xwin build --release --target x86_64-pc-windows-msvc
```

#### ✅ With `napi-rs`

To build with NAPI-RS for Windows:

```sh
napi build --platform --release --target x86_64-pc-windows-msvc --cross-compile
```

---

### ⚡ Electron Build for Windows

#### 🧰 Install Wine (on Debian/Ubuntu)

Follow the Wine installation guide:

```sh
sudo dpkg --add-architecture i386
sudo apt install --install-recommends winehq-stable

sudo mkdir -pm755 /etc/apt/keyrings
wget -O - https://dl.winehq.org/wine-builds/winehq.key | sudo gpg --dearmor -o /etc/apt/keyrings/winehq-archive.key

sudo wget -NP /etc/apt/sources.list.d/ https://dl.winehq.org/wine-builds/debian/dists/bookworm/winehq-bookworm.sources

winecfg
```

Optional: Install NSIS for Windows installer creation

```sh
sudo apt install nsis
```

#### 🏗️ Build Electron App

```sh
npm run build:win
```

### ⚡ Electron Build for Linux ARM

Optional zig installation

```sh
sudo apt install -y tar xz-utils
# Install zig
wget https://ziglang.org/download/0.14.1/zig-x86_64-linux-0.14.1.tar.xz && \
tar -C /usr/local -xf zig-x86_64-linux-0.14.1.tar.xz && \
rm zig-x86_64-linux-0.14.1.tar.xz

export PATH="$PWD/zig-linux-x86_64-0.11.0:$PATH"

rustup target add aarch64-unknown-linux-gnu
```
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
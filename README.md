# QuantSimul ⚛️

A high-performance, hybrid Quantum Circuit Simulator built with **Rust**, **WebAssembly**, and **Svelte**. 

QuantSimul allows users to design quantum circuits using a visual drag-and-drop editor, simulate quantum states in real-time via a Rust-powered engine, and persist data using a hybrid SQLite backend.

## 🚀 Features

- **Visual Circuit Editor:** Drag-and-drop quantum gates (H, X, Y, Z, CNOT, CCX/Toffoli) onto a multi-qubit timeline.
- **High-Performance Engine:** Quantum statevector math computed in Rust and compiled to WebAssembly for near-native browser speed.
- **3D Visualization:** Real-time Bloch Sphere representation using Three.js.
- **Multi-User Backend:** Secure login system (Local & Social) with a Rust Axum API and SQLite persistence.
- **Cross-Platform:** Developed with modern tooling compatible with Windows (GNU/MSYS2) and Linux (Arch/EndeavourOS).

## 🛠️ Tech Stack

- **Frontend:** [Svelte](https://svelte.dev) + [Three.js](https://threejs.org) (Visuals)
- **Backend API:** [Rust](https://rust-lang.org) + [Axum](https://github.com) (Web Server)
- **Quantum Core:** Rust + [WASM-Pack](https://github.io) (Mathematical Engine)
- **Database:** [SQLite](https://sqlite.org) (Local Persistence)

## 📦 Installation & Setup

### Prerequisites
- Rust & Cargo
- Node.js & NPM
- wasm-pack

### 1. Build the Quantum Engine (WASM)
```bash
cd simulatorquantic
wasm-pack build --target web --out-dir ../frontend/src/lib/pkg
```

### 2. Run the Backend API
```bash
cd ..
# Ensure you are in the folder containing main.rs
cargo run
```

### 3. Run the Frontend
```bash
cd frontend
npm install
npm run dev
```

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.

---
*Created as a high-performance quantum computing educational tool.*

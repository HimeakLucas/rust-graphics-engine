# rust-graphics-engine

A Graphics Engine built in Rust using OpenGL and real-time post-processing filters.

## First things first: why Rust?

Rust handles memory in a smart and safe way — which is ideal for an engine that deals with lots of complex data types.

If you want a deeper explanation, check out the [Why Rust?](./why_rust.md) document!





## 🚀 Implemented Packages & Systems

### ✅ Model Loading Package
The project already includes a **model loading package**, allowing 3D models to be imported and used within the rendering pipeline.

you can see a example by running the command below:

```bash
cargo run --example ML_demo
```

### 🎨 Simple Shader System
A **basic shader system** is also implemented, enabling custom vertex and fragment shaders to be loaded, compiled, and applied to objects in the scene.

to see a basic use of it run:

```bash
cargo run
```

# 📦 Installation

Make sure you have:
- Rust (latest stable)
- Cargo
- A system with OpenGL 3.3+ support

Clone the repository:
```bash
git clone https://github.com/HimeakLucas/rust-graphics-engine.git
cd rust-graphics-engine
```

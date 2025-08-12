# Phoenix

Phoenix is a high-performance graphics engine written in Rust, supporting both OpenGL and Vulkan backends. It is designed for flexibility, speed, and modern rendering techniques.

## Features

- Modular architecture
- OpenGL and Vulkan support
- Cross-platform (Windows, Linux, macOS)
- Modern rendering pipeline
- Easy integration with Rust projects

## Project Structure

```
phoenix/
├── src/                    # Core engine source code
│   ├── lib.rs             # Main library entry point
│   ├── renderer/          # Rendering system components
│   ├── graphics/          # Graphics API abstractions (OpenGL/Vulkan)
│   ├── math/              # Mathematical utilities and linear algebra
│   ├── scene/             # Scene management and entities
│   └── utils/             # Common utilities and helpers
├── bins/                   # Binary executables
│   ├── prototyping/       # Prototyping and demo applications
│   └── graphic-tests/     # Automated graphics testing suite
```

### Key Components

- **Renderer**: Core rendering pipeline supporting multiple backends
- **Graphics API Layer**: Abstraction over OpenGL and Vulkan APIs
- **Scene System**: Entity management and scene graph implementation
- **Math Library**: Optimized linear algebra for graphics operations

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- Git
- CMake (for some dependencies)
- Appropriate graphics drivers for OpenGL/Vulkan

### Building the Project

```sh
git submodule update --init
cargo build --release
```

### Running Examples

```sh
cargo run --bin --prototyping -- -d light
```
Replace `<example_name>` with the desired example (see the `examples/` directory).

## Running Graphic Tests

To run the automated graphic tests, use:

```sh
cargo run --bin graphic-tests --release --
# or
cargo run-gt
# help
cargo run --bin graphic-tests --release -- -h
```

This will execute all tests that require a graphical environment. Make sure your system supports the required graphics backend and that you have a display available.

## Testing and Coverage

Install [cargo-llvm-cov](https://crates.io/crates/cargo-llvm-cov):

```sh
cargo install cargo-llvm-cov
```

Run tests with coverage:

```sh
cargo llvm-cov
# or to generate an HTML report:
cargo llvm-cov --html
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [OpenGL](https://www.opengl.org/)
- [Vulkan](https://www.vulkan.org/)

---
For more information, see the documentation or open an issue.

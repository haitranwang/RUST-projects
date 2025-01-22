# Cargo Directory Structure

Cargo recognizes the `examples` folder in your workspace because it is a standard directory used by Cargo to store example binaries. When you run a command with the `--example` flag, Cargo automatically looks for the specified example in the `examples` directory of your package.

Here's how Cargo organizes and recognizes different types of files and directories:

- `src/`: Main source code of your package.
- `tests/`: Integration tests.
- `benches/`: Benchmarks.
- `examples/`: Example binaries.
# Cargo Package
https://doc.rust-lang.org/cargo/commands/cargo-package.html

By following this convention, Cargo can easily locate and manage different types of code in your project.

The `--package` flag is used to specify a particular package in a workspace when running a command. This is useful when you have multiple packages in a single workspace and you want to run a command for a specific package.

Here's an example of how to use the `--package` flag:

Assume you have a workspace with the following structure:

```
my_workspace/
├── Cargo.toml
├── package1/
│   └── Cargo.toml
├── package2/
│   └── Cargo.toml
└── examples/
    └── sample_0.rs
```

To run the main binary of `package1`, you would use:

```sh
cargo run --package package1
```


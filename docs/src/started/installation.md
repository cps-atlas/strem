To install STREM, you must first have [Rust](https://www.rust-lang.org/) installed on your system. After installing Rust, run the following command from your preferred shell:

```bash
cargo install strem
```

Alternatively, for the **latest** version, you can install the tool with the following command:

```bash
cargo install --git https://github.com/cps-atlas/strem.git
```

### Verifying Installation

You can verify that the installation has succeeded by running the `strem` command within your preferred shell with the `--version` (`-V` for shorthand notation) flag as such:

```bash
strem --version
```

If you received `strem X.Y.Z` (where X, Y, and Z are versioning numbers) as the output, the tool is ready for use! You can further familiarize yourself with its usage by passing the help flag `-h`.

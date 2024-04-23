# Uv Plugin

[![ci](https://github.com/fluentci-io/uv-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/uv-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [uv](https://github.com/astral-sh/uv).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm uv setup
```

## Functions

| Name   | Description                            |
| ------ | -------------------------------------- |
| setup  | Installs a specific version of uv.     |
| pip    | Resolve and install Python packages    |
| venv   | Create a virtual environment           |
| clean  | Clear the cache                        |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/uv@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: uv
    args: |
      setup
- name: Show uv version
  run: |
    type uv
    uv --version
```

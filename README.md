# Capistrano Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/capistrano)](https://pkg.fluentci.io/capistrano)
[![ci](https://github.com/fluentci-io/capistrano-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/capistrano-plugin/actions/workflows/ci.yml)

This plugin install and run [Capistrano](https://capistranorb.com/) on your CI/CD pipelines.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm capistrano setup
fluentci run --wasm capistrano deploy production
```

## Functions

| Name      | Description                                |
| --------- | ------------------------------------------ |
| setup     | Install Capistrano                         |
| deploy    | Deploy your application using Capistrano   |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/capistrano@v0.1.1?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: capistrano
    args: |
      setup
- name: Show capistrano help
  run: |
    flox activate -- type cap
    flox activate -- cap --version
    flox activate -- cap --help
```

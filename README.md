# What is this?

Servicer is a tool to run apps and servers indefinitely. It is a modern alternative to pm2 written in Rust that runs on systemd. Systemd is a great tool but complicated to use. Servicer is daemonless, it provides a simple pm2 style API that proxies to systemd. The pm2 of Rust, Golang, Java is here.

## Goals

- **Use OS native primitives**: Servicer does not fork processes nor run a custom logger. It hooks up your app into systemd and gets out of the way. Logs are handled by journald. You need not worry about your services going down if something wrong happens to servicer.
- **Lightweight**: Servicer is daemonless, i.e. does not run in the background consuming resources.
- **Language agnostic**: Servicer comes as a binary executable which does not require rust installed. There is not bloat from language exclusive features.

## Platform support

Currently servicer supports Linux. Systemd must be installed on the system. MacOS (launchd) and Windows (SCM) support is planned.

## How do I install it?

### Snap

```sh

```

### Cargo

```sh
cargo install servicer
```

## Usage

1. Create service

## Quirks

1. nvm: `node` is unavailable in sudo mode. You must symlink `node` to the path available to sudo. Source- https://stackoverflow.com/a/40078875/7721443

  ```sh
  sudo ln -s "$NVM_DIR/versions/node/$(nvm version)/bin/node" "/usr/local/bin/node"
  sudo ln -s "$NVM_DIR/versions/node/$(nvm version)/bin/npm" "/usr/local/bin/npm"
  ```

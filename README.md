# Guide

```sh
cargo build && ./target/debug/stabled start 'hello-world.js' --name 'hello-world'
```

## Quirks

1. nvm: `node` is unavailable in sudo mode.
  - Option 1: Symlink `node` to the path available to sudo. Source- https://stackoverflow.com/a/40078875/7721443

  ```sh
  sudo ln -s "$NVM_DIR/versions/node/$(nvm version)/bin/node" "/usr/local/bin/node"
  sudo ln -s "$NVM_DIR/versions/node/$(nvm version)/bin/npm" "/usr/local/bin/npm"
  ```

  - Option 2: Find the path of `node` as regular user

  - Option 3: Use `n` instead of `nvm`


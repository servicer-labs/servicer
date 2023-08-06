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

# Create a symlink to use the short name `ser`. We can now access servicer in sudo mode
sudo ln -s /home/your_username/.cargo/bin/servicer /usr/bin/ser
```

## Usage

Run `--help` to display tooltip. Note that `sudo` mode is needed for all commands except `status`.

### 1. Create service

```sh
# Create a service for index.js
sudo ser create index.js

# Create service, start and enable on boot
sudo ser create index.js --start --enable

# Create a service for a binary
sudo ser create awesome-binary

# Custom interpreter
sudo ser create hello-typescript.ts --interpreter /home/hp/.config/nvm/versions/node/v20.1.0/bin/ts-node

# Custom name
sudo ser create index.js --name hello-world

# Pass params to index.js by adding them after a `--` followed by space
sudo ser create index.js -- --foo bar

# Pass env variables
sudo ser create index.js --env-vars "FOO=BAR GG=WP"

# Enable auto-restart on exit
sudo ser create index.js --auto-restart
```

- This creates a service file in `etc/systemd/system/hello-world.ser.service`. You must follow up with `start` and `enable` commands to start the service.

- Servicer auto-detects the interpreter for `node` and `python` from $PATH available to the sudo user. You must manually provide the interpeter for other platforms using the interpreter flag, eg. `--interpreter conda`. If the interpreter is not found in sudo $PATH, run `which conda` and paste the absolute path.

- You can write your own service files and manage them with `servicer`. Simply rename file to end with `.ser.service` instead of `.service`.


#### Docs

```
Usage: ser create [OPTIONS] <PATH> [-- <INTERNAL_ARGS>...]

Arguments:
  <PATH>              The file path
  [INTERNAL_ARGS]...  Optional args passed to the file. Eg. to run `node index.js --foo bar` call `ser create index.js -- --foo bar`

Options:
  -n, --name <NAME>                Optional custom name for the service
  -s, --start                      Start the service
  -e, --enable                     Enable the service to start every time on boot. This doesn't immediately start the service, to do that run together with `start
  -r, --auto-restart               Auto-restart on failure. Default false. You should edit the .service file for more advanced features. The service must be enabled for auto-restart to work
  -i, --interpreter <INTERPRETER>  Optional custom interpreter. Input can be the executable's name, eg `python3` or the full path `usr/bin/python3`. If no input is provided servicer will use the file extension to detect the interpreter
  -v, --env-vars <ENV_VARS>        Optional environment variables. To run `FOO=BAR node index.js` call `servicer create index.js --env_vars "FOO=BAR"`
  -h, --help                       Print help
```

2. Start service

```sh

```

## Quirks

1. nvm: `node` is unavailable in sudo mode. You must symlink `node` to the path available to sudo. Source- https://stackoverflow.com/a/40078875/7721443

  ```sh
  sudo ln -s "$NVM_DIR/versions/node/$(nvm version)/bin/node" "/usr/local/bin/node"
  sudo ln -s "$NVM_DIR/versions/node/$(nvm version)/bin/npm" "/usr/local/bin/npm"
  ```

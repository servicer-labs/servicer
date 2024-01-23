# servicer: Simplify Service Management on systemd

[![Crates.io](https://img.shields.io/crates/v/servicer?style=flat-square)](https://crates.io/crates/servicer)
[![Crates.io](https://img.shields.io/crates/d/servicer?style=flat-square)](https://crates.io/crates/servicer)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

`servicer` is a user-friendly Command Line Interface (CLI) tool designed to simplify service management on `systemd`, abstracting away the complexities of the systemd ecosystem. With an easy-to-use API comparable to popular tools like pm2, servicer empowers users to create, control, and manage services effortlessly.

## Key Features:

1. **Intuitive CLI**: servicer provides a simple and intuitive command-line interface, making it accessible to both beginners and experienced users.

2. **Service Creation**: Easily create and define new services by specifying essential details like service name, command, working directory, and environment variables.

3. **Service Control**: Start, stop, restart, enable, or disable services seamlessly with straightforward commands.

4. **Process Monitoring**: Monitor the status and health of services, ensuring reliable operation and automatic restarts in case of failures.

5. **Service Logs**: View real-time service logs directly from the terminal to facilitate troubleshooting and debugging.

6. **Cross-platform Compatibility**: servicer is designed to work on various Linux distributions with systemd support. MacOS and Windows support using `launchd` and `scm` is planned.

## Goals

1. **Use OS native primitives**: Unlike `pm2`, `servicer` does not fork processes nor run a custom logger. It hooks up your app into systemd and gets out of the way. Logs are handled by journald. You need not worry about your services going down if something wrong happens to `servicer`.

2. **Lightweight**: Servicer is daemonless, i.e. does not run in the background consuming resources.

3. **Language agnostic**: Servicer comes as a binary executable which does not require rust to be installed. There is not bloat from language exclusive features, such as `pm2` cluster mode for node.

## Platform support

Currently servicer only supports systemd-based Linux. MacOS (launchd) and Windows (SCM) support is planned.

## How do I install it?

### Download binary

```sh
sudo wget https://github.com/servicer-labs/servicer/releases/latest/download/servicer-x86_64-unknown-linux-gnu -O /usr/local/bin/servicer
sudo chmod +x /usr/local/bin/servicer
```

### Cargo

```sh
cargo install servicer
```

### AUR

On Arch Linux and derivatives (like Manjaro or EndavourOS), you can install one of the AUR packages:

```sh
# choose one
paru -S servicer # stable, build from source
paru -S servicer-bin # stable, from GitHub releases
paru -S servicer-git # main branch, latest development version
```

### IMPORTANT FOR NVM USERS

If you installed node with nvm, it isn’t visible to root. Fix this by symlinking node and npm to `/usr/local/bin`
```sh
sudo ln -s $(which node) "/usr/local/bin/node"
sudo ln -s $(which npm) "/usr/local/bin/npm"
```

### Alias to ser

If you want to type `ser` instead of `servicer`, add the following to your shell config file.
```sh
echo 'alias ser=servicer' >> ~/.bashrc # for bash
echo 'alias ser=servicer' >> ~/.zshrc # for zsh
echo 'alias ser=servicer' >> ~/.config/fish/config.fish # for fish
```

## Usage

Run `--help` to display tooltip. Note that `sudo` mode is needed for most commands.

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

- This creates a service file in `/etc/systemd/system/hello-world.ser.service`. You must follow up with `start` and `enable` commands to start the service.

- Servicer auto-detects the interpreter for `node` and `python` from $PATH available to root. You must manually provide the interpreter for other platforms using the interpreter flag, eg. `--interpreter conda`. If the interpreter is not found in sudo $PATH, run `which conda` and paste the absolute path.

- You can write your own service files and manage them with `servicer`. Simply rename file to end with `.ser.service` instead of `.service`.

### 2. Edit .service file

```sh
sudo ser edit hello-world

# Custom editor
sudo ser edit hello-world --editor vi
```

Opens a `.service` file in a text editor. Provides a template if the service doesn't exist.

### 3. Start service

```sh
sudo ser start hello-world
```

### 4. Stop service

```sh
sudo ser stop hello-world
```

### 5. Enable service

```sh
sudo ser enable hello-world
```

### 6. Disable service

```sh
sudo ser disable hello-world
```

### 7. Delete service

```sh
sudo ser delete hello-world

sudo ser rm hello-world
```

### 8. View status of services

Prints PID, name, active state, enabled state, CPU and memory utilization for every service.

```sh
sudo ser status
```

```
+-------+-------------+--------+----------------+-------+--------+
| pid   | name        | active | enable on boot | cpu % | memory |
+-------+-------------+--------+----------------+-------+--------+
| 24294 | index.js    | active | false          | 0     | 9.5 KB |
+-------+-------------+--------+----------------+-------+--------+
```

### 9. View file paths for a service

Finds the `.service` and unit file path for a service.

```sh
sudo ser which hello-world
```

```sh
+--------------+--------------------------------------------------------------+
| name         | path                                                         |
+--------------+--------------------------------------------------------------+
| Service file | /etc/systemd/system/hello-world.ser.service                  |
+--------------+--------------------------------------------------------------+
| Unit file    | /org/freedesktop/systemd1/unit/hello_2dworld_2eser_2eservice |
+--------------+--------------------------------------------------------------+
```

### 10. View logs

```sh
ser logs hello-world

# Follow live logs
ser logs hello-world --follow
```

### 11. Print contents of .service file

```sh
ser cat hello-world
```

### 13. Rename service

```sh
ser rename index.js hello-world

# Or
ser mv index.js hello-world
```

## License

`servicer` is licensed under the MIT license.

## Disclaimer

`servicer` is distributed "as-is" and without any warranty, expressed or implied. The authors and contributors of `servicer` shall not be held liable for any damages or losses resulting from the use or inability to use the software.

Before using `servicer`, please review the MIT License and the lack of warranty carefully. By using the software, you are agreeing to the terms of the license and acknowledging the lack of warranty.

## Acknowledgements

We acknowledge all the packages and libraries used in the development of `servicer`. Their contributions have been invaluable in making this project possible. Parts of the README and codebase are generated with ChatGPT.

## Contribution and support

We welcome contributions and feedback from the community. Feel free to open issues, submit pull requests, or share your thoughts on how we can improve servicer further.

Get started with servicer and simplify your service management on systemd. Happy service creation!

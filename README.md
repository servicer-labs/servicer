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

## D-bus concepts

A dbus service has 3 parts:

1. Name: The service name, i.e. `org.freedesktop.Notifications`
2. Object: An instance to interact with the service. Eg `org/freedesktop/Notifications`
3. Interface: XML ABI to interact with service, eg


```sh
# Start
## ss is the signature, i.e. two strings must be passed.
busctl call \
  org.freedesktop.systemd1 \
  /org/freedesktop/systemd1 \
  org.freedesktop.systemd1.Manager \
  StartUnit ss \
  "hello.service" \
  "replace"

busctl call \
  org.freedesktop.systemd1 \
  /org/freedesktop/systemd1 \
  org.freedesktop.systemd1.Manager \
  GetDefaultTarget

# Scripts to read PID, RAM and other details are in the next section
sudo systemctl list-units > list-units.txt

busctl call org.freedesktop.systemd1 /org/freedesktop/systemd1 org.freedesktop.systemd1.Manager ListUnits

#TODO find equivalent of `systemctl show`. The below scripts don't work

# Get status of hello-world, equivalent of `sudo systemctl status hello-world.stabled`
busctl get-property org.freedesktop.systemd1 /org/freedesktop/systemd1/unit/hello_2dworld_2establed_2eservice org.freedesktop.systemd1.Unit ActiveState


# Stop
```

- `ListUnits` output for hello-world

```
"hello-world.stabled.service" "stabled: hello-world" "loaded" "active" "running" "" "/org/freedesktop/systemd1/unit/hello_2dworld_2establed_2eservice"
```

- Naming convention of paths: `/` and `_` are ok
- Other characters are encoded as lowercase HEX, eg `-` becomes `_2d`
- How is `_` escaped?

```
  hello-world.stabled.service                                                                loaded active     running   stabled: hello-world
```

- `sudo systemctl status hello-world.stabled` returns a bunch of details. Unfortunately it doesn't expose a DBUS API to read RAM and CPU. We use two objects `org.freedesktop.systemd1.Unit` and `org.freedesktop.systemd1.Service` to read fields.

  - Loaded state, active state and sub state (alive/dead): `busctl get-property org.freedesktop.systemd1 /org/freedesktop/systemd1/unit/hello_2dworld_2establed_2eservice org.freedesktop.systemd1.Unit LoadState ActiveState SubState`

  - PID: This is found on the service unit object, i.e. `.Service` not `.Unit`. `busctl get-property org.freedesktop.systemd1 /org/freedesktop/systemd1/unit/hello_2dworld_2establed_2eservice org.freedesktop.systemd1.Service MainPID`

  - Memory and CPU: read from `/proc/{pid}/stat`. Use crate https://github.com/rust-psutil/rust-psutil
  - Logs

- Source code: https://github.com/systemd/systemd/blob/4cf5b343c927509ea91cf56ca88e330f193a6963/src/systemctl/systemctl-show.c#L713

```
● hello-world.stabled.service - stabled: hello-world
     Loaded: loaded (/etc/systemd/system/hello-world.stabled.service; disabled; preset: enabled)
     Active: active (running) since Wed 2023-07-26 18:57:54 +04; 10min ago
   Main PID: 25875 (node)
      Tasks: 7 (limit: 18621)
     Memory: 10.2M
        CPU: 159ms
     CGroup: /system.slice/hello-world.stabled.service
             └─25875 /usr/local/bin/node index.js

Jul 26 18:57:54 hp systemd[1]: Started hello-world.stabled.service - stabled: hello-world.
Jul 26 18:57:54 hp node[25875]: Server listening on port 3000
```

## Logs

```sh
# Show latest logs but does not stream them. Shows 10 lines at most. No separation between error and output streams.
systemctl status hello-world.stabled.service

# Using journalctl- this shows old logs. Logs are retained even after the service is deleted.
journalctl -u hello-world.stabled.service
```

- Read logs from `journalctl`, then filter upto the start command

```
Aug 04 12:39:01 hp node[3510]: Server listening on port 3000
Aug 04 13:57:56 hp systemd[1]: Stopping hello-world.stabled.service - stabled: hello-world...
Aug 04 13:57:56 hp systemd[1]: hello-world.stabled.service: Deactivated successfully.
Aug 04 13:57:56 hp systemd[1]: Stopped hello-world.stabled.service - stabled: hello-world.
Aug 04 14:33:41 hp systemd[1]: Started hello-world.stabled.service - stabled: hello-world.
Aug 04 14:33:51 hp node[10218]: this is output
Aug 04 14:34:01 hp node[10218]: this is error
Aug 04 14:34:01 hp node[10218]: this is output
Aug 04 14:34:11 hp node[10218]: this is output
Aug 04 14:34:21 hp node[10218]: this is error
Aug 04 14:34:21 hp node[10218]: this is output
Aug 04 14:34:31 hp node[10218]: this is output
Aug 04 14:34:41 hp node[10218]: this is error
```

- Separating output from error: `journalctl -u hello-world.stabled.service -p err` has old messages, not `console.error()` messages. journald doesn't distinguish between stdout and stderr yet- https://github.com/systemd/systemd/pull/6599#issuecomment-1658445824

- Flushing: No need of the command. Systemd automatically rotates logs.

- Solution: past N lines upto the start command, while streaming the latest output.

```
journalctl -u hello-world.stabled.service -n 15 --follow
```
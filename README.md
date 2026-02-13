# Rustainer

## A minimal container runtime written in Rust.

## Architecture

### Container Execution Flow

![Container Execution Flow](https://mermaid.ink/svg/Zmxvd2NoYXJ0IFRECiAgICBBW0NMSSAvIFJFUEwgSW5wdXRdIC0tPiBCe1BhcnNlIENvbW1hbmR9CiAgICBCIC0tPnxydW58IENbQ29udGFpbmVyIFJ1bnRpbWVdCiAgICBCIC0tPnxwdWxsfCBEW0ltYWdlIE1hbmFnZXJdCiAgICBCIC0tPnxyZXBsfCBFW0ludGVyYWN0aXZlIFNoZWxsXQogICAgCiAgICBDIC0tPiBGW1NldHVwIE5hbWVzcGFjZXNdCiAgICBGIC0tPiBHW0NvbmZpZ3VyZSBDZ3JvdXBzXQogICAgRyAtLT4gSFtNb3VudCBGaWxlc3lzdGVtc10KICAgIEggLS0-IElbQ2hyb290IEludG8gUm9vdGZzXQogICAgSSAtLT4gSltFeGVjdXRlIFByb2Nlc3Nd)

### System Components

![System Components](https://mermaid.ink/svg/Zmxvd2NoYXJ0IExSCiAgICBzdWJncmFwaCBDTElbIkNMSSBMYXllciJdCiAgICAgICAgQVttYWluLnJzXQogICAgICAgIEJbcmVwbC5yc10KICAgIGVuZAogICAgCiAgICBzdWJncmFwaCBDb3JlWyJDb3JlIFJ1bnRpbWUiXQogICAgICAgIENbcnVuLnJzXQogICAgICAgIERbbmFtZXNwYWNlLnJzXQogICAgICAgIEVbY2dyb3VwLnJzXQogICAgZW5kCiAgICAKICAgIHN1YmdyYXBoIFN0b3JhZ2VbIkltYWdlIFN0b3JhZ2UiXQogICAgICAgIEZbc3RvcmFnZS5yc10KICAgICAgICBHW2RvY2tlci5yc10KICAgICAgICBIW3B1bGwucnNdCiAgICBlbmQKICAgIAogICAgQSAtLT4gQwogICAgQiAtLT4gQwogICAgQyAtLT4gRAogICAgQyAtLT4gRQogICAgQyAtLT4gRgogICAgSCAtLT4gRw)

### Container Lifecycle

![Container Lifecycle](https://mermaid.ink/svg/c3RhdGVEaWFncmFtLXYyCiAgICBbKl0gLS0-IENyZWF0ZWQKICAgIENyZWF0ZWQgLS0-IFJ1bm5pbmc6IHN0YXJ0CiAgICBSdW5uaW5nIC0tPiBTdG9wcGVkOiBleGl0CiAgICBSdW5uaW5nIC0tPiBTdG9wcGVkOiBzaWduYWwKICAgIFN0b3BwZWQgLS0-IFsqXTogY2xlYW51cA)

## Installation

```sh
# Clone
git clone https://github.com/Adel-Ayoub/rustainer.git
cd rustainer

# Build
make

# Run
./target/debug/rustainer repl
```

---

## Requirements

- Rust toolchain (2021 edition)
- Make
- Linux (full support) or macOS/BSD (limited support)

---

## Features

### Completed Features

#### Core Container Functionality
- Chroot Isolation: Change root directory for container filesystem
- Mount Support: Mount proc, tmp, and other filesystems
- Namespace Isolation: PID, UTS, Mount namespace support (Linux)
- Process Management: Container process lifecycle management

#### Resource Management
- Cgroups Support: Memory, CPU, and PID limits
- Resource Limits: Configurable resource constraints
- Process Isolation: Isolated process trees

#### Image Management
- Docker Format: Docker image format structure support
- Layer Storage: Image layer storage and extraction
- Manifest Handling: OCI manifest parsing

#### CLI & Interface
- Command Line: Full CLI with subcommands
- Interactive REPL: Interactive shell for container management
- Tracing: Debug logging with tracing support

### In Progress
- Image Pull: Pull images from Docker Hub
- Network Namespace: Network isolation support
- User Namespace: User ID mapping

### Planned Features
- Container Hooks: Pre/post start hooks
- Volume Mounts: Bind mount support
- Resource Monitoring: Real-time resource usage

---

## Commands

| Command | Description |
|---------|-------------|
| `run` | Run application in container |
| `repl` | Start interactive REPL mode |
| `help` | Show help information |

---

## Usage Examples

### Installation
```sh
# Clone
git clone https://github.com/Adel-Ayoub/rustainer.git
cd rustainer

# Build
make

# Run REPL
./target/debug/rustainer repl
```

### Create Root Filesystem
```bash
# Using Docker to create alpine rootfs
mkdir alpine-rootfs
docker export $(docker create alpine) | tar -C alpine-rootfs -xvf -
```

### Run Container (Linux only)
```bash
# Run shell in container
sudo ./target/debug/rustainer run -f alpine-rootfs /bin/sh

# Run command with arguments
sudo ./target/debug/rustainer run -f alpine-rootfs /bin/ls -la
```

### Interactive REPL
```bash
# Start REPL
./target/debug/rustainer repl

# REPL commands
rustainer> help
rustainer> info
rustainer> ps
rustainer> images
rustainer> exit
```

### REPL Commands
```bash
help          # Show available commands
info          # Show system information
version       # Show version
list, ls      # List all containers
images        # List downloaded images
ps            # List running containers
clear         # Clear screen
exit, quit    # Exit REPL
```

---

## Build System

| Target | Description |
|--------|-------------|
| `make` | Build debug binary |
| `make build` | Build debug binary |
| `make release` | Build optimized release binary |
| `make test` | Run tests |
| `make clean` | Remove build artifacts |
| `make check` | Check code without building |
| `make fmt` | Format code |
| `make lint` | Run clippy linter |

---

## Project Structure

```
rustainer/
├── Cargo.toml
├── Cargo.lock
├── Makefile
├── README.md
└── src/
    ├── main.rs           # Entry point with CLI
    ├── cgroup.rs         # Cgroups resource management
    ├── namespace.rs      # Linux namespace isolation
    ├── docker.rs         # Docker image structures
    ├── storage.rs        # Image/container storage
    ├── utils.rs          # Utility functions
    └── commands/
        ├── mod.rs
        ├── run.rs        # Container run command
        ├── pull.rs       # Image pull command
        └── repl.rs       # Interactive REPL
```

---

## Platform Support

| Platform | Support Level | Features |
|----------|--------------|----------|
| Linux | Full | namespaces, cgroups, chroot, mounts |
| macOS | Limited | chroot only |
| BSD | Limited | chroot only |

---

## Future Improvements

- [x] Chroot isolation
- [x] Mount support (proc, tmp)
- [x] Namespace isolation (PID, UTS, Mount)
- [x] Cgroups resource management
- [x] Interactive REPL mode
- [x] Docker image format support
- [ ] Image pull from registries
- [ ] Network namespace support
- [ ] User namespace mapping
- [ ] Container hooks
- [ ] Volume/bind mounts
- [ ] Resource monitoring

---

## License

Apache License 2.0 - See [LICENSE](LICENSE) for details.


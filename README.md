# ncli

built to learn rust
A simple NixOS CLI helper that prompts to update repos, rebuild the system, and clean garbage.

## Usage

```bash
cargo run
```

Or use the binary directly:

```bash
./ncli-linux-x86_64
```

To use it conveniently, add a shell alias to your `~/.bashrc` or `~/.zshrc`:

```bash
alias ncli="/path/to/ncli/ncli-linux-x86_64"
```


> [!NOTE]
> This tool requires your system to use flake and the config file/flake in `/etc/nixos/` directory

I will try to fix this in future releases

# ncli

A simple NixOS CLI helper that prompts to update repos, rebuild the system, and clean garbage.

## Usage

```bash
cargo run
```

Or use the release binary directly:

```bash
./target/release/ncli
```

To use it conveniently, add a shell alias to your `~/.bashrc` or `~/.zshrc`:

```bash
alias ncli="/path/to/ncli/target/release/ncli"
```

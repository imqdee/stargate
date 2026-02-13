# Stargate

Blockchain network switcher CLI for [Foundry](https://book.getfoundry.sh/). Switch between networks and automatically set `ETH_RPC_URL` for forge/cast.

## Installation

```bash
cargo install stargate-evm
```

Add to your `~/.zshrc` (or `~/.bashrc` for bash):

```bash
eval "$(stargate init zsh)"
```

Restart your shell or run `source ~/.zshrc` to activate.

## Configuration

Set your [Alchemy](https://www.alchemy.com/) API key:

```bash
# Interactive mode (recommended - input is hidden)
stargate config set api-key

# Or pass directly (visible in shell history)
stargate config set api-key YOUR_ALCHEMY_KEY
```

Config is stored at `~/.stargate/config.toml`.

## Usage

```bash
# Switch to a network (by name, alias, or chain ID)
sg switch mainnet    # using name
sg switch arb        # aliases work too
sg switch 42161      # chain IDs work too
sg sw polygon        # using the alias

# Switch to local anvil
sg root

# See current network
sg current
sg c                # using the alias

# List all networks
sg list

# Open block explorer
sg explorer
sg explorer 0x1234...     # opens address page
sg explorer 0xabcd...     # opens transaction page
sg e                      # using the alias
```

## Supported Networks

| Network  | Aliases       | Chain ID |
| -------- | ------------- | -------- |
| mainnet  | eth, ethereum | 1        |
| polygon  | -             | 137      |
| optimism | op            | 10       |
| arbitrum | arb           | 42161    |
| base     | -             | 8453     |
| bnb      | bsc           | 56       |
| linea    | -             | 59144    |
| ink      | -             | 57073    |
| anvil    | local         | 31337    |

## Environment Variables

After running `switch <network>`, these are exported:

- `ETH_RPC_URL` - RPC endpoint (Foundry reads this automatically)
- `BLOCK_EXPLORER` - Block explorer base URL
- `STARGATE_NETWORK` - Current network name
- `STARGATE_CHAIN_ID` - Current chain ID

## Starship Integration

[Starship](https://starship.rs/) users can display the current network name in their prompt by adding this to their `starship.toml`:

```toml
# display the network name currently connected with cast
[env_var.STARGATE_NETWORK]
default = 'local'
format = "[$env_value]($style)"
```

## Developers

### Building from Source

```bash
git clone https://github.com/imqdee/stargate.git
cd stargate
cargo build --release
```

The binary will be at `target/release/stargate`.

### Local Installation

**Option 1: Install globally** (replaces any existing installation)

```bash
cargo install --path .
```

**Option 2: Test without installing**

Build and run directly:

```bash
cargo build --release
./target/release/stargate list
./target/release/stargate switch mainnet
```

Test shell integration with local build:

```bash
# Test switch output
eval "$(./target/release/stargate switch mainnet)"
echo $ETH_RPC_URL

# Test init output
./target/release/stargate init zsh

# Test full integration
eval "$(./target/release/stargate init zsh)"
sg list
```

### Running Tests

```bash
cargo test
```

### Git Hooks

This project uses [lefthook](https://github.com/evilmartians/lefthook) for git hooks.

```bash
# Install lefthook (macOS)
brew install lefthook

# Install hooks
lefthook install
```

Hooks run automatically on commit (fmt, clippy) and push (test, build).

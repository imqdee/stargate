# Stargate

Blockchain network switcher CLI for [Foundry](https://book.getfoundry.sh/). Switch between networks and automatically set `ETH_RPC_URL` for forge/cast.

## Installation

```bash
cargo install stargate-evm
```

Then install shell integration:

```bash
# Install to ~/.zshrc
stargate init zsh

# Or for bash
stargate init bash
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
# Switch to a network
sg travel mainnet
sg travel polygon
sg travel arb        # aliases work too

# Switch to local anvil
sg root

# See current network
sg current

# List all networks
sg list

# Open block explorer
sg explorer               # opens explorer homepage
sg explorer 0x1234...     # opens address page
sg explorer 0xabcd...     # opens transaction page
```

## Supported Networks

| Network  | Aliases      | Chain ID |
|----------|--------------|----------|
| mainnet  | eth, ethereum| 1        |
| polygon  | -            | 137      |
| optimism | op           | 10       |
| arbitrum | arb          | 42161    |
| base     | -            | 8453     |
| bnb      | bsc          | 56       |
| linea    | -            | 59144    |
| ink      | -            | 57073    |
| anvil    | local        | 31337    |

## Environment Variables

After running `travel <network>`, these are exported:

- `ETH_RPC_URL` - RPC endpoint (Foundry reads this automatically)
- `BLOCK_EXPLORER` - Block explorer base URL
- `STARGATE_NETWORK` - Current network name
- `STARGATE_CHAIN_ID` - Current chain ID

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
./target/release/stargate travel mainnet
```

Test shell integration with local build:

```bash
# Test travel output
eval "$(./target/release/stargate travel mainnet)"
echo $ETH_RPC_URL

# Test init (creates backup first)
cp ~/.zshrc ~/.zshrc.bak
./target/release/stargate stargate-init zsh
```

### Running Tests

```bash
cargo test
```

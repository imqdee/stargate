# Stargate

Blockchain network switcher CLI for [Foundry](https://book.getfoundry.sh/). Switch between networks and automatically set `ETH_RPC_URL` for forge/cast.

## Installation

```bash
cargo install stargate
```

Then install shell integration:

```bash
# Auto-install to ~/.zshrc
stargate init zsh --install

# Or for bash
stargate init bash --install
```

Restart your shell or run `source ~/.zshrc` to activate.

<details>
<summary>Manual installation</summary>

Add to your `~/.zshrc` (or `~/.bashrc`):

```bash
eval "$(stargate init zsh)"
```

</details>

## Configuration

Set your [Alchemy](https://www.alchemy.com/) API key:

```bash
stargate config set api-key YOUR_ALCHEMY_KEY
```

Config is stored at `~/.stargate/config.toml`.

## Usage

```bash
# Switch to a network
stargate travel mainnet
stargate travel polygon
stargate travel arb        # aliases work too

# Switch to local anvil
stargate root

# See current network
stargate current

# List all networks
stargate list

# Open block explorer
stargate explorer               # opens explorer homepage
stargate explorer 0x1234...     # opens address page
stargate explorer 0xabcd...     # opens transaction page
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

# Test full shell integration in a subshell
zsh -c 'eval "$(./target/release/stargate init zsh)" && travel arb && current'

# Test --install flag (creates backup first)
cp ~/.zshrc ~/.zshrc.bak
./target/release/stargate init zsh --install
```

### Running Tests

```bash
cargo test
```

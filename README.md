# parch

A user-friendly command-line wrapper over `paru` (and `pacman`) for Arch Linux.

```
parch install firefox   → paru -Sy firefox
parch search ripgrep    → paru -Ss ripgrep
parch list              → paru -Q
```

## Why?

`paru` and `pacman` have powerful but terse flags. `parch` wraps them with intuitive subcommand names and consistent short aliases, so you never have to remember whether it was `-S`, `-R`, `-Ss`, `-Qo`, or `-Ql` again.

## Installation

```bash
# Build from source
git clone https://github.com/your-username/parch
cd parch
cargo build --release

# Install binary and fish completions
cp target/release/parch ~/.local/bin/
mkdir -p ~/.config/fish/completions
parch completions > ~/.config/fish/completions/parch.fish
```

Or with `just`:

```bash
just install
```

## Usage

### Commands

| parch command | alias | maps to | description |
|---|---|---|---|
| `install <pkg>` | `i` | `paru -Sy` | Install package(s) |
| `uninstall <pkg>` | `u` | `paru -Rc` | Remove package(s) with cascade |
| `update` | `up` | `paru -Suy` | Update all packages (including AUR) |
| `search <query>` | `s` | `paru -Ss` | Search packages (repos + AUR) |
| `list [query]` | `l` | `paru -Q` / `-Qs` | List all installed, or filter with query |
| `whichpkg <path>` | `wp` | `paru -Qo` | Find which package owns a file |
| `filesof <pkg>` | `f` | `paru -Ql` | List all files owned by a package |
| `info <pkg>` | `show` | `paru -Qi` / `-Si` | Show package info (tries installed first) |
| `orphans` | `o` | `paru -Qdt` | List orphan/unneeded packages |
| `clean` | `c` | `paru -Sc` | Clean package cache |
| `upgrades` | `pending` | `paru -Qu` | List packages with available updates |
| `deps <pkg>` | `d` | `paru -Qi` | Show dependencies of an installed package |
| `why <pkg>` | `w` | `paru -Qi` | Show why a package is installed (reverse deps) |
| `completions` | — | — | Generate fish shell completions |

### Options

| flag | applies to | description |
|---|---|---|
| `-v`, `--verbose` | all commands | Print the `paru` command being executed (as `# paru ...`) |
| `--noconfirm` | `install`, `uninstall`, `update`, `clean` | Skip confirmation prompts |
| `--needed` | `install` | Don't reinstall up-to-date packages |

### Examples

```bash
# Install a package
parch install neovim
parch i neovim                       # same thing, shorter

# Install without prompts, skipping up-to-date packages
parch install --noconfirm --needed neovim

# See what command is being run
parch -v install neovim
# → # paru -Sy neovim

# Search for a package
parch search rust
parch s rust

# Remove a package and its unneeded dependencies
parch uninstall firefox
parch u firefox

# Update everything
parch update

# List installed packages
parch list
parch list niri                     # search installed packages

# Inspect packages
parch whichpkg /usr/bin/bash        # who owns this file?
parch filesof paru                  # what files does this package own?
parch info paru                     # show package details
parch deps paru                     # show dependencies
parch why bash                      # why is bash installed?

# Maintenance
parch orphans                       # find orphans
parch clean                         # clean cache
parch upgrades                      # check for updates
```

## Shell Completions

Fish shell completions are built in. They provide:

- **Command names** with descriptions (`install`, `uninstall`, `update`, etc.)
- **Dynamic package names** for `install`, `search`, `uninstall`, `filesof`, `info`, `deps`, `why`
- **File paths** for `whichpkg`
- **Flags** (`--noconfirm`, `--needed`, `--verbose`)

```bash
# Install fish completions
parch completions > ~/.config/fish/completions/parch.fish
# Restart fish or source: source ~/.config/fish/completions/parch.fish
```

## Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run without installing
cargo run -- install neovim
```

### Prerequisites

- Rust 2024 edition (1.85+)
- `paru` installed on the system
- `pacman` (part of Arch Linux base)

## License

MIT

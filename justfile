# ── Build ───────────────────────────────────────────────────────────────────
build:
    cargo build

release:
    cargo build --release

# ── Run ─────────────────────────────────────────────────────────────────────
run cmd="--help":
    cargo run -- {{cmd}}

# ── Completions ─────────────────────────────────────────────────────────────
# Generate fish completions file
completions:
    mkdir -p completions
    cargo run -- completions > completions/parch.fish 2>/dev/null
    @echo "✅ Generated completions/parch.fish"

# Install fish completions to user dir
install-completions: completions
    cp completions/parch.fish ~/.config/fish/completions/parch.fish
    @echo "✅ Installed to ~/.config/fish/completions/parch.fish"
    @echo "   Restart fish or run: source ~/.config/fish/completions/parch.fish"

# ── Install ─────────────────────────────────────────────────────────────────
# Build release + install binary + completions
install: release completions
    cp target/release/parch ~/.local/bin/parch
    mkdir -p ~/.config/fish/completions
    cp completions/parch.fish ~/.config/fish/completions/parch.fish
    @echo "✅ Installed parch to ~/.local/bin/parch"
    @echo "✅ Installed completions to ~/.config/fish/completions/parch.fish"

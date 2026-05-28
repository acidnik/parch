# parch completions for fish

# ── Helper: extract current token ───────────────────────────────────────────
function __fish_parch_needs_command
    set -l cmd (commandline -opc)
    set -e cmd[1]
    if test (count $cmd) -eq 0
        return 0
    end
    # Check if the next token is a known subcommand
    contains -- $cmd[1] install i uninstall u update up search s list l \
        whichpkg wp filesof f info show orphans o clean c upgrades pending \
        deps d why w completions help
    and return 1
    return 0
end

function __fish_parch_using_subcommand
    set -l cmd (commandline -opc)
    set -e cmd[1]
    contains -- $cmd[1] $argv
end

# ── Commands ────────────────────────────────────────────────────────────────
# Top-level completions
complete -c parch -f -n '__fish_parch_needs_command' -a install     -d 'Install packages (paru -Sy)'
complete -c parch -f -n '__fish_parch_needs_command' -a i           -d 'Install packages (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a uninstall   -d 'Remove packages (paru -Rc)'
complete -c parch -f -n '__fish_parch_needs_command' -a u           -d 'Remove packages (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a update      -d 'Update all packages (paru -Suy)'
complete -c parch -f -n '__fish_parch_needs_command' -a up          -d 'Update all packages (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a search      -d 'Search for packages (paru -Ss)'
complete -c parch -f -n '__fish_parch_needs_command' -a s           -d 'Search for packages (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a list        -d 'List installed packages'
complete -c parch -f -n '__fish_parch_needs_command' -a l           -d 'List installed packages (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a whichpkg   -d 'Find package owning a file (paru -Qo)'
complete -c parch -f -n '__fish_parch_needs_command' -a wp          -d 'Find package owning a file (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a filesof    -d 'List files owned by package (paru -Ql)'
complete -c parch -f -n '__fish_parch_needs_command' -a f           -d 'List files owned by package (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a info        -d 'Show package info'
complete -c parch -f -n '__fish_parch_needs_command' -a show        -d 'Show package info (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a orphans     -d 'List orphan packages'
complete -c parch -f -n '__fish_parch_needs_command' -a o           -d 'List orphan packages (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a clean       -d 'Clean package cache (paru -Sc)'
complete -c parch -f -n '__fish_parch_needs_command' -a c           -d 'Clean package cache (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a upgrades    -d 'List upgradable packages'
complete -c parch -f -n '__fish_parch_needs_command' -a pending     -d 'List upgradable packages (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a deps        -d 'Show package dependencies'
complete -c parch -f -n '__fish_parch_needs_command' -a d           -d 'Show package dependencies (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a why         -d 'Show why a package is installed'
complete -c parch -f -n '__fish_parch_needs_command' -a w           -d 'Show why a package is installed (alias)'
complete -c parch -f -n '__fish_parch_needs_command' -a completions -d 'Generate fish completions'

# ── Flags ──────────────────────────────────────────────────────────────────
complete -c parch -n '__fish_parch_using_subcommand install i' -l noconfirm -d 'Skip confirmation prompts'
complete -c parch -n '__fish_parch_using_subcommand install i' -l needed    -d "Don't reinstall up-to-date packages"
complete -c parch -n '__fish_parch_using_subcommand uninstall u' -l noconfirm -d 'Skip confirmation prompts'
complete -c parch -n '__fish_parch_using_subcommand update up' -l noconfirm -d 'Skip confirmation prompts'
complete -c parch -n '__fish_parch_using_subcommand clean c' -l noconfirm -d 'Skip confirmation prompts'

# ── Global flags ─────────────────────────────────────────────────────────────
complete -c parch -s v -l verbose -d 'Print the paru command'
# ── Dynamic: available packages (install, search) ──────────────────────────
complete -c parch -n '__fish_parch_using_subcommand install i' -a '(pacman -Slq 2>/dev/null)' -d 'Package'
complete -c parch -n '__fish_parch_using_subcommand search s' -a '(pacman -Slq 2>/dev/null)' -d 'Package'

# ── Dynamic: installed packages (uninstall, filesof, info, deps, why) ─────
complete -c parch -n '__fish_parch_using_subcommand uninstall u' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'
complete -c parch -n '__fish_parch_using_subcommand filesof f' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'
complete -c parch -n '__fish_parch_using_subcommand info show' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'
complete -c parch -n '__fish_parch_using_subcommand deps d' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'
complete -c parch -n '__fish_parch_using_subcommand why w' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'

# ── File paths (whichpkg) ─────────────────────────────────────────────────
complete -c parch -n '__fish_parch_using_subcommand whichpkg wp' -k -a '(__fish_complete_path (commandline -ct))'

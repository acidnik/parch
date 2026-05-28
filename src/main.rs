mod commands;

use clap::{Parser, Subcommand, Args};
use std::io::Write;

/// parch — a user-friendly wrapper over paru/pacman for Arch Linux
#[derive(Parser)]
#[command(name = "parch", version, about)]
struct Cli {
    /// Print the paru/pacman command being executed
    #[arg(global = true, short = 'v', long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install packages (paru -Sy)
    #[command(visible_alias = "i")]
    Install(InstallArgs),

    /// Remove packages (paru -Rc)
    #[command(visible_alias = "u")]
    Uninstall(UninstallArgs),

    /// Update all packages (paru -Suy)
    #[command(visible_alias = "up")]
    Update(UpdateArgs),

    /// Search for packages (paru -Ss)
    #[command(visible_alias = "s")]
    Search(SearchArgs),

    /// List installed packages
    #[command(visible_alias = "l")]
    List(ListArgs),

    /// Find which package owns a file (paru -Qo)
    #[command(name = "whichpkg", visible_alias = "wp")]
    WhichPkg(WhichPkgArgs),

    /// List all files owned by a package (paru -Ql)
    #[command(name = "filesof", visible_alias = "f")]
    FilesOf(FilesOfArgs),

    /// Show detailed package info
    #[command(visible_alias = "show")]
    Info(InfoArgs),

    /// List orphan packages (unneeded dependencies)
    #[command(visible_alias = "o")]
    Orphans,

    /// Clean package cache (paru -Sc)
    #[command(visible_alias = "c")]
    Clean(CleanArgs),

    /// List packages that can be upgraded
    #[command(visible_alias = "pending")]
    Upgrades,

    /// Show dependencies of an installed package
    #[command(visible_alias = "d")]
    Deps(DepsArgs),

    /// Show why a package is installed (reverse dependencies)
    #[command(visible_alias = "w")]
    Why(WhyArgs),

    /// Generate fish shell completions
    Completions,
}

#[derive(Args)]
struct InstallArgs {
    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    noconfirm: bool,

    /// Don't reinstall up-to-date packages
    #[arg(long)]
    needed: bool,

    /// Package(s) to install
    packages: Vec<String>,
}

#[derive(Args)]
struct UninstallArgs {
    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    noconfirm: bool,

    /// Package(s) to remove
    packages: Vec<String>,
}

#[derive(Args)]
struct UpdateArgs {
    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    noconfirm: bool,
}

#[derive(Args)]
struct SearchArgs {
    /// Search query
    query: Vec<String>,
}

#[derive(Args)]
struct ListArgs {
    /// Optional filter to search installed packages
    query: Option<String>,
}

#[derive(Args)]
struct WhichPkgArgs {
    /// File path(s) to look up
    paths: Vec<String>,
}

#[derive(Args)]
struct FilesOfArgs {
    /// Package(s) to inspect
    packages: Vec<String>,
}

#[derive(Args)]
struct InfoArgs {
    /// Package(s) to inspect
    packages: Vec<String>,
}

#[derive(Args)]
struct CleanArgs {
    /// Skip confirmation prompts
    #[arg(short = 'y', long)]
    noconfirm: bool,
}

#[derive(Args)]
struct DepsArgs {
    /// Package(s) to inspect
    packages: Vec<String>,
}

#[derive(Args)]
struct WhyArgs {
    /// Package(s) to inspect
    packages: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install(args) => {
            commands::set_verbose(cli.verbose);
            if args.packages.is_empty() {
                eprintln!("error: no packages specified for install");
                std::process::exit(1);
            }
            commands::install(&args.packages, args.noconfirm, args.needed);
        }
        Commands::Uninstall(args) => {
            commands::set_verbose(cli.verbose);
            if args.packages.is_empty() {
                eprintln!("error: no packages specified for uninstall");
                std::process::exit(1);
            }
            commands::uninstall(&args.packages, args.noconfirm);
        }
        Commands::Update(args) => {
            commands::set_verbose(cli.verbose);
            commands::update(args.noconfirm);
        }
        Commands::Search(args) => {
            commands::set_verbose(cli.verbose);
            commands::search(&args.query);
        }
        Commands::List(args) => {
            commands::set_verbose(cli.verbose);
            commands::list(args.query.as_deref());
        }
        Commands::WhichPkg(args) => {
            commands::set_verbose(cli.verbose);
            commands::whichpkg(&args.paths);
        }
        Commands::FilesOf(args) => {
            commands::set_verbose(cli.verbose);
            commands::filesof(&args.packages);
        }
        Commands::Info(args) => {
            commands::set_verbose(cli.verbose);
            commands::info(&args.packages);
        }
        Commands::Orphans => {
            commands::set_verbose(cli.verbose);
            commands::orphans();
        }
        Commands::Clean(args) => {
            commands::set_verbose(cli.verbose);
            commands::clean(args.noconfirm);
        }
        Commands::Upgrades => {
            commands::set_verbose(cli.verbose);
            commands::upgrades();
        }
        Commands::Deps(args) => {
            commands::set_verbose(cli.verbose);
            commands::deps(&args.packages);
        }
        Commands::Why(args) => {
            commands::set_verbose(cli.verbose);
            commands::why(&args.packages);
        }
        Commands::Completions => {
            generate_completions();
        }
    }
}

fn generate_completions() {
    let script = r#"# parch completions for fish

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

# ── Dynamic package helpers ──────────────────────────────────────────────────
function __fish_parch_available_packages
    set -l token (commandline -ct)
    if test -z "$token"
        # No token yet — list all repo packages (fast)
        command pacman -Slq 2>/dev/null
    else
        # Search both repos AND AUR via paru
        command paru -Ssq -- "$token" 2>/dev/null
    end
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
complete -c parch -n '__fish_parch_using_subcommand install i' -s y -l noconfirm -d 'Skip confirmation prompts'
complete -c parch -n '__fish_parch_using_subcommand install i' -l needed    -d "Don't reinstall up-to-date packages"
complete -c parch -n '__fish_parch_using_subcommand uninstall u' -s y -l noconfirm -d 'Skip confirmation prompts'
complete -c parch -n '__fish_parch_using_subcommand update up' -s y -l noconfirm -d 'Skip confirmation prompts'
complete -c parch -n '__fish_parch_using_subcommand clean c' -s y -l noconfirm -d 'Skip confirmation prompts'

# ── Global flags ─────────────────────────────────────────────────────────────
complete -c parch -s v -l verbose -d 'Print the paru command'
# ── Dynamic: available packages with AUR support (install, search) ────────
complete -c parch -n '__fish_parch_using_subcommand install i' -a '(__fish_parch_available_packages)' -d 'Package'
complete -c parch -n '__fish_parch_using_subcommand search s' -a '(__fish_parch_available_packages)' -d 'Package'

# ── Dynamic: installed packages (uninstall, filesof, info, deps, why) ─────
complete -c parch -n '__fish_parch_using_subcommand uninstall u' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'
complete -c parch -n '__fish_parch_using_subcommand filesof f' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'
complete -c parch -n '__fish_parch_using_subcommand info show' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'
complete -c parch -n '__fish_parch_using_subcommand deps d' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'
complete -c parch -n '__fish_parch_using_subcommand why w' -a '(pacman -Qq 2>/dev/null)' -d 'Installed'

# ── File paths (whichpkg) ─────────────────────────────────────────────────
complete -c parch -n '__fish_parch_using_subcommand whichpkg wp' -k -a '(__fish_complete_path (commandline -ct))'
"#;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", script).expect("failed to write completions");
    handle.flush().expect("failed to flush completions");
}

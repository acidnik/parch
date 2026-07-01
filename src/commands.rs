use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(v: bool) {
    VERBOSE.store(v, Ordering::Relaxed);
}

fn paru() -> Command {
    Command::new("paru")
}

fn debug_cmd(cmd: &Command) {
    if VERBOSE.load(Ordering::Relaxed) {
        let prog = cmd.get_program().to_string_lossy();
        let args: Vec<String> = cmd.get_args().map(|a| a.to_string_lossy().to_string()).collect();
        eprintln!("# {} {}", prog, args.join(" "));
    }
}

fn run(cmd: &mut Command) {
    debug_cmd(cmd);
    let status = cmd.status().expect("failed to execute paru");
    std::process::exit(status.code().unwrap_or(1));
}

fn run_with_output(cmd: &mut Command) -> String {
    debug_cmd(cmd);
    let output = cmd.output().expect("failed to execute paru");
    if !output.status.success() {
        eprintln!(
            "error: paru failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(output.status.code().unwrap_or(1));
    }
    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Show package info — try installed (`-Qi`) first, fall back to remote (`-Si`).
fn show_pkg_info(pkg: &str) {
    let installed = run_with_output(paru().args(["-Qi", pkg]));
    if !installed.is_empty() && !installed.contains("error:") {
        print!("{}", installed);
    } else {
        // Try remote info
        let remote = run_with_output(paru().args(["-Si", pkg]));
        print!("{}", remote);
    }
}

pub fn install(packages: &[String], noconfirm: bool, needed: bool) {
    let mut cmd = paru();
    cmd.arg("-Sy");
    if noconfirm {
        cmd.arg("--noconfirm");
    }
    if needed {
        cmd.arg("--needed");
    }
    cmd.args(packages);
    run(&mut cmd);
}

pub fn uninstall(packages: &[String], noconfirm: bool) {
    let mut cmd = paru();
    cmd.arg("-Rc");
    if noconfirm {
        cmd.arg("--noconfirm");
    }
    cmd.args(packages);
    run(&mut cmd);
}

pub fn update(noconfirm: bool) {
    let mut cmd = paru();
    cmd.arg("-Suy");
    if noconfirm {
        cmd.arg("--noconfirm");
    }
    run(&mut cmd);
}

pub fn search(query: &[String]) {
    let mut cmd = paru();
    cmd.arg("-Ss");
    cmd.args(query);
    run(&mut cmd);
}

fn build_repo_map() -> std::collections::HashMap<String, String> {
    let output = Command::new("pacman")
        .args(["-Sl"])
        .output()
        .expect("failed to run pacman -Sl");
    let mut map = std::collections::HashMap::new();
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        // Format: "repo pkgname pkgver ..."
        let mut parts = line.split_whitespace();
        if let (Some(repo), Some(pkg)) = (parts.next(), parts.next()) {
            map.insert(pkg.to_string(), repo.to_string());
        }
    }
    map
}

pub fn list(query: Option<&str>, repo: bool) {
    if repo {
        let repo_map = build_repo_map();

        let output = match query {
            Some(q) => run_with_output(paru().args(["-Qs", q])),
            None => run_with_output(paru().arg("-Q")),
        };

        for line in output.lines() {
            // Skip description lines (indented) and empty lines
            if line.is_empty() || line.starts_with(' ') {
                continue;
            }
            // Strip 'local/' prefix added by -Qs
            let clean = line.strip_prefix("local/").unwrap_or(line);
            let pkg_name = clean.split_whitespace().next().unwrap_or(clean);
            let repo_name = repo_map.get(pkg_name).map(|s| s.as_str()).unwrap_or("aur");
            println!("{}/{}", repo_name, clean);
        }
    } else {
        let mut cmd = paru();
        match query {
            Some(q) => {
                cmd.arg("-Qs");
                cmd.arg(q);
            }
            None => {
                cmd.arg("-Q");
            }
        }
        run(&mut cmd);
    }
}

pub fn whichpkg(paths: &[String], all: bool) {
    if all {
        let pkgfile_path = which::which("pkgfile");
        match pkgfile_path {
            Ok(_) => {
                for path in paths {
                    let mut cmd = Command::new("pkgfile");
                    cmd.arg(path);
                    run(&mut cmd);
                }
            }
            Err(_) => {
                eprintln!("warning: pkgfile is not installed");
                eprintln!("  install: paru -S pkgfile && paru -Su -- pkgfile");
                eprintln!("  then update file list: pkgfile -u");
                std::process::exit(1);
            }
        }
    } else {
        let mut cmd = paru();
        cmd.arg("-Qo");
        cmd.args(paths);
        run(&mut cmd);
    }
}

pub fn filesof(packages: &[String]) {
    let mut cmd = paru();
    cmd.arg("-Ql");
    cmd.args(packages);
    run(&mut cmd);
}

pub fn info(packages: &[String]) {
    for pkg in packages {
        show_pkg_info(pkg);
    }
}

pub fn orphans() {
    let mut cmd = paru();
    cmd.args(["-Qdt"]);
    run(&mut cmd);
}

pub fn clean(noconfirm: bool) {
    let mut cmd = paru();
    cmd.arg("-Sc");
    if noconfirm {
        cmd.arg("--noconfirm");
    }
    run(&mut cmd);
}

pub fn upgrades() {
    let mut cmd = paru();
    cmd.arg("-Qu");
    run(&mut cmd);
}

pub fn deps(packages: &[String]) {
    for pkg in packages {
        let output = run_with_output(paru().args(["-Qi", pkg]));
        for line in output.lines() {
            if line.starts_with("Depends On") || line.starts_with("Optional Deps") {
                println!("{}", line);
            }
        }
    }
}

pub fn why(packages: &[String]) {
    for pkg in packages {
        let output = run_with_output(paru().args(["-Qi", pkg]));
        for line in output.lines() {
            if line.starts_with("Required By") {
                println!("{}", line);
            }
        }
    }
}

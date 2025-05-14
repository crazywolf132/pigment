// build.rs
//
// *Only* runs when you set PIGMENT_REGEN=1.
// Creates .pigment-venv/ inside OUT_DIR (target/**/build/…)
// Installs requests, beautifulsoup4, tqdm if not present.
// Runs tools/scrape.py with that venv’s interpreter.
// Finally, asks Cargo to re-compile if the generated file changed.
//
// Note: The generated/colors.rs file is included in the published package.
// If you need to regenerate it, set PIGMENT_REGEN=1 before building.

use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const PY_PKGS: &[&str] = &["requests", "beautifulsoup4", "tqdm"];

fn main() {
    let regen = env::var_os("PIGMENT_REGEN").is_some();
    let _table = Path::new("generated/colors.rs");
    if !regen {
        println!("cargo:rerun-if-changed=generated/colors.rs");
        return;
    }

    // -------- 1. locate python3 --------------------------------------------
    let python = which::which("python3").expect("python3 not found in PATH");

    // -------- 2. create / reuse a venv --------------------------------------
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let venv_dir = out_dir.join(".pigment-venv");
    let bin = |name: &str| {
        if cfg!(target_os = "windows") {
            venv_dir.join("Scripts").join(format!("{name}.exe"))
        } else {
            venv_dir.join("bin").join(name)
        }
    };

    if !venv_dir.exists() {
        run(Command::new(&python).args(["-m", "venv", venv_dir.to_str().unwrap()]));
    }

    // Ensure pip exists (some distros ship venv w/out pip)
    run(Command::new(&python).args([
        "-m",
        "ensurepip",
        "--upgrade",
        "--root",
        venv_dir.to_str().unwrap(), // macOS needs this
    ]));

    // -------- 3. install deps if missing ------------------------------------
    let pip = bin("pip");
    // quick check: does `pip show requests` succeed?
    let needs_install = !Command::new(&pip)
        .args(["show", "requests"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if needs_install {
        let mut cmd = Command::new(&pip);
        cmd.arg("install").args(PY_PKGS);
        // Comment out if you *do* want global mirrors:
        // cmd.arg("--no-index").arg("--find-links").arg("…/wheels");
        run(&mut cmd);
    }

    // -------- 4. run the scraper -------------------------------------------
    let py = bin("python");
    run(Command::new(py).arg("tools/scrape.py"));

    // tell Cargo to watch the generated file
    println!("cargo:rerun-if-changed=generated/colors.rs");
}

// Helper: run a command and panic on failure
fn run(cmd: &mut Command) {
    let status = cmd.status().expect("failed to spawn process");
    if !status.success() {
        panic!("command failed: {cmd:?}");
    }
}

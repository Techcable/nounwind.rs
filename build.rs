use std::ffi::OsString;
use std::process::{self, Command};
use std::{env, str};

pub fn main() {
    let rustc = match rustc_minor_version() {
        Some(x) => x,
        None => return,
    };

    if rustc >= 80 {
        println!("cargo:rustc-check-cfg=cfg(nounwind_extern_c_will_abort)");
    }

    if rustc >= 81 {
        println!("cargo:rustc-cfg=nounwind_extern_c_will_abort");
    }
}

// Copied from anyhow@1.0.100/build.rs: <https://github.com/dtolnay/anyhow/blob/1.0.100/build.rs#L213-L232>
// This has the same license that we do (MIT OR APACHE-2.0)
fn rustc_minor_version() -> Option<u32> {
    let rustc = cargo_env_var("RUSTC");
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    pieces.next()?.parse().ok()
}

fn cargo_env_var(key: &str) -> OsString {
    env::var_os(key).unwrap_or_else(|| {
        eprintln!(
            "Environment variable ${} is not set during execution of build script",
            key,
        );
        process::exit(1);
    })
}

use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn show_toplevel() -> Option<PathBuf> {
    let mut cmd = Command::new("git");
    cmd.arg("rev-parse")
        .arg("--show-toplevel")
        .stderr(Stdio::null())
        .stdout(Stdio::piped());
    let child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => return None,
    };
    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(_) => return None,
    };
    if !output.status.success() {
        return None;
    }

    let mut output = output.stdout;
    if output.last().copied() == Some(b'\n') {
        output.pop();
    }
    Some(PathBuf::from(OsString::from_vec(output)))
}

pub fn show_prefix() -> anyhow::Result<PathBuf> {
    let mut cmd = Command::new("git");
    cmd.arg("rev-parse")
        .arg("--show-prefix")
        .stderr(Stdio::null())
        .stdout(Stdio::piped());
    let output = cmd.spawn()?.wait_with_output()?;
    if !output.status.success() {
        anyhow::bail!("git rev-parse --show-prefix failed");
    }

    let mut output = output.stdout;
    if output.last().copied() == Some(b'\n') {
        output.pop();
    }
    Ok(PathBuf::from(OsString::from_vec(output)))
}

pub fn get_branch(repository_root: &Path) -> anyhow::Result<GitHead> {
    let head_path = {
        let mut p = repository_root.join(".git");
        p.push("HEAD");
        p
    };
    let mut contents = fs::read_to_string(head_path)?;
    if !contents.ends_with('\n') {
        anyhow::bail!("HEAD didn't end in NL?");
    }
    contents.pop();
    if let Some(gref) = contents.strip_prefix("ref: ") {
        if let Some(branch) = gref.strip_prefix("refs/heads/") {
            Ok(GitHead::Branch(branch.to_owned()))
        } else {
            anyhow::bail!("HEAD is a ref, but not a branch?")
        }
    } else {
        Ok(GitHead::Detached(contents))
    }
}

pub enum GitHead {
    Branch(String),
    Detached(String),
}

/*
pub fn get_branch() -> Result<String, ()> {
    let mut cmd = Command::new("git");
    cmd
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .stderr(Stdio::null())
        .stdout(Stdio::piped());
    let child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => return Err(()),
    };
    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(_) => return Err(()),
    };

    let mut output = output.stdout;
    if output.last().copied() == Some(b'\n') {
        output.pop();
    }
    Ok(String::from_utf8_lossy(&output).into_owned())
}
*/

use std::ffi::{CStr, OsString};
use std::fmt;
use std::io::{self, Write};
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

use nix::sys::signal::Signal;

pub fn prompt(args: &[&CStr]) -> Result<(), i32> {
    let args = parse_args(args)?;
    let uid_and_host = get_uid_and_host(args.default_username);
	let location = get_location();
    let last_exit_status = last_command_exit_status(args.last_exit_status);
	let key_mode_t = key_mode(args.vi_mode);
    let cmd_sym = cmd_symbol();
	print!("{uid_and_host}{location}\n{last_exit_status}{key_mode_t}{cmd_sym} ");
    io::stdout().flush().unwrap();
    Ok(())
}

struct Args<'a> {
    last_exit_status: i32,
    default_username: &'a CStr,
    vi_mode: bool,
}

fn parse_args<'a>(args: &[&'a CStr]) -> Result<Args<'a>, i32> {
    if args.len() != 3 {
        eprintln!(
            "Usage:\n\
             \t_rust-prompt-alpha \"$?\" \"$_PROMPT_ALPHA_DEFAULT_USER\" \"emacs|vi\""
        );
        return Err(1);
    }
    let last_exit_status = {
        let arg = args[0];
        let arg = match arg.to_str() {
            Ok(s) => s,
            Err(_err) => {
                eprintln!("Could not covert $1 to &str");
                return Err(1);
            }
        };
        match arg.parse::<i32>() {
            Ok(v) => v,
            Err(_err) => {
                eprintln!("Could not convert $1 to integer");
                return Err(1);
            }
        }
    };
    let default_username = args[1];
    // Note: match doesn't like CStrs.
    let vi_mode = if args[2] == c"emacs" {
        false
    } else if args[2] == c"vi" {
        true
    } else {
        eprintln!("The emacs|vi arg must be emacs, or vi.");
        return Err(1);
    };

    Ok(Args {
        last_exit_status,
        default_username,
        vi_mode,
    })
}

fn get_uid_and_host(default_username: &CStr) -> UidAndHost {
    let uid = if nix::unistd::getuid().is_root() {
        PromptUid::Root
    } else {
        let zsh_username = unsafe { crate::zsh::get_string_param(c"USERNAME") };
        if zsh_username != Some(default_username) {
            PromptUid::OtherUser
        } else {
            PromptUid::Normal
        }
    };

    let ssh_connection = unsafe { crate::zsh::get_string_param(c"SSH_CONNECTION") };
    let via_ssh = ssh_connection.is_some();

    UidAndHost {
        uid,
        via_ssh,
    }
}

struct UidAndHost {
    uid: PromptUid,
    via_ssh: bool,
}

impl fmt::Display for UidAndHost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output_at = !matches!(self.uid, PromptUid::Normal) || self.via_ssh;

        match self.uid {
            PromptUid::Root => write!(f, "%B%F{{red}}%n%f%b")?,
            PromptUid::Normal => (),
            PromptUid::OtherUser => write!(f, "%B%F{{yellow}}%n%f%b")?,
        }

        if output_at {
            let color = match self.uid {
                PromptUid::Normal => "green",
                _ => "yellow",
            };
            write!(f, "%F{{{color}}}@%f")?;
        }

        if self.via_ssh {
            write!(f, "%B%F{{green}}%M%f%b")?;
        }

        if output_at {
            write!(f, " ")?;
        }
        Ok(())
    }
}

enum PromptUid {
    Root,
    Normal,
    OtherUser,
}

fn get_location() -> Location {
    let git_root = crate::git::show_toplevel();

    if let Some(git_root) = git_root {
        let branch = crate::git::get_branch(&git_root);

        let prefix = crate::git::show_prefix();
        let prefix = match prefix {
            Ok(pr) => {
                let mut bytes = pr.into_os_string().into_vec();
                if bytes.last().copied() == Some(b'/') {
                    bytes.pop();
                }
                Ok(PathBuf::from(OsString::from_vec(bytes)))
            }
            Err(err) => Err(err),
        };

        let repo = match git_root.file_name() {
            Some(fname) => fname.to_string_lossy().into_owned(),
            None => "???".to_owned(),
        };

        Location::Git {
            repo,
            branch: Branch(branch),
            prefix,
        }
    } else {
        Location::NotGit
    }
}

struct Branch(anyhow::Result<crate::git::GitHead>);

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Ok(crate::git::GitHead::Branch(b)) => write!(f, "%F{{green}}{b}%f"),
            Ok(crate::git::GitHead::Detached(s)) => write!(f, "%F{{yellow}}(detached HEAD: {s})%f"),
            Err(_err) => write!(f, "%B%F{{red}}(err)%f%b"),
        }
    }
}

enum Location {
    Git {
        repo: String,
        branch: Branch,
        prefix: anyhow::Result<PathBuf>,
    },
    NotGit,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Git { repo, branch, prefix } => {
                write!(f, "%F{{green}}±%f %B{repo}%b:{branch}:")?;
                match prefix {
                    Ok(p) => {
                        let lossy = p.to_string_lossy();
                        write!(f, "%B%F{{blue}}/{lossy}%f%b")?;
                    }
                    Err(_) => write!(f, "%B%F{{red}}(err)%f%b")?,
                }
                Ok(())
            }
            Location::NotGit => write!(f, "%B%F{{blue}}%~%f%b"),
        }
    }
}

/// Prints an indicator of what mode we're in: vi-normal, vi-insert, or emacs.
fn key_mode(vi_mode: bool) -> KeyMode {
    let keymap = unsafe { crate::zsh::get_string_param(c"KEYMAP") };
    if keymap == Some(c"main") {
        if vi_mode {
            KeyMode::Insert
        } else {
            KeyMode::Emacs
        }
    } else if keymap == Some(c"vicmd") {
        KeyMode::Normal
    } else {
        let as_string = match keymap {
            Some(v) => v.to_string_lossy().into_owned(),
            None => "< $KEYMAP unset >".to_owned(),
        };
        KeyMode::Unknown(as_string)
    }
}

enum KeyMode {
    Emacs,
    Insert,
    Normal,
    Unknown(String),
}

impl fmt::Display for KeyMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyMode::Emacs => Ok(()),
            KeyMode::Insert => write!(f, "[%{{\x1b[92m%}}i%{{\x1b[0m%}}] "),
            KeyMode::Normal => write!(f, "[%{{\x1b[93m%}}n%{{\x1b[0m%}}] "),
            KeyMode::Unknown(map) => write!(f, "[? ({map})] "),
        }
    }
}

fn last_command_exit_status(exit_status: i32) -> LastCommandExit {
    if exit_status == 0 {
        return LastCommandExit::Success;
    } else if 128 <= exit_status {
        if let Ok(sig) = Signal::try_from(exit_status - 128) {
            return LastCommandExit::Signal(sig);
        }
    }

    LastCommandExit::Error
}

enum LastCommandExit {
    Success,
    Error,
    Signal(Signal),
}

impl fmt::Display for LastCommandExit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LastCommandExit::Success => Ok(()),
            LastCommandExit::Error => write!(f, "%B%F{{red}}(last command returned %?.%)%f%b\n"),
            LastCommandExit::Signal(signal) => {
                write!(f, "%B%F{{red}}(last command got signal {})%f%b\n", signal.as_str())
            }
        }
    }
}

/// Emit either a red '#' if we're root, or a blue '»' otherwise.
fn cmd_symbol() -> CmdSymbol {
    let uid = nix::unistd::getuid();
    if uid.is_root() {
        CmdSymbol::Root
    } else {
        CmdSymbol::NotRoot
    }
}

enum CmdSymbol {
    Root,
    NotRoot,
}

impl fmt::Display for CmdSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmdSymbol::Root => write!(f, "%F{{red}}%B#%b%f"),
            CmdSymbol::NotRoot => write!(f, "%{{\x1b[1;38;5;033m%}}»%{{\x1b[0m%}}"),
        }
    }
}

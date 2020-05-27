use serde::Deserialize;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;

const CONF_FILE_NAME: &str = "switsshfs.toml";
const MTAB_PATH: &str = "/etc/mtab";

#[derive(Debug, Deserialize)]
struct Config {
    remote: String,
}

#[derive(Debug)]
enum SwitchMode {
    MOUNT,
    UNMOUNT,
}

fn assert_cmd_exists(cmd_str: &str) {
    let cmd = &mut Command::new(cmd_str);

    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    if let Err(error) = cmd.status() {
        match error.kind() {
            NotFound => eprintln!("Command '{}' not found.", cmd_str),
            _ => eprintln!(
                "Error while assert that the command '{}' exists: {}",
                cmd_str, error
            ),
        }

        exit(1);
    }
}

fn mount(mountpoint: &str) {
    let conf_path: PathBuf = [mountpoint, CONF_FILE_NAME].iter().collect();

    let toml_config = fs::read_to_string(conf_path).expect(&format!(
        "Could not read {} config from '{}'",
        CONF_FILE_NAME, mountpoint
    ));
    let config: Config = toml::from_str(&toml_config).expect("Invalid switsshfs configuration");

    let status = Command::new("sshfs")
        .arg(config.remote)
        .arg(mountpoint)
        .arg("-o")
        .arg("nonempty,idmap=user")
        .status()
        .expect("Could not run sshfs: please check your sshfs installation.");

    if !status.success() {
        exit(status.code().unwrap());
    }
}

fn unmount(mountpoint: &str) {
    let status = Command::new("fusermount")
        .arg("-u")
        .arg(mountpoint)
        .status()
        .expect("Could not run fusermount: please check your fuse installation.");

    if !status.success() {
        exit(status.code().unwrap());
    }
}

fn detect_mode(mountpoint: &str) -> SwitchMode {
    let file = File::open(MTAB_PATH).expect("Could not open mtab file");
    let is_mounted = BufReader::new(file)
        .lines()
        .any(fuse_mountpoint_matcher(mountpoint));

    if is_mounted {
        SwitchMode::UNMOUNT
    } else {
        SwitchMode::MOUNT
    }
}

fn fuse_mountpoint_matcher(mountpoint: &str) -> impl FnMut(Result<String, Error>) -> bool {
    let mountpoint = fs::canonicalize(mountpoint).expect("Invalid mountpoint");
    let mountpoint = mountpoint.to_str().unwrap().to_string();

    move |mtab_line: Result<String, Error>| -> bool {
        // Example mtab line:
        // user@host:directory/path /path/to/mountpoint fuse.sshfs rw,nosuid,nodev,relatime,user_id=1000,group_id=1000 0 0

        let mtab_line = mtab_line.expect("Could not read mtab line");
        let parts: Vec<&str> = mtab_line.split(" ").collect();

        parts[2] == "fuse.sshfs" && parts[1] == mountpoint
    }
}

fn main() {
    assert_cmd_exists("sshfs");
    assert_cmd_exists("fusermount");

    let mountpoint = &args().nth(1).unwrap_or(".".to_string());

    match detect_mode(mountpoint) {
        SwitchMode::MOUNT => mount(mountpoint),
        SwitchMode::UNMOUNT => unmount(mountpoint),
    }
}

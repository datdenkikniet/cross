use std::str::FromStr;
use std::{env, path::PathBuf};

use crate::cargo::Subcommand;
use crate::rustc::TargetList;
use crate::Target;

#[derive(Debug)]
pub struct Args {
    pub all: Vec<String>,
    pub subcommand: Option<Subcommand>,
    pub channel: Option<String>,
    pub target: Option<Target>,
    pub target_dir: Option<PathBuf>,
    pub docker_in_docker: bool,
    pub manifest_path: Option<PathBuf>,
}

pub fn parse(target_list: &TargetList) -> Args {
    let mut channel = None;
    let mut target = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut target_dir = None;
    let mut sc = None;
    let mut all: Vec<String> = Vec::new();

    {
        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            if arg.is_empty() {
                continue;
            }
            if arg == "--manifest-path" {
                all.push(arg);
                if let Some(m) = args.next() {
                    let p = PathBuf::from(&m);
                    all.push(m);
                    manifest_path = env::current_dir().ok().map(|cwd| cwd.join(p));
                }
            } else if arg.starts_with("--manifest-path=") {
                manifest_path = arg
                    .splitn(2, '=')
                    .nth(1)
                    .map(PathBuf::from)
                    .and_then(|p| env::current_dir().ok().map(|cwd| cwd.join(p)));
                all.push(arg);
            } else if let ("+", ch) = arg.split_at(1) {
                channel = Some(ch.to_string());
            } else if arg == "--target" {
                all.push(arg);
                if let Some(t) = args.next() {
                    target = Some(Target::from(&t, target_list));
                    all.push(t);
                }
            } else if arg.starts_with("--target=") {
                target = arg
                    .splitn(2, '=')
                    .nth(1)
                    .map(|s| Target::from(&*s, target_list));
                all.push(arg);
            } else if arg == "--target-dir" {
                all.push(arg);
                if let Some(td) = args.next() {
                    target_dir = Some(PathBuf::from(&td));
                    all.push("/target".to_string());
                }
            } else if arg.starts_with("--target-dir=") {
                if let Some(td) = arg.splitn(2, '=').nth(1) {
                    target_dir = Some(PathBuf::from(&td));
                    all.push(format!("--target-dir=/target"));
                }
            } else {
                if !arg.starts_with('-') && sc.is_none() {
                    sc = Some(Subcommand::from(arg.as_ref()));
                }

                all.push(arg.to_string());
            }
        }
    }

    let docker_in_docker = env::var("CROSS_DOCKER_IN_DOCKER")
        .map(|s| bool::from_str(&s).unwrap_or_default())
        .unwrap_or_default();

    Args {
        all,
        subcommand: sc,
        channel,
        target,
        target_dir,
        docker_in_docker,
        manifest_path,
    }
}

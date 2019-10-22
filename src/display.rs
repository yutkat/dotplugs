use crate::git::{GitStatus, UpdateStatus};
use colored::Colorize;

pub fn display(statues: &Vec<GitStatus>) {
    for s in statues {
        if s.status == UpdateStatus::Already {
            continue;
        }
        println!("{} {}", s.uri, format!("{:?}", s.status).red());
    }
}

use crate::git::{GitStatus, UpdateStatus};
use colored::Colorize;

pub fn display(statuses: &Vec<GitStatus>) {
    for s in statuses {
        if s.status == UpdateStatus::Required {
            println!("{} {}", s.uri, format!("{:?}", s.status).red());
        }
    }
}

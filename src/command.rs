use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;
use std::{convert::TryFrom, fmt::Display};

use crate::{error::CliError, parser::*};

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    AdminCancelTask {
        address: Pubkey,
    },
    Blocktime,
    ConfigGet,
    ConfigSetMinRecurr {
        new_value: i64,
    },
    DaemonData,
    DaemonNew,
    HealthCheck,
    TaskData {
        address: Pubkey,
    },
    TaskNew {
        exec_at: Option<i64>,
        stop_at: Option<i64>,
        recurr: Option<i64>,
    },
}

impl Display for CliCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliCommand::AdminCancelTask { address } => write!(f, "admin cancel {}", address),
            CliCommand::Blocktime => write!(f, "blocktime"),
            CliCommand::ConfigGet => write!(f, "config"),
            CliCommand::ConfigSetMinRecurr { new_value } => {
                write!(f, "config set min_recurr {}", new_value)
            }
            CliCommand::DaemonData => write!(f, "daemon"),
            CliCommand::DaemonNew => write!(f, "daemon new"),
            CliCommand::HealthCheck => write!(f, "health"),
            CliCommand::TaskData { address } => write!(f, "task {}", address),
            CliCommand::TaskNew { .. } => write!(f, "task new"),
        }
    }
}

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = CliError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("admin", matches)) => admin_command(matches),
            Some(("blocktime", _matches)) => Ok(CliCommand::Blocktime {}),
            Some(("config", matches)) => config_command(matches),
            Some(("daemon", matches)) => daemon_command(matches),
            Some(("health", matches)) => health_command(matches),
            Some(("task", matches)) => task_command(matches),
            _ => Err(CliError::CommandNotRecognized(
                matches.subcommand().unwrap().0.into(),
            )),
        }
    }
}

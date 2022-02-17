use clap::{App, Arg};

pub fn app() -> App<'static> {
    App::new("admin")
        .about("Run admin instructions against Cronos")
        .subcommand(cancel_task_app())
        .subcommand(schedule_health_check_app())
}

fn schedule_health_check_app() -> App<'static> {
    App::new("health").about("Schedules a new health check app")
}

fn cancel_task_app() -> App<'static> {
    App::new("cancel").about("Cancels a scheduled task").arg(
        Arg::new("address")
            .index(1)
            .takes_value(true)
            .help("A task address"),
    )
}

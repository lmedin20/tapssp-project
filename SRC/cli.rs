use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(name = "tapssp", about = "CSV filter, stats, visualization, and SQL export")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    Filter {
        file: String,
        #[arg(long)]
        filter: String,
    },
    Stats {
        file: String,
        #[arg(long)]
        col: String,
        #[arg(long)]
        filter: Option<String>,
    },
    Viz {
        file: String,
        #[arg(long)]
        col: String,
        #[arg(long, default_value_t = 10)]
        bins: usize,
    },

    Bench {
        file: String,
    },
    SqlExport {
        csv_file: String,
        db_file: String,
    },
    SqlQuery {
        db_file: String,
        #[arg(long)]
        query: String,
    }
}








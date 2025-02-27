use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ClArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        template: Option<String>,
    },
    Run {
        #[arg(trailing_var_arg = true)]
        extra_args: Vec<String>,
    },
    Add {
        /// plugin name
        plugin: String
    }
}

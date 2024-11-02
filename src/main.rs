mod bolt;
mod gcc;
mod llvm_pgo;
mod constants;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "TrufC")]
#[command(version = "0.0.1")]
#[command(about = "PGO without the hassle.", long_about = None)]
struct CliCommand {
    #[command(subcommand)]
    command: CliArgs,
}

#[derive(Subcommand, Debug)]
enum CliArgs {
    Gcc { 
        #[arg(long="out", short='o')]
        output_filepath: String,

        #[arg(required=true)]
        target: String,
    },
    Bolt { 
        #[arg(long="out", short='o')]
        output_filepath: String,

        #[arg(required=true)]
        target: String,
    },
    LlvmPgo { 
        #[arg(long="out", short='o')]
        output_filepath: String,

        #[arg(required=true)]
        target: String,
    },
}
 

fn main() {
    let cli = CliCommand::parse();

    match &cli.command {
        CliArgs::Gcc {output_filepath, target} => {
            gcc::build_c(target, output_filepath).unwrap();
        },
        CliArgs::Bolt {output_filepath, target} => {
            bolt::compile_bolt(target, output_filepath).unwrap();
        },
        CliArgs::LlvmPgo {output_filepath, target} => {
            llvm_pgo::compile_llvm_pgo(target, output_filepath).unwrap();
        },
    }
}

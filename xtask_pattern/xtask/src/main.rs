use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Sqlx(sqlx_cli::Opt),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Sqlx(opt) => sqlx_cli::run(opt).await?,
    }
    Ok(())
}

mod add;
mod include;
mod info;
mod list;
mod remove;
mod use_config;
mod user;
mod util;

use clap::{Parser, Subcommand};
use user::UserOperator;

#[derive(Parser)]
#[command(version, about, long_about = None, styles = util::get_clap_styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(add::AddArgs),
    Remove(remove::RemoveArgs),
    List(list::RemoveArgs),
    Use(use_config::UseArgs),
    Include(include::IncludeArgs),
    Info(info::InfoArgs),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut user_operator = UserOperator::new().await;

    match &cli.command {
        Commands::Add(args) => {
            add::add(args, &mut user_operator).await?;
        }
        Commands::Remove(args) => {
            remove::remove(args, &mut user_operator);
        }
        Commands::List(_) => {
            list::list_users(&user_operator).await;
        }
        Commands::Use(args) => {
            use_config::use_config(args, &user_operator).await?;
        }
        Commands::Include(args) => {
            include::include(args, &mut user_operator).await?;
        }
        Commands::Info(args) => {
            info::info(args).await?;
        }
    }

    user_operator.sync_config().await?;

    Ok(())
}

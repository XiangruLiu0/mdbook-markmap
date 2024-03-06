use clap::{Parser, Subcommand};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_markmap::MarkmapPreprocessor;
use tracing::{debug, info};

fn init_tracing() {
    // Initialize tracing here with debug level logging
    let tracing_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("Failed to set global default tracing subscriber");
}

#[derive(Parser, Debug)]
struct Command {
    #[clap(subcommand)]
    sub: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(name = "supports")]
    Supports(Supports),
}

#[derive(Parser, Debug)]
struct Supports {
    // #[clap(long)]
    renderer: String,
}

fn main() -> mdbook::errors::Result<()> {
    init_tracing();

    let cmd = Command::parse();

    if let Some(sub_cmd) = cmd.sub {
        match sub_cmd {
            SubCommand::Supports(cmd) => {
                info!(cmd.renderer, "Support command called");
                std::process::exit(0);
            }
        }
    } else {
        debug!("No subcommand provided, running default preprocessor");

        let (ctx, book) = CmdPreprocessor::parse_input(std::io::stdin())?;
        let preprocessor = MarkmapPreprocessor;
        let book = preprocessor.run(&ctx, book)?;

        serde_json::to_writer(std::io::stdout(), &book)?;
    }
    Ok(())
}

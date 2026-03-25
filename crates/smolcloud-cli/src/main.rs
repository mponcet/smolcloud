use models::notes::{Note, NoteId};
use smolcloud_api::BaseClient;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get(GetArgs),
    List,
    Create(CreateArgs),
    Update(UpdateArgs),
    Delete(DeleteArgs),
}

#[derive(Args)]
struct GetArgs {
    id: String,
}

#[derive(Args)]
struct CreateArgs {
    title: String,
    content: String,
}

#[derive(Args)]
struct UpdateArgs {
    id: String,
    title: String,
    content: String,
}

#[derive(Args)]
struct DeleteArgs {
    id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = BaseClient::try_new("http://localhost:8787")?;
    let api = client.notes_api();

    match &cli.command {
        Commands::Get(args) => {
            let id = NoteId::try_parse(&args.id)?;
            let note = api.get(id).await?;
            println!("{}:\n{}", note.title, note.content.unwrap_or_default());
        }
        Commands::List => {
            let notes = api.get_all().await?;
            for note in &notes {
                println!("{}: {}", note.id, note.title);
            }
        }
        Commands::Create(args) => {
            let note = Note {
                title: args.title.clone(),
                content: Some(args.content.clone()),
            };
            let id = api.create(note).await?;
            println!("note created: {id}");
        }
        Commands::Update(args) => {
            let id = NoteId::try_parse(&args.id)?;
            let note = Note {
                title: args.title.clone(),
                content: Some(args.content.clone()),
            };
            api.update(id, note).await?;
            println!("note updated: {id}");
        }
        Commands::Delete(args) => {
            let id = NoteId::try_parse(&args.id)?;
            api.delete(id).await?;
        }
    }

    Ok(())
}

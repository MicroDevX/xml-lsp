use clap::Parser;
use tower_lsp::{LspService, Server};

#[derive(Parser, Debug)]
#[command(
    name = "xml-lsp",
    version = "0.1.0",
    about = "A blazing fast XML Language Server"
)]
struct Args {
    /// Start the language server
    #[arg(short, long)]
    start: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse;

    if args.start {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let (service, socket) = LspService::new(|client| Backend {
            client,
            document_map: dashmap::DashMap::new(),
        });
        Server::new(stdin, stdout, socket).serve(service).await;
    } else {
        println!("Please use --start to run the LSP server, or --help for more information.");
    }
}

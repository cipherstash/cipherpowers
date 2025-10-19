use clap::Parser;

mod models;
mod parser;

#[derive(Parser, Debug)]
#[command(name = "workflow")]
#[command(about = "Execute markdown-based workflows", long_about = None)]
struct Args {
    /// Path to workflow markdown file
    workflow_file: String,
}

fn main() {
    let args = Args::parse();
    println!("Workflow file: {}", args.workflow_file);
}

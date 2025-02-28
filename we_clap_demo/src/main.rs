use clap::Parser;

use we_clap::WeParser; // web enabled parser

impl we_clap::WeParser for Opts {} // enable web enabled parser for your clap struct

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about)]
pub struct Opts {
    /// A required string
    #[arg(short, long)]
    pub words: String,

    /// An optional value
    #[arg(short, long)]
    pub value: Option<f32>,
}

fn main() {
    let opts: Opts = Opts::we_parse(); // use web enabled parse and it works on native or web.

    // Note : we use cliw for output in this example.
    let msg = format!("{opts:?}");
    cliw::output::print(&msg);
}

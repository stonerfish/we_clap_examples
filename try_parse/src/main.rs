use clap::Parser;
use we_clap::WeParser;

impl we_clap::WeParser for Opts {}

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
    let opts: Result<Opts, clap::error::Error> = Opts::we_try_parse();

    match opts {
        Ok(opts) => {
            let msg = format!("{opts:?}");
            println!("{msg}");
            cliw::output::print(&msg);
        }
        Err(err) => {
            // jump through some hoops to get proper color and error direction on native
            #[cfg(not(target_arch = "wasm32"))]
            {
                err.print().unwrap();
            }
            // and this hoop to get output for web/wasm
            #[cfg(target_arch = "wasm32")]
            {
                let msg = format!("{err}");
                cliw::output::eprint(&msg);
            }
        }
    };
}

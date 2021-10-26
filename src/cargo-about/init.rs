use std::fs;
use structopt::StructOpt;

static DEFAULT_CONFIG: &str = include_str!("../../resources/about.toml");
static DEFAULT_HBS: &str = include_str!("../../resources/default.hbs");

#[derive(StructOpt, Debug)]
pub struct Args {
    /// Disables the handlebars generation
    #[structopt(long)]
    no_handlebars: bool,
    /// Forces cargo-about to overwrite the local config file
    #[structopt(long)]
    overwrite: bool,
}

pub fn cmd(args: Args) -> anyhow::Result<()> {
    let root_path = krates::cm::MetadataCommand::new().exec()?.workspace_root;
    let with_handlebars = !args.no_handlebars;

    if with_handlebars {
        let handlebars_path = root_path.join("about.hbs");
        let write_handlebars = !handlebars_path.is_file() || args.overwrite;
        if write_handlebars {
            fs::write(handlebars_path, DEFAULT_HBS)?;
        }
    }

    let config_path = root_path.join("about.toml");
    let write_config = !config_path.exists() || args.overwrite;
    if write_config {
        fs::write(config_path, DEFAULT_CONFIG)?;
    }

    Ok(())
}

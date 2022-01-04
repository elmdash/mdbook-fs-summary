use mdbook::errors::Error;
use mdbook::preprocess::CmdPreprocessor;
use semver::{Version, VersionReq};

mod book;
mod config;

fn main() {
    check_args();

    if let Err(e) = handle_preprocessing() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, _book) = CmdPreprocessor::parse_input(std::io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            config::PREPROCESSOR_NAME,
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let book = book::load_book(&ctx)?;
    serde_json::to_writer(std::io::stdout(), &book)?;

    Ok(())
}

fn check_args() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 && args[1] == "supports" {
        if args[2] == "not-supported" {
            eprintln!("renderer not supported: {}", args[1]);
            std::process::exit(1);
        } else {
            std::process::exit(0);
        }
    }
}

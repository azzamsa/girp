use clap::Parser;

#[derive(Parser)]
#[command(
    name = "girp",
    version,
    about = "Girp 🐙. \nPreview GitHub Markdown files locally.",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/girp/issues"
)]
pub struct Opts {}

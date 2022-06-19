use structopt::StructOpt;


#[derive(StructOpt)]
pub struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    pub message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    pub dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat text picture from the specified file
    pub catfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    pub stdin: bool,
}
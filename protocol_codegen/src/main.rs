use structopt::StructOpt;
use failure::Error;

pub mod generate_messages;

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    GenerateMessages,
}

#[derive(Debug, StructOpt)]
pub struct ApplicationArguments {
    #[structopt(subcommand)]
    pub command: SubCommand,
}

fn main() -> Result<(), Error> {
    let opt = ApplicationArguments::from_args();

    match opt.command {
        SubCommand::GenerateMessages => generate_messages::run()?,
    };
    Ok(())
}

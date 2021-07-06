use failure::Error;

pub mod generate_messages;

fn main() -> Result<(), Error> {
    generate_messages::run()
}

use clap::{Args, Parser, Subcommand};

mod studio;

#[derive(Subcommand, Debug)]
enum AuthToken {
    Get {},
    Set {},
}

#[derive(Subcommand, Debug)]
enum Install {}

#[derive(Subcommand, Debug)]
enum AuthCommands {
    // Gets the Roblox Studio authentication token for the current user. Rolox
    // Studio needs to have been logged in to at least once for this to return
    // anything
    Get {
        token_name: String,
    },

    // Sets the authentication token to use when logging into Roblox Studio.
    // This should be an active .ROBLOSECURITY cookie
    Set {
        token_name: String,
        token_value: String,
    },
}

#[derive(Args, Debug)]
struct AuthArgs {
    #[command(subcommand)]
    command: AuthCommands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Install Roblox Studio for the current user
    Install {},

    // Manage authentication credentials for Roblox Studio
    Auth(AuthArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Program {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let program = Program::parse();

    match program.command {
        Commands::Install {} => {
            println!("Install Roblox Studio");
        }
        Commands::Auth(args) => match args.command {
            AuthCommands::Get { token_name } => {
                let credential = studio::get_auth_credential(token_name.as_str());

                match credential {
                    Ok(credential) => {
                        println!("{:?}", credential.as_str());
                    }
                    Err(err) => {
                        println!("{:?}", err)
                    }
                }
            }
            AuthCommands::Set {
                token_name,
                token_value,
            } => {
                studio::set_auth_credential(token_name.as_str(), token_value.as_str()).ok();
            }
        },
    }
}

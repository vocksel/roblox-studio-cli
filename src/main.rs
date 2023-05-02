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
enum AuthTokenCommands {
    // Gets the Roblox Studio authentication token for the current user. Rolox
    // Studio needs to have been logged in to at least once for this to return
    // anything
    Get {},

    // Sets the authentication token to use when logging into Roblox Studio.
    // This should be an active .ROBLOSECURITY cookie
    Set { token: String },
}

#[derive(Args, Debug)]
struct AuthTokenArgs {
    #[command(subcommand)]
    command: AuthTokenCommands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Install Roblox Studio for the current user
    Install {},

    // Manage authentication credentials for Roblox Studio
    AuthToken(AuthTokenArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Program {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let program = Program::parse();

    match program.command {
        Commands::Install {} => {
            println!("Install Roblox Studio");
        }
        Commands::AuthToken(args) => match args.command {
            Get {} => {}
            Set {} => {}
        },
    }

    // let roblosecurity = StudioCredential {
    //     name: ".ROBLOSECURITY",
    //     value :""
    // };

    // let rbxidcheck = StudioCredential {
    //     name: ".RBXIDCHECK",
    //     value: "",
    // };

    // studio::set_auth_credential(roblosecurity.name, roblosecurity.value).ok();
    // // studio::set_auth_credential(rbxidcheck.name, rbxidcheck.value).ok();
    // studio::set_auth_credential(
    //     "Cookies",
    //     &format!("{};{};", roblosecurity.name, rbxidcheck.name),
    // )
    // .ok();
}

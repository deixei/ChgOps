use clap::{Command, Arg};

pub mod playbook_engine; // Add this import statement

fn cli() -> Command {
    Command::new("chgops")
        .about("ChgOps - Change management and operations tool")
        .version("1.0")
        .author("deixei <deixei@deixei.com>")
        .subcommand(
            Command::new("init")
                .about("Initializes a change management project")
                .arg(Arg::new("name")
                    .long("name")
                    .short('n')
                    .required(true))
                .arg(Arg::new("template")
                    .long("template")
                    .short('t')
                    .required(true)),
        )
        .subcommand(
            Command::new("run")
                .about("Runs a playbook")
                .arg(Arg::new("playbook")
                    .long("playbook")
                    .short('p')
                    .required(true))
                .arg(Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .help("Sets the level of verbosity")
                    .default_value("v")
                    .required(false))
                .arg(Arg::new("arguments")
                    .long("arguments")
                    .short('a')
                    .required(false)),
        )
        .subcommand(
            Command::new("build")
                .about("Builds chgops")
                .arg(Arg::new("debug")
                    .long("debug")
                    .short('d')
                    .required(true))
                .arg(Arg::new("change_id")
                    .long("change_id")
                    .short('c')
                    .required(true)),
        )
        .subcommand(
            Command::new("test")
                .about("Tests chgops")
                .arg(Arg::new("scope")
                    .long("scope")
                    .short('s')
                    .required(true)),
        )
        .subcommand(
            Command::new("publish")
                .about("Publishes chgops")
                .arg(Arg::new("ado_pack")
                    .long("ado_pack")
                    .required(true))
                .arg(Arg::new("package")
                    .long("package")
                    .short('p')
                    .required(true)),
        )
        .subcommand(
            Command::new("download")
                .about("Downloads chgops")
                .arg(Arg::new("name")
                    .long("name")
                    .short('n')
                    .required(true))
                .arg(Arg::new("version")
                    .long("version")
                    .short('v')
                    .required(true)),
        )
}

fn main() {
    let matches: clap::ArgMatches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").expect("required");
            let template = sub_matches.get_one::<String>("template").expect("required");
            println!(
                "Initializing with name: {}, template: {}",
                name,
                template
            );

        }
        Some(("run", sub_matches)) => {
            let playbook = sub_matches.get_one::<String>("playbook").expect("required");
            println!(
                "Running playbook: {}, verbose: {}, arguments: {}",
                playbook,
                sub_matches.get_one::<String>("verbose").expect("required"),
                sub_matches.get_one::<String>("arguments").expect("required")
            );

            playbook_engine::engine_run(playbook.to_string(), "template".to_string());
        }
        Some(("build", sub_matches)) => {
            println!(
                "Building with debug: {}, change_id: {}",
                sub_matches.get_one::<String>("debug").expect("required"),
                sub_matches.get_one::<String>("change_id").expect("required")
            );
        }
        Some(("test", sub_matches)) => {
            println!(
                "Testing with scope: {}",
                sub_matches.get_one::<String>("scope").expect("required")
            );
        }
        Some(("publish", sub_matches)) => {
            println!(
                "Publishing with ado_pack: {}, package: {}",
                sub_matches.get_one::<String>("ado_pack").expect("required"),
                sub_matches.get_one::<String>("package").expect("required")
            );
        }
        Some(("download", sub_matches)) => {
            println!(
                "Downloading with name: {}, version: {}",
                sub_matches.get_one::<String>("name").expect("required"),
                sub_matches.get_one::<String>("version").expect("required")
            );
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<String>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {ext:?} with {args:?}");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!
    }
}
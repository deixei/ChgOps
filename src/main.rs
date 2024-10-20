use clap::{Command, Arg};
use collections::dx::yaml_handler;
pub mod collections;
use crate::collections::dx::WORKSPACE;

pub mod command_line;
//use crate::command_line::collection;

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
                    .default_value("default")
                    .required(false))
                .arg(Arg::new("force")
                    .long("force")
                    .short('f')
                    .help("Force initialization on vars files")
                    .action(clap::ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("run")
                .about("Runs a playbook")
                .arg(Arg::new("name")
                    .long("name")
                    .short('n')
                    .default_value("playbook")
                    .required(false))
                .arg(Arg::new("path")
                    .long("path")
                    .short('p')
                    .default_value("")
                    .required(false))
                .arg(Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .help("Sets the level of verbosity")
                    .default_value("")
                    .required(false))
                .arg(Arg::new("arguments")
                    .long("arguments")
                    .short('a')
                    .default_value("STAGE=dev")
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
        .subcommand(
            Command::new("collection")
                .about("Manages collections")
                .subcommand(
                    Command::new("init")
                        .about("Initializes a collection")
                        .arg(Arg::new("name")
                            .long("name")
                            .short('n')
                            .required(true))
                        .arg(Arg::new("collection")
                            .long("collection")
                            .short('c')
                            .required(true))
                        .arg(Arg::new("force")
                            .long("force")
                            .short('f')
                            .help("Force initialization on vars files")
                            .action(clap::ArgAction::SetTrue)),
                )
                .subcommand(
                    Command::new("test")
                        .about("Tests a collection")
                        .arg(Arg::new("scope")
                            .long("scope")
                            .short('s')
                            .required(true)),
                ),
        )
}

fn main() {
    let matches: clap::ArgMatches = cli().get_matches();

    println!("ChgOps -----------------------------------------");

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").expect("required");
            let template = sub_matches.get_one::<String>("template").expect("required");
            let force = sub_matches.get_flag("force");
            
            println!("Initializing with name: {}, template: {}", name, template);
            {
                let _ = command_line::init::action_init(name.as_str(), template.as_str(), force);
            }

        }
        Some(("run", sub_matches)) => {
            
            let playbook_name = sub_matches.get_one::<String>("name").expect("required");
            let workspace_path = sub_matches.get_one::<String>("path").expect("required");
            let verbose = sub_matches.get_one::<String>("verbose").expect("required");
            let arguments = sub_matches.get_one::<String>("arguments").expect("required");
            
            println!(
                "Running playbook: {}, verbose: {}, arguments: {}",
                playbook_name,
                verbose,
                arguments
            );
            {
                let mut workspace = WORKSPACE.lock().unwrap();
                workspace.playbook_name = playbook_name.to_string();
                workspace.workspace_path = workspace_path.to_string();
                workspace.verbose = verbose.to_string();
                workspace.arguments = arguments.to_string();

                workspace.load_workspace();

                workspace.run_playbook();
            }
        }
        Some(("build", sub_matches)) => {
            println!(
                "Building with debug: {}, change_id: {}",
                sub_matches.get_one::<String>("debug").expect("required"),
                sub_matches.get_one::<String>("change_id").expect("required")
            );
        }
        Some(("test", sub_matches)) => {
            let scope = sub_matches.get_one::<String>("scope").expect("required");
            println!("Testing with scope: {}", scope);
            let _ = yaml_handler::example();
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
        Some(("collection", sub_matches)) => {
            // ./chgops collection init -n demo -c basic 
            match sub_matches.subcommand() {
                Some(("init", sub_matches)) => {
                    let name = sub_matches.get_one::<String>("name").expect("required");
                    let collection = sub_matches.get_one::<String>("collection").expect("required");
                    let force = sub_matches.get_flag("force");

                    let _ = command_line::collection::collection_init(name.as_str(), collection.as_str(), force);
                }
                Some(("test", sub_matches)) => {
                    let scope = sub_matches.get_one::<String>("scope").expect("required");
                    let _ = command_line::collection::collection_test(scope.as_str());
                }

                _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!
            }
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
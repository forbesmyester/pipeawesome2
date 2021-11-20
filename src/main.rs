use pipeawesome2::config::FaucetConfig;
use pipeawesome2::junction::Junction;
use pipeawesome2::motion::Push;
use pipeawesome2::drain::Drain;
use pipeawesome2::waiter::JoinTo;
use pipeawesome2::waiter::JoinFrom;
use pipeawesome2::motion::Pull;
use pipeawesome2::faucet::Faucet;
use pipeawesome2::motion::ReadSplitControl;
use pipeawesome2::config::InputPort;
use pipeawesome2::config::ComponentType;
use std::collections::HashSet;
use pipeawesome2::waiter::Waiter;
use pipeawesome2::buffer::Buffer;
use pipeawesome2::launch::Launch;
use pipeawesome2::config::Connection;
use pipeawesome2::config::ConfigLintWarning;
use pipeawesome2::config::DeserializedConnection;
use pipeawesome2::config::load_connection_from_string;
use pipeawesome2::config::Config;
use clap::ArgMatches;
use clap::SubCommand;
use std::collections::HashMap;
use clap::{ App, Arg };


#[derive(Debug)]
enum ConfigFormat {
    JSON,
    YAML,
}

fn get_clap_app() -> App<'static, 'static> {


    fn get_required_arg_with<'a>(name: &'a str, help: &'a str, value_name: &'a str) -> Arg<'a, 'a> {
        Arg::with_name(name)
            .long(name)
            .help(help)
            .required(false)
            .takes_value(true)
            .value_name(value_name)
    }


    fn get_required_index_arg<'a>(name: &'a str, help: &'a str, index: u64) -> Arg<'a, 'a> {
        Arg::with_name(name)
            .help(help)
            .required(true)
            .index(index)
    }


    fn get_multi_arg<'a>(name: &'a str, help: &'a str, index: u64) -> Arg<'a, 'a> {
        Arg::with_name(name)
            .help(help)
            .required(true)
            .index(index)
            .multiple(true)
    }

    fn config_format<'a>() -> Arg<'a, 'a> {
        Arg::with_name("config-format")
            .help("The format of the configuration data")
            .required(false)
            .possible_values(&["yaml", "json"])
    }

    App::new("PipeAwesome")
        .author("Matthew Forrester, githib.com@speechmarks.com")
        .version("0.0.0")
        .about("Like UNIX pipes, but on sterroids")

        .subcommand(
            SubCommand::with_name("process")
                .arg(get_required_arg_with("config", "The config file to read, \"-\" for STDIN. If not specified it will be blank", "FILENAME"))
                .arg(config_format())
            )

        .subcommand(
            SubCommand::with_name("config")
                .arg(get_required_arg_with("config", "The config file to read, \"-\" for STDIN. If not specified it will be blank", "FILENAME"))
                .arg(config_format())
                .arg(get_required_arg_with("config-out", "The config file to write, \"-\" for STDOUT. Defaults to the file being read (otherwise STDOUT)", "FILENAME"))
                .subcommand(
                    SubCommand::with_name("empty")
                )

                .subcommand(
                    SubCommand::with_name("lint")
                )

                .subcommand(
                    SubCommand::with_name("faucet")
                        .arg(get_required_arg_with("id", "The ID of the faucet to modify", "ID"))
                        .subcommand(
                            SubCommand::with_name("source")
                                .arg(get_required_index_arg("SOURCE", "SOURCE must be either a filename, \"-\" for STDIN, or empty for NULL input", 1))
                        )
                        .subcommand(
                            SubCommand::with_name("watermark")
                                .arg(get_required_index_arg("MIN", "MIN must be an integer", 1))
                                .arg(get_required_index_arg("MAX", "MAX must be an integer greater than MIN", 2))
                        )
                        .subcommand(
                            SubCommand::with_name("min-max-clear")
                        )
                )

                .subcommand(
                    SubCommand::with_name("drain")
                        .arg(get_required_arg_with("id", "The ID of the drain to modify", "ID"))
                        .subcommand(
                            SubCommand::with_name("destination")
                                .arg(get_required_index_arg("DESTINATION", "DESTINATION must be either a filename, \"-\" for STDOUT, \"_\" for STDERR or empty for NULL output", 1))
                        )
                )

                .subcommand(
                    SubCommand::with_name("connection")
                        .arg(get_required_arg_with("id", "The ID of the connection to modify", "ID"))
                        .subcommand(
                            SubCommand::with_name("join")
                                .arg(get_required_index_arg("JOIN", "The Join to extablish either as a JoinString or JSON", 1))
                        )
                        .subcommand(
                            SubCommand::with_name("del")
                        )
                )

                .subcommand(
                    SubCommand::with_name("launch")
                        .arg(get_required_arg_with("id", "The ID of the launch to modify", "ID"))
                        .subcommand(
                            SubCommand::with_name("command")
                                .arg(get_required_index_arg("COMMAND", "The command to run, can be a full path if not in your $PATH", 1))
                        )
                        .subcommand(
                            SubCommand::with_name("args")
                                .arg(get_multi_arg("ARGS", "Arguments that will be passed to the program", 1))
                        )
                        .subcommand(
                            SubCommand::with_name("env")
                                .arg(get_multi_arg("ENV", "The environmental variables for the program, specifiy as NAME=VALUE", 1))
                        )
                        .subcommand(
                            SubCommand::with_name("path")
                                .arg(get_required_index_arg("PATH", "PATH will be the directory in which the COMMAND in launch-cmd is ran in", 1))
                        )
                )
        )
        .after_help("Longer explanation to appear after the options when \
                     displaying the help information from --help or -h")
}

#[derive(Debug)]
struct UserConfigOptionBase {
    id: String,
    config_in: String,
    config_format: ConfigFormat,
    config_out: String,
}

#[derive(Debug)]
enum UserRequest {
    LaunchCommand {
        base_options: UserConfigOptionBase,
        command: String,
    },
    LaunchPath {
        base_options: UserConfigOptionBase,
        path: String,
    },
    LaunchArgs {
        base_options: UserConfigOptionBase,
        args: Vec<String>,
    },
    LaunchEnv {
        base_options: UserConfigOptionBase,
        env: HashMap<String, String>,
    },
    FaucetSrc {
        base_options: UserConfigOptionBase,
        src: String,
    },
    DrainDst {
        base_options: UserConfigOptionBase,
        dst: String,
    },
    FaucetWatermark {
        base_options: UserConfigOptionBase,
        min: usize,
        max: usize,
    },
    ConnectionJoin {
        base_options: UserConfigOptionBase,
        join: String,
    },
    ConfigLintShow { config_in: String, config_format: ConfigFormat },
    Process { config_in: String, config_format: ConfigFormat },
}


pub fn convert_to_deserialized_connection(s: String) -> Result<DeserializedConnection, String> {
    if let Ok(_conns) = load_connection_from_string(&s) {
        return Ok(DeserializedConnection::JoinString(s));
    }
    if let Ok(conn) = serde_json::from_str::<Connection>(&s) {
        return Ok(DeserializedConnection::Connections(vec![conn]));
    }
    if let Ok(conns) = serde_json::from_str::<Vec<Connection>>(&s) {
        return Ok(DeserializedConnection::Connections(conns));
    }
    if let Ok(conn) = serde_yaml::from_str::<Connection>(&s) {
        return Ok(DeserializedConnection::Connections(vec![conn]));
    }
    if let Ok(conns) = serde_yaml::from_str::<Vec<Connection>>(&s) {
        return Ok(DeserializedConnection::Connections(conns));
    }
    Err(format!("Looks neither like JSON, YAML or JoinString '{}'", s))
}


fn get_user_config_action(matches: &ArgMatches) -> Result<UserRequest, String> {

    #[derive(Debug)]
    struct CollectedSubcommands<'a> {
        subcommands: Vec<(&'a ArgMatches<'a>, &'a str)>,
        final_sub_command: &'a ArgMatches<'a>
    }

    fn collect_subcommands<'a>(matches: &'a ArgMatches) -> CollectedSubcommands<'a> {

        fn subcommand_inquire(mut v: CollectedSubcommands) -> CollectedSubcommands {
            match v.final_sub_command.subcommand() {
                (s, Some(am2)) => {
                    v.subcommands.push((am2, s));
                    v.final_sub_command = am2;
                    subcommand_inquire(v)
                },
                _ => v
            }
        }

        subcommand_inquire(CollectedSubcommands { subcommands: vec![], final_sub_command: matches })
    }

    fn get_config_format<'a>(first_sub_command: Option<&'a ArgMatches>) -> Result<ConfigFormat, String> {
        let first = first_sub_command.map(|sc1| sc1.value_of("config").or(Some("-"))).flatten();

        let fmt = match first_sub_command.map(|sc1| sc1.value_of("config_format").or(first)).flatten() {
            Some("yaml") => Some(ConfigFormat::YAML),
            Some("json") => Some(ConfigFormat::JSON),
            Some(filename) if filename.ends_with(".yaml") => Some(ConfigFormat::YAML),
            Some(filename) if filename.ends_with(".yml") => Some(ConfigFormat::YAML),
            Some(filename) if filename.ends_with(".json") => Some(ConfigFormat::JSON),
            _ => None
        };

        fmt.ok_or("Could not figure out configuration format".to_string())
    }

    fn get_standard_config_opts<'a>(first_sub_command: Option<&'a ArgMatches>, second_sub_command: Option<&'a ArgMatches>, second_name: Option<&str>) -> Result<UserConfigOptionBase, String> {

        let first = first_sub_command.map(|sc1| sc1.value_of("config").or(Some("-"))).flatten();

        let fmt = get_config_format(first_sub_command)?;

        let standard_options = (
            first,
            first_sub_command.map(|sc1| sc1.value_of("config-out")).flatten().or(first),
            second_sub_command.map(|sc2| sc2.value_of("id")).flatten(),
            second_name
        );

        match standard_options {
            (Some(config_in), Some(config_out), Some(id), _) => {
                Ok(UserConfigOptionBase { config_format: fmt, config_in: config_in.to_string(), config_out: config_out.to_string(), id: id.to_string() })
            },
            (_, _, None, Some(second_name)) => Err(format!("You need to specify an id for the '{}'", second_name)),
            (_, _, None, _) => Err("You didn't correctly specify what to configure".to_string()),
            _ => Err("Somehow we didn't understand those commands".to_string())
        }

    }

    fn option_of_tuples_to_option_tuple<X>(x: (Option<X>, Option<X>)) -> Option<(X, X)> {
        if let (Some(a), Some(b)) = x {
            return Some((a, b));
        }
        None
    }


    fn get_user_action(collected_subcommands: CollectedSubcommands) -> Result<UserRequest, String> {

        fn get_config_in(first_sub_command: Option<&clap::ArgMatches>) -> String {
            first_sub_command.map(|sc1| sc1.value_of("config")).flatten().unwrap_or("-").to_string()
        }

        let first_sub_command = collected_subcommands.subcommands.iter().map(|x| x.0).next();

        let base_options = get_standard_config_opts(
            first_sub_command,
            collected_subcommands.subcommands.iter().map(|x| x.0).nth(1),
            collected_subcommands.subcommands.iter().map(|x| x.1).nth(1)
        );

        let coll_subcomm_str: Vec<&str> = collected_subcommands.subcommands.iter().map(|x| x.1).collect();
        let last_sub_command: Option<&ArgMatches> = collected_subcommands.subcommands.iter().map(|x| x.0).last();



        match (base_options, &coll_subcomm_str[..]) {
            (_, ["process"]) => {
                Ok(UserRequest::Process { config_format: get_config_format(first_sub_command)?, config_in: get_config_in(first_sub_command) })
            },
            (_, ["config", "lint"]) => {
                Ok(UserRequest::ConfigLintShow { config_format: get_config_format(first_sub_command)?, config_in: get_config_in(first_sub_command) })
            },
            (Ok(base_options), ["config", "connection", "join"]) => {
                last_sub_command
                    .map(|lsc| lsc.value_of("JOIN")).flatten()
                    .map(|join| UserRequest::ConnectionJoin { base_options, join: join.to_string() })
                    .ok_or_else(|| "Command {:?} did not have all required values".to_string())
            },
            (Ok(base_options), ["config", "drain", "destination"]) => {
                last_sub_command
                    .map(|lsc| lsc.value_of("DESTINATION")).flatten()
                    .map(|dst| UserRequest::DrainDst { base_options, dst: dst.to_string() })
                    .ok_or_else(|| "Command {:?} did not have all required values".to_string())
            },
            (Ok(base_options), ["config", "faucet", "source"]) => {
                last_sub_command
                    .map(|lsc| lsc.value_of("SOURCE")).flatten()
                    .map(|src| UserRequest::FaucetSrc { base_options, src: src.to_string() })
                    .ok_or_else(|| "Command {:?} did not have all required values".to_string())
            },
            (Ok(base_options), ["config", "faucet", "watermark"]) => {
                last_sub_command
                    .map(|lsc| {
                        (
                            lsc.value_of("MIN").map(|a| a.parse::<usize>().ok()).flatten(),
                            lsc.value_of("MAX").map(|a| a.parse::<usize>().ok()).flatten()
                        )
                    })
                    .map(|tup| option_of_tuples_to_option_tuple((tup.0, tup.1))).flatten()
                    .map(|tup| UserRequest::FaucetWatermark { base_options, min: tup.0, max: tup.1 })
                    .ok_or_else(|| "Command {:?} did not have all required values".to_string())
            },
            (Ok(base_options), ["config", "launch", "command"]) => {
                last_sub_command
                    .map(|lsc| lsc.value_of("COMMAND")).flatten()
                    .map(|cmd| UserRequest::LaunchCommand { base_options, command: cmd.to_string() })
                    .ok_or_else(|| "Command {:?} did not have all required values".to_string())
            },
            (Ok(base_options), ["config", "launch", "path"]) => {
                last_sub_command
                    .map(|lsc| lsc.value_of("PATH")).flatten()
                    .map(|path| UserRequest::LaunchPath { base_options, path: path.to_string() })
                    .ok_or_else(|| "Command {:?} did not have all required values".to_string())
            },
            (Ok(base_options), ["config", "launch", "args"]) => {
                last_sub_command
                    .map(|lsc|
                        match lsc.values_of("ARGS") {
                            None => vec![],
                            Some(xs) => xs.map(|s| s.to_string()).collect()
                        }
                    )
                    .map(|args| UserRequest::LaunchArgs { base_options, args })
                    .ok_or_else(|| "Command {:?} did not have all required values".to_string())
            },
            (Ok(base_options), ["config", "launch", "env"]) => {
                last_sub_command
                    .ok_or_else(|| Err("Cannot read command".to_string()))
                    .map(|lsc|
                        match lsc.values_of("ENV") {
                            None => vec![],
                            Some(xs) => xs.map(|s| s.to_string()).collect()
                        }
                    )
                    .map_err(|_e: Result<Vec<String>, std::string::String>| "Could not read argument ENV".to_string())
                    .and_then(|envs_str| {
                        fn as_kv(form: &str, s: String) -> Result<(String, String), String> {
                            s.split_once('=')
                                .ok_or(format!("{} does not include a name (must be in form {})", s, form))
                                .and_then(|(a, b)| {
                                    if a.is_empty() {
                                        return Err(format!("{} The {} form must include a non-zero length NAME", s, form));
                                    }
                                    Ok((a.to_string(), b.to_string()))
                                })
                        }

                        envs_str.into_iter().fold(
                            Ok(HashMap::new()),
                            |hm, s| {
                                match (hm, as_kv("NAME=VALUE", s)) {
                                    (Ok(mut hm), Ok((k, mut v))) => {
                                        hm.entry(k).and_modify(|x| std::mem::swap(x, &mut v)).or_insert(v);
                                        Ok(hm)
                                    }
                                    (Err(e), _) => Err(e),
                                    (_, Err(e)) => Err(e),
                                }
                            }
                        )

                    })
                    .map(|env| UserRequest::LaunchEnv { base_options, env })
                    .map_err(|_| "Command {:?} did not have all required values".to_string())

            },
            _ => {
                Err(format!("The command {:?} requires further sub commands", coll_subcomm_str))
            }
        }
    }

    get_user_action(collect_subcommands(matches))

}

fn read_config_as_str(config_in: &str) -> Result<String, String> {

    use std::io::prelude::*;
    let mut buffer = String::new();

    if config_in == "-" {
        let mut stdin = std::io::stdin(); // We get `Stdin` here.
        stdin.read_to_string(&mut buffer).map_err(|_x| "We could not read STDIN to get the config".to_string())?;
        return Ok(buffer);
    }

    let mut f = std::fs::File::open(config_in).map_err(|_x| format!("We could not open the file '{}' to get the config", config_in))?;
    f.read_to_string(&mut buffer).map_err(|_x| format!("We could not open the file '{}' to get the config", config_in))?;
    Ok(buffer)
}

fn parse_config_str(fmt: &ConfigFormat, config_str: &str) -> Result<Config, String> {
    match fmt {
        ConfigFormat::JSON => serde_json::from_str::<Config>(config_str).map_err(|e| format!("Could not parse config ({})", e)),
        ConfigFormat::YAML => serde_yaml::from_str::<Config>(config_str).map_err(|e| format!("Could not parse config ({})", e)),
    }
}

fn read_config(fmt: &ConfigFormat, config_in: &str) -> Result<Config, String> {
    result_flatten(read_config_as_str(config_in).map(|cas| parse_config_str(fmt, &cas)))
}

fn result_flatten<X>(x: Result<Result<X, String>, String>) -> Result<X, String> {
    match x {
        Ok(Ok(x)) => Ok(x),
        Ok(Err(s)) => Err(s),
        Err(s) => Err(s),
    }
}


enum UserResponse {
    Config(Config),
    Process(Config),
}


fn process_user_config_action(result_config: Result<UserRequest, String>) -> Result<UserResponse, String> {

    match result_config {
        Err(x) => Err(x),
        Ok(UserRequest::ConnectionJoin { base_options: UserConfigOptionBase { id, config_in, config_format, .. }, join, .. }) => {
            convert_to_deserialized_connection(join).and_then(|dsc|
                read_config(&config_format, &config_in).map(|old_config| Config::connection_join(old_config, id, dsc))
                    .map(UserResponse::Config)
            )
        },
        Ok(UserRequest::FaucetSrc { base_options: UserConfigOptionBase { id, config_in, config_format, .. }, src }) => {
            read_config(&config_format, &config_in).map(|old_config| Config::faucet_set_source(old_config, id, src))
                .map(UserResponse::Config)
        },
        Ok(UserRequest::DrainDst { base_options: UserConfigOptionBase { id, config_in, config_format, .. }, dst }) => {
            read_config(&config_format, &config_in).map(|old_config| Config::drain_set_destination(old_config, id, dst))
                .map(UserResponse::Config)
        },
        Ok(UserRequest::FaucetWatermark { base_options: UserConfigOptionBase { id, config_in, config_format, .. }, min, max }) => {
            read_config(&config_format, &config_in).map(|old_config| Config::faucet_set_watermark(old_config, id, min, max))
                .map(UserResponse::Config)
        },
        Ok(UserRequest::LaunchCommand { base_options: UserConfigOptionBase { id, config_in, config_format, .. }, command }) => {
            read_config(&config_format, &config_in).map(|old_config| Config::launch_set_command(old_config, id, command))
                .map(UserResponse::Config)
        },
        Ok(UserRequest::LaunchPath { base_options: UserConfigOptionBase { id, config_in, config_format, .. }, path }) => {
            read_config(&config_format, &config_in).map(|old_config| Config::launch_set_path(old_config, id, path))
                .map(UserResponse::Config)
        },
        Ok(UserRequest::LaunchArgs { base_options: UserConfigOptionBase { id, config_in, config_format, .. }, args }) => {
            read_config(&config_format, &config_in).map(|old_config| Config::launch_set_args(old_config, id, args))
                .map(UserResponse::Config)
        },
        Ok(UserRequest::LaunchEnv { base_options: UserConfigOptionBase { id, config_in, config_format, .. }, env }) => {
            read_config(&config_format, &config_in).map(|old_config| Config::launch_set_env(old_config, id, env))
                .map(UserResponse::Config)
        },
        Ok(UserRequest::ConfigLintShow { config_in, config_format }) => {
            let mut config = read_config(&config_format, &config_in)?;
            let errs = Config::lint(&mut config).into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
            if errs.is_empty() {
                return Ok(config).map(UserResponse::Config)
            }
            return Err(format!("We found the following warnings / errors: \n\n * {}\n", errs.join("\n * ")));
        },
        Ok(UserRequest::Process { config_in, config_format }) => {
            let mut config = read_config(&config_format, &config_in)?;
            let errs = Config::lint(&mut config).into_iter()
                .filter(|lint_err| match lint_err {
                    ConfigLintWarning::InConfigButMissingFlowConnection { .. } => false,
                    _ => true,
                })
                .map(|c| c.to_string()).collect::<Vec<String>>();
            if errs.is_empty() {
                return Ok(config).map(UserResponse::Process);
            }
            return Err(format!("Process {}", errs.join("\n * ")));
        },
    }

}

fn get_waiter(mut config: Config) -> Result<Waiter, String> {

    let mut created: HashSet<(&ComponentType, &str)> = HashSet::new();
    let mut last: Option<&Connection> = None;

    let all_connections = config.connection.iter().fold(
        Vec::new(),
        |mut acc, (_hash_key, deser_conn)| {
            if let DeserializedConnection::Connections(v) = deser_conn {
                acc.extend_from_slice(v);
                return acc;
            }
            panic!("Encountered DeserializedConnection::JoinString in main::get_waiter()")
        }
    );

    let mut waiter = Waiter::new();

    struct CreateSpec<'a> {
        component_type: &'a ComponentType,
        component_name: &'a String,
    }

    fn get_create_spec(connection: &Connection) -> CreateSpec {
        match connection {
            Connection::MiddleConnection { component_type, component_name, .. } => CreateSpec { component_type, component_name },
            Connection::StartConnection { component_type, component_name, .. } => CreateSpec { component_type, component_name },
            Connection::EndConnection { component_type, component_name, .. } => CreateSpec { component_type, component_name },
        }
    }

    fn convert_connection_to_join_from(connection: &Connection) -> Option<JoinFrom> {
        match connection {
            Connection::MiddleConnection { component_type, component_name, output_port, .. } => Some(JoinFrom { component_type: *component_type, component_name: component_name, output_port: *output_port }),
            Connection::StartConnection { component_type, component_name, output_port } => Some(JoinFrom { component_type: *component_type, component_name: component_name, output_port: *output_port }),
            Connection::EndConnection { .. } => None,
        }
    }

    fn convert_connection_to_join_to(connection: &Connection) -> Option<JoinTo> {
        match connection {
            Connection::MiddleConnection { component_type, component_name, input_port: InputPort::In(input_priority), .. } => Some(JoinTo { component_type: *component_type, component_name, input_priority: *input_priority }),
            Connection::StartConnection { .. } => None,
            Connection::EndConnection { component_type, component_name, input_port: InputPort::In(input_priority) } => Some(JoinTo { component_type: *component_type, component_name, input_priority: *input_priority })
        }
    }

    async fn constructor(create_spec: &CreateSpec<'_>, config: &mut Config, w: &mut Waiter) -> Result<(), String> {

        match create_spec {
            CreateSpec { component_type: ComponentType::Faucet, component_name, .. } => {
                // TODO: Figure out how to get this in...
                let pull = match config.faucet.get(*component_name).map(|t| t.source.as_str()).unwrap_or("") {
                    "-" => Pull::Stdin(async_std::io::stdin(), ReadSplitControl::new()),
                    "" => Pull::None,
                    filename => {
                        let file = async_std::fs::File::open(filename).await.map_err(|_| { format!("Could not open file: {}", filename) })?;
                        Pull::File(file, ReadSplitControl::new())
                    },
                };
                let faucet = Faucet::new(pull);
                w.add_faucet(component_name.to_string(), faucet);
                Ok(())
            },
            CreateSpec { component_type: ComponentType::Drain, component_name, .. } => {
                // TODO: Figure out how to get this in...
                let push = match config.drain.get(*component_name).map(|s| s.destination.as_str()).unwrap_or("") {
                    "-" => Push::Stdout(async_std::io::stdout()),
                    "_" => Push::Stderr(async_std::io::stderr()),
                    "" => Push::None,
                    filename => {
                        let file = async_std::fs::File::create(filename).await.map_err(|_| { format!("Could not write to file: {}", filename) })?;
                        Push::File(async_std::io::BufWriter::new(file))
                    },
                };
                w.add_drain(component_name.to_string(), Drain::new(push));
                Ok(())
            },
            CreateSpec { component_type: ComponentType::Buffer, component_name, .. } => {
                w.add_buffer(component_name.to_string(), Buffer::new());
                Ok(())
            },
            CreateSpec { component_type: ComponentType::Junction, component_name, .. } => {
                w.add_junction(component_name.to_string(), Junction::new());
                Ok(())
            },
            CreateSpec { component_type: ComponentType::Launch, component_name, .. } => {
                if let Some(cfg) = config.launch.remove(*component_name) {
                    if cfg.command.is_none() {
                        return Err(format!("Launch '{}' did not have a command specified", component_name));
                    }
                    let launch: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
                        if cfg.env.is_empty() { None } else { Some(cfg.env) },
                        cfg.path,
                        cfg.command.ok_or(format!("Launch '{}' did not have a command specified", component_name))?,
                        if cfg.arg.is_empty() { None } else { Some(cfg.arg) }
                    );
                    w.add_launch(component_name.to_string(), launch);
                    return Ok(());
                }
                Err(format!("Could not find configuration for Launch {}", component_name))
            }
        }
    }


    for connection in all_connections.iter() {
        let create_spec = get_create_spec(connection);

        if !created.contains(&(create_spec.component_type, create_spec.component_name)) {
            async_std::task::block_on(constructor(&create_spec, &mut config, &mut waiter))?;
            created.insert((create_spec.component_type, create_spec.component_name));
        }

        if let Some(last_connection) = last {
            let err = match (convert_connection_to_join_from(last_connection), convert_connection_to_join_to(connection)) {
                (Some(join_component_from), Some(join_component_to)) => {
                    waiter.join(join_component_from, join_component_to).map_err(|c| format!("{}", c))
                },
                _ => Err(format!("There should have been a connection between {:?} and {:?}", last_connection, connection))
            };
            if let Err(err_msg) = err {
                return Err(err_msg);
            }
        }

        last = Some(connection);
        if let Connection::EndConnection { .. } = connection {
            last = None;
        }
    }

    for (faucet_name, FaucetConfig { source: _, monitored_buffers, buffered}) in config.faucet.into_iter() {
        if let Some(b) = buffered {
            waiter.configure_faucet(faucet_name, monitored_buffers, b.0, b.1);
        }
    }

    Ok(waiter)

}

fn main() {

    let app = get_clap_app();
    let matches = app.get_matches();

    let user_action = get_user_config_action(&matches);
    let new_config = process_user_config_action(user_action);

    let r = match new_config {
        Ok(UserResponse::Config(new_cfg)) => {
            serde_json::to_string(&new_cfg).map_err(|_x| "Could not serialize new Config".to_string())
        },
        Ok(UserResponse::Process(new_config)) => {
            match get_waiter(new_config) {
                Ok(mut waiter) => {
                    async_std::task::block_on(waiter.start())
                        .map_err(|err| format!("{:?}", err))
                        .map(|_processed_count| "".to_string())
                }
                Err(x) => Err(x),
            }
        },
        Err(msg) => Err(msg)
    };


    match r {
        Ok(s) => {
            if !s.is_empty() {
                println!("{}", s);
            }
        }
        Err(s) => {
            eprintln!("{}", s);
            std::process::exit(1);
        }
    }

}

// == Probably dead! ================================================

//     fn as_kkv<'a>(form: &str, s: &'a str) -> Result<(&'a str, &'a str, &'a str), String> {
//         let split: Vec<&str> = s.splitn(3, '=').collect();
//         match split.len() {
//             3 => Ok((split[0], split[1], split[2])),
//             _ => Err(format!("{} The {} form must include a non-zero length NAME", s, form))
//         }
//     }
// 
// 
//     // TODO: Must be above 0
//     fn min_max_validator(form: &str, s: String) -> Result<KMinMax, String>{
//         let (k, v) = as_kv(form, &s)?;
// 
//         v.split_once(',')
//             .ok_or(format!("{} does not include a comma (must be in form {})", s, form))
//             .and_then(|(a, b)| {
// 
//                 match (a.parse::<usize>(), b.parse::<usize>()) {
//                     (Ok(aa), Ok(bb)) => {
//                         match aa > bb {
//                             true => Ok(KMinMax {k: k.to_string(), min: bb, max: aa }),
//                             false => Ok(KMinMax {k: k.to_string(), min: aa, max: bb }),
//                         }
//                     },
//                     _ => Err(format!("{} Min and Max must both be positive integers", s)),
//                 }
// 
//             })
// 
//     }
// 
//     fn kv_validator(form: &str, s: String) -> Result<KV, String>{
//         let (a, b) = as_kv(form, &s)?;
//         Ok(KV {k: a.to_string(), v: b.to_string()})
//     }
// 
//     fn kkv_validator(form: &str, s: String) -> Result<KKV, String>{
//         let (a, b, c) = as_kkv(form, &s)?;
//         Ok(KKV {kk: a.to_string(), k: b.to_string(), v: c.to_string()})
//     }
// 
//     fn kv_arg_quick<'a>(name: &'a str, form: &'a str, help: &'a str, validator: fn(String) -> Result<(), String>) -> Arg<'a, 'a> {
//         Arg::with_name(name)
//             .long(name)
//             .help(help)
//             .required(false)
//             .takes_value(true)
//             .multiple(true)
//             .value_name(form)
//             .validator(validator)
//     }
// 
//     fn non_empty_string_validator(form: &str, s: String) -> Result<String, String>{
//         match s.len() {
//             0 => Err(format!("{} The {} form must be a non-zero length", s, form)),
//             _ => Ok(s),
//         }
//     }
// 
// struct KMinMax {
//     k: String,
//     min: usize,
//     max: usize,
// }
// 
// struct KV {
//     k: String,
//     v: String,
// }
// 
// struct KKV {
//     kk: String,
//     k: String,
//     v: String,
// }


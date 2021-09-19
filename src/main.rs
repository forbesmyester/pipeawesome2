use pipeawesome2::config::Config;
use clap::ArgMatches;
use clap::SubCommand;
use clap::ArgGroup;
use std::collections::HashMap;
use clap::{ App, Arg };
use async_std::io as aio;
use pipeawesome2::{buffer::Buffer, drain::Drain, faucet::Faucet, junction::Junction, launch::Launch, motion::{Pull, Push, ReadSplitControl}, waiter::{ WaiterError, Waiter }};
use async_std::task;

async fn do_stuff() -> Result<usize, WaiterError> {

    let stdin = aio::stdin();
    let stdout = aio::stdout();
    let stderr = aio::stderr();

    let mut faucet = Faucet::new(Pull::Stdin(stdin, ReadSplitControl::new()));
    let mut junction_0 = Junction::new();
    let mut junction_1 = Junction::new();
    let mut junction_2 = Junction::new();
    let mut drain = Drain::new(Push::Stdout(stdout));
    let mut buffer = Buffer::new();

    let mut launch_line_numbers: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "mawk".to_string(),
        Some(vec!["-W".to_string(), "interactive".to_string(), r#"{ printf("%04d %s\n", NR, $0) }"#.to_string()])
    );

    let mut launch_filter_odd_only: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "mawk".to_string(),
        Some(vec!["-W".to_string(), "interactive".to_string(), "-f".to_string(), "res/modulus_print.awk".to_string(), "-v".to_string(), "rem=1".to_string()])
    );

    let mut launch_filter_even_only: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "mawk".to_string(),
        Some(vec!["-W".to_string(), "interactive".to_string(), "-f".to_string(), "res/modulus_print.awk".to_string(), "-v".to_string(), "rem=0".to_string()])
    );

    let mut launch_add_letter_o: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "mawk".to_string(),
        Some(vec!["-W".to_string(), "interactive".to_string(), "-v".to_string(), "letter=O".to_string(), r#"{ printf("%s %s: ", $1, letter); for (i=2; i<=NF; i++) printf("%s ", $i); print "" }"#.to_string()])
    );

    let mut launch_add_letter_e: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "mawk".to_string(),
        Some(vec!["-W".to_string(), "interactive".to_string(), "-v".to_string(), "letter=E".to_string(), r#"{ printf("%s %s: ", $1, letter); for (i=2; i<=NF; i++) printf("%s ", $i); print "" }"#.to_string()])
    );

    let mut slow_0: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "mawk".to_string(),
        Some(vec!["-W".to_string(), "interactive".to_string(), r#"{ system("sleep 0.1"); print $0 }"#.to_string()])
    );

    let mut launch_decorate_exit: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "mawk".to_string(),
        Some(vec!["-W".to_string(), "interactive".to_string(), r#"{ print "EXIT: " $0 }"#.to_string()])
    );

    let mut slow_1: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "mawk".to_string(),
        Some(vec!["-W".to_string(), "interactive".to_string(), r#"{ system("sleep 0.2"); print $0 }"#.to_string()])
    );


    slow_0.add_stdin(faucet.add_stdout());
    launch_line_numbers.add_stdin(slow_0.add_stdout());
    junction_0.add_stdin(launch_line_numbers.add_stdout(), false);
    launch_add_letter_e.add_stdin(junction_0.add_stdout());
    launch_add_letter_o.add_stdin(junction_0.add_stdout());
    launch_filter_even_only.add_stdin(launch_add_letter_e.add_stdout());
    launch_filter_odd_only.add_stdin(launch_add_letter_o.add_stdout());
    junction_1.add_stdin(launch_filter_even_only.add_stdout(), false);
    junction_1.add_stdin(launch_filter_odd_only.add_stdout(), false);

    buffer.add_stdin(junction_1.add_stdout());
    slow_1.add_stdin(buffer.add_stdout());
    launch_decorate_exit.add_stdin(launch_line_numbers.add_exit_status());
    junction_2.add_stdin(launch_decorate_exit.add_stdout(), false);
    junction_2.add_stdin(slow_1.add_stdout(), false);
    drain.add_stdin(junction_2.add_stdout());

    let mut w = Waiter::new();

    w.add_drain("drain".to_string(), drain);
    w.add_faucet("faucet".to_string(), faucet);
    w.add_launch("launch_line_numbers".to_string(), launch_line_numbers);
    w.add_launch("launch_filter_odd_only".to_string(), launch_filter_odd_only);
    w.add_launch("launch_filter_even_only".to_string(), launch_filter_even_only);
    // w.add_launch("launch_sort".to_string(), launch_sort);
    w.add_launch("launch_add_letter_e".to_string(), launch_add_letter_e);
    w.add_launch("launch_add_letter_o".to_string(), launch_add_letter_o);
    w.add_launch("slow_0".to_string(), slow_0);
    w.add_launch("slow_1".to_string(), slow_1);
    w.add_launch("launch_decorate_exit".to_string(), launch_decorate_exit);
    w.add_buffer("buffer".to_string(), buffer);
    w.add_junction("junction_0".to_string(), junction_0);
    w.add_junction("junction_1".to_string(), junction_1);
    w.add_junction("junction_2".to_string(), junction_2);
    w.configure_faucet("faucet".to_string(), vec!["buffer".to_string()], 1, 2);

    w.start().await

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


    App::new("PipeAwesome")
        .author("Matthew Forrester, githib.com@speechmarks.com")
        .version("0.0.0")
        .about("Like UNIX pipes, but on sterroids")

        .subcommand(
            SubCommand::with_name("config")
                .arg(get_required_arg_with("config-in", "The config file to read, \"-\" for STDIN. If not specified it will be blank", "FILENAME"))
                .arg(get_required_arg_with("config-out", "The config file to write, \"-\" for STDOUT. Defaults to the file being read (otherwise STDOUT)", "FILENAME"))
                .subcommand(
                    SubCommand::with_name("empty")
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
                            SubCommand::with_name("dst")
                                .arg(Arg::from_usage("--destination=<DESTINATION> 'DESTINATION must be either a filename, \"-\" for STDIN, \"_\" for STDOUT or empty for NULL output'"))
                        )
                )

                .subcommand(
                    SubCommand::with_name("connection")
                        .arg(get_required_arg_with("id", "The ID of the connection to modify", "ID"))
                        .subcommand(
                            SubCommand::with_name("join")
                                .arg(Arg::from_usage("--join=<JOIN> 'The join to establish'"))
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
                                .arg(Arg::from_usage("--command=<COMMAND> 'What to execute'"))
                        )
                        .subcommand(
                            SubCommand::with_name("arg")
                            .subcommand(
                                SubCommand::with_name("add")
                                    .arg(Arg::from_usage("--val=<ARGUMENT>... 'The argument to add'"))
                            )
                            .subcommand(
                                SubCommand::with_name("clear")
                            )
                        )
                        .subcommand(
                            SubCommand::with_name("path")
                                .subcommand(
                                    SubCommand::with_name("set")
                                        .arg(Arg::from_usage("--path=<PATH> 'PATH will be the directory in which the COMMAND in launch-cmd is ran in'"))
                                )
                        )
                        .subcommand(
                            SubCommand::with_name("env")
                                .arg(Arg::from_usage("--id=<ID> 'The ID of the command to set (add / modify)'"))
                                .subcommand(
                                    SubCommand::with_name("add")
                                        .arg(Arg::from_usage("--name=<NAME>... 'The name of the environmental variable'"))
                                        .arg(Arg::from_usage("--val=<VALUE>... 'The value of the environmental variable'"))
                                )
                                .subcommand(
                                    SubCommand::with_name("clear")
                                )
                                .subcommand(
                                    SubCommand::with_name("remove")
                                        .arg(Arg::from_usage("--name=<NAME> 'The name of the environmental variable'"))
                                )
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
    config_out: String,
}

#[derive(Debug)]
enum UserConfigAction {
    FaucetSrc {
        base_options: UserConfigOptionBase,
        src: String
    },
    FaucetWatermark {
        base_options: UserConfigOptionBase,
        min: usize,
        max: usize,
    }
}


fn get_user_config_action<'a>(matches: &'a ArgMatches) -> Result<UserConfigAction, String> {

    #[derive(Debug)]
    struct CollectedSubcommands<'a> {
        subcommands: Vec<(&'a ArgMatches<'a>, &'a str)>,
        final_sub_command: &'a ArgMatches<'a>
    }

    fn collect_subcommands<'a>(matches: &'a ArgMatches) -> CollectedSubcommands<'a> {

        fn subcommand_inquire<'a>(mut v: CollectedSubcommands<'a>) -> CollectedSubcommands<'a> {
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

    fn get_standard_config_opts<'a>(first_sub_command: Option<&'a ArgMatches>, second_sub_command: Option<&'a ArgMatches>) -> Result<UserConfigOptionBase, String> {


        let first = first_sub_command.map(|sc1| sc1.value_of("config-in").or(Some("-"))).flatten();

        let standard_options = (
            first,
            first_sub_command.map(|sc1| sc1.value_of("config-out")).flatten().or(first),
            second_sub_command.map(|sc2| sc2.value_of("id")).flatten()
        );

        match standard_options {
            (Some(config_in), Some(config_out), Some(id)) => {
                Ok(UserConfigOptionBase { config_in: config_in.to_string(), config_out: config_out.to_string(), id: id.to_string() })
            },
            _ => Err("Somehow we didn't understand those commands".to_string())
        }

    }

    fn option_of_tuples_to_option_tuple<X>(x: (Option<X>, Option<X>)) -> Option<(X, X)> {
        if let (Some(a), Some(b)) = x {
            return Some((a, b));
        }
        None
    }


    fn get_user_action<'a>(mut collected_subcommands: CollectedSubcommands<'a>) -> Result<UserConfigAction, String> {
        use std::iter::FromIterator;

        let base_options = get_standard_config_opts(
            collected_subcommands.subcommands.iter().map(|x| x.0).nth(0),
            collected_subcommands.subcommands.iter().map(|x| x.0).nth(1)
        )?;

        let coll_subcomm_str: Vec<&str> = collected_subcommands.subcommands.iter().map(|x| x.1).collect();
        let last_sub_command: Option<&ArgMatches> = collected_subcommands.subcommands.iter().map(|x| x.0).last();

        match &coll_subcomm_str[..] {
            ["config", "faucet", "source"] => {
                last_sub_command
                    .map(|lsc| lsc.value_of("SOURCE")).flatten()
                    .map(|src| UserConfigAction::FaucetSrc { base_options, src: src.to_string() })
                    .ok_or("Command {:?} did not have all required values".to_string())
            },
            ["config", "faucet", "watermark"] => {
                last_sub_command
                    .map(|lsc| {
                        (
                            lsc.value_of("MIN").map(|a| a.parse::<usize>().ok()).flatten(),
                            lsc.value_of("MAX").map(|a| a.parse::<usize>().ok()).flatten()
                        )
                    })
                    .map(|tup| option_of_tuples_to_option_tuple((tup.0, tup.1))).flatten()
                    .map(|tup| UserConfigAction::FaucetWatermark { base_options, min: tup.0, max: tup.1 })
                    .ok_or("Command {:?} did not have all required values".to_string())
            },
            _ => {
                Err(format!("Unhandled: {:?}", coll_subcomm_str))
            }
        }
    }

    get_user_action(collect_subcommands(&matches))

}

fn read_config_as_str(config_in: &str) -> Result<String, String> {

    use std::io::prelude::*;
    let mut buffer = String::new();

    if config_in == "-" {
        let mut stdin = std::io::stdin(); // We get `Stdin` here.
        stdin.read_to_string(&mut buffer).map_err(|_x| "We could not read STDIN to get the config".to_string())?;
        return Ok(buffer);
    }

    let mut f = std::fs::File::open("config_in").map_err(|_x| format!("We could not open the file '{}' to get the config", config_in).to_string())?;
    f.read_to_string(&mut buffer).map_err(|_x| format!("We could not open the file '{}' to get the config", config_in).to_string())?;
    Ok(buffer)
}

fn parse_config_str(config_str: &str) -> Result<Config, String> {
    serde_json::from_str::<Config>(config_str).map_err(|_x| "Could not parse config".to_string())
}

fn read_config(user_action: &UserConfigAction) -> Result<Config, String> {
    match user_action {
        UserConfigAction::FaucetSrc { base_options: UserConfigOptionBase { config_in, ..}, .. } => {
            result_flatten(read_config_as_str(config_in).map(|cas| parse_config_str(&cas)))
        },
        UserConfigAction::FaucetWatermark { base_options: UserConfigOptionBase { id, config_in, config_out }, min, max } => {
            result_flatten(read_config_as_str(config_in).map(|cas| parse_config_str(&cas)))
        }
    }

}

fn result_flatten<X>(x: Result<Result<X, String>, String>) -> Result<X, String> {
    match x {
        Ok(Ok(x)) => Ok(x),
        Ok(Err(s)) => Err(s),
        Err(s) => Err(s),
    }
}

fn main() {

    let app = get_clap_app();
    let matches = app.get_matches();

    let config_and_action: Result<(Config, UserConfigAction), String> = match get_user_config_action(&matches) {
        Ok(ua) => read_config(&ua).map(|c| (c, ua)),
        Err(e) => Err(e),
    };


    let new_config = match config_and_action {
        Err(x) => Err(x),
        Ok((old_config, UserConfigAction::FaucetSrc { base_options: UserConfigOptionBase { id, .. }, src, .. })) => {
            Ok(Config::faucet_set_source(old_config, id, src))
        },
        Ok((old_config, UserConfigAction::FaucetWatermark { base_options: UserConfigOptionBase { id, .. }, min, max, .. })) => {
            Ok(Config::faucet_set_watermark(old_config, id, min, max))
        },
    };


    match result_flatten(new_config.map(|new_cfg| serde_json::to_string(&new_cfg).map_err(|_x| "Could not serialize new Config".to_string()))) {
        Err(msg) => {
            eprintln!("{:?}", msg);
            std::process::exit(1);
        }
        Ok(json) => {
            println!("{}", json);
        }
    }


}

// == Probably dead! ================================================

    fn as_kv<'a>(form: &str, s: &'a str) -> Result<(&'a str, &'a str), String> {
        s.split_once('=')
            .ok_or(format!("{} does not include a name (must be in form {})", s, form))
            .and_then(|(a, b)| {
                if a.len() == 0 {
                    return Err(format!("{} The {} form must include a non-zero length NAME", s, form));
                }
                Ok((a, b))
            })
    }


    fn as_kkv<'a>(form: &str, s: &'a str) -> Result<(&'a str, &'a str, &'a str), String> {
        let split: Vec<&str> = s.splitn(3, '=').collect();
        match split.len() {
            3 => Ok((split[0], split[1], split[2])),
            _ => Err(format!("{} The {} form must include a non-zero length NAME", s, form))
        }
    }


    // TODO: Must be above 0
    fn min_max_validator(form: &str, s: String) -> Result<KMinMax, String>{
        let (k, v) = as_kv(form, &s)?;

        v.split_once(',')
            .ok_or(format!("{} does not include a comma (must be in form {})", s, form))
            .and_then(|(a, b)| {

                match (a.parse::<usize>(), b.parse::<usize>()) {
                    (Ok(aa), Ok(bb)) => {
                        match aa > bb {
                            true => Ok(KMinMax {k: k.to_string(), min: bb, max: aa }),
                            false => Ok(KMinMax {k: k.to_string(), min: aa, max: bb }),
                        }
                    },
                    _ => Err(format!("{} Min and Max must both be positive integers", s)),
                }

            })

    }

    fn kv_validator(form: &str, s: String) -> Result<KV, String>{
        let (a, b) = as_kv(form, &s)?;
        Ok(KV {k: a.to_string(), v: b.to_string()})
    }

    fn kkv_validator(form: &str, s: String) -> Result<KKV, String>{
        let (a, b, c) = as_kkv(form, &s)?;
        Ok(KKV {kk: a.to_string(), k: b.to_string(), v: c.to_string()})
    }

    fn kv_arg_quick<'a>(name: &'a str, form: &'a str, help: &'a str, validator: fn(String) -> Result<(), String>) -> Arg<'a, 'a> {
        Arg::with_name(name)
            .long(name)
            .help(help)
            .required(false)
            .takes_value(true)
            .multiple(true)
            .value_name(form)
            .validator(validator)
    }

    fn non_empty_string_validator(form: &str, s: String) -> Result<String, String>{
        match s.len() {
            0 => Err(format!("{} The {} form must be a non-zero length", s, form)),
            _ => Ok(s),
        }
    }

struct KMinMax {
    k: String,
    min: usize,
    max: usize,
}

struct KV {
    k: String,
    v: String,
}

struct KKV {
    kk: String,
    k: String,
    v: String,
}


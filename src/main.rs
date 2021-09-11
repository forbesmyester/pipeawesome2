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

fn get_clap_app() -> App<'static, 'static> {


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


    App::new("PipeAwesome")
        .author("Matthew Forrester, githib.com@speechmarks.com")
        .version("0.0.0")
        .about("Like UNIX pipes, but on sterroids")
        .arg(Arg::with_name("in_file").index(1))

        .arg(kv_arg_quick("faucet-src-set", "ID=SOURCE", "SOURCE must be either a filename, \"-\" for STDIN, or empty for NULL input", |x| kv_validator("ID=SOURCE", x).map(|_x| ())))
        .arg(kv_arg_quick("faucet-min-max-set", "ID=MIN,MAX", "Try keep the size of all buffers pointing to ID between MIN and MAX", |x| min_max_validator("ID=MIN,MAX", x).map(|_x| ())))
        .arg(kv_arg_quick("faucet-min-max-clr", "ID", "Remove min / max setting from faucet ID", |x| non_empty_string_validator("ID", x).map(|_x| ())))

        .arg(kv_arg_quick("drain-dst-set", "ID=DESTINATION", "DESTINATION must be either a filename, \"-\" for STDIN, \"_\" for STDOUT or empty for NULL output", |x| kv_validator("ID=DESTINATION", x).map(|_x| ())))

        .arg(kv_arg_quick("launch-cmd-set", "ID=COMMAND", "COMMAND is the command that will be executed", |x| kv_validator("ID=COMMAND", x).map(|_x| ())))
        .arg(kv_arg_quick("launch-arg-add", "ID=ARGUMENT", "ARGUMENT that will be an argument to COMMAND in launch-cmd", |x| kv_validator("ID=ARGUMENT", x).map(|_x| ())))
        .arg(kv_arg_quick("launch-arg-clr", "ID", "Remove all arguments from launch ID", |x| non_empty_string_validator("ID", x).map(|_x| ())))
        .arg(kv_arg_quick("launch-path-set", "ID=PATH", "PATH will be the directory in which the COMMAND in launch-cmd is ran in", |x| kv_validator("ID=PATH", x).map(|_x| ())))
        .arg(kv_arg_quick("launch-env-add", "ID=NAME=VALUE", "Sets the environmental variable NAME to VALUE for the COMMAND in launch-cmd", |x| kkv_validator("ID=NAME=VALUE", x).map(|_x| ())))
        .arg(kv_arg_quick("launch-env-clr", "ID", "Remove all environmental variables from launch ID", |x| non_empty_string_validator("ID", x).map(|_x| ())))

        .arg(kv_arg_quick("connection-add", "ID=CONNECTION", "Join things", |x| kv_validator("ID=CONNECTION", x).map(|_x| ())))
        .arg(kv_arg_quick("connection-del", "ID", "Remove connection with ID", |x| non_empty_string_validator("ID", x).map(|_x| ())))
        .group(ArgGroup::with_name("launch")
             .args(&["launch-cmd-set", "launch-arg-add", "launch-arg-clr", "launch-path-set", "launch-env-add", "launch-env-clr"])
        )
        .after_help("Longer explanation to appear after the options when \
                     displaying the help information from --help or -h")
}

fn main() {

    let app = get_clap_app();
    app.get_matches();

    // assert_eq!(list_parser::list("[1,1,2,3,5,8]"), Ok(vec![1, 1, 2, 3, 5, 8]));



    // println!("{:?}", task::block_on(pipeawesome2::tap::test_tap_impl()));
    // println!("{:?}", task::block_on(pipeawesome2::buffer::test_buffer_impl()));
    // println!("{:?}", task::block_on(pipeawesome2::junction::test_junction_impl()));
    // println!("{:?}", task::block_on(do_stuff()));


}

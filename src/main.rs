use std::collections::HashMap;
// use std::time::{ Duration, Instant };
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


fn main() {

    // let (input_chan_snd, input_chan_rcv) = bounded(8);


    // println!("{:?}", task::block_on(pipeawesome2::tap::test_tap_impl()));
    // println!("{:?}", task::block_on(pipeawesome2::buffer::test_buffer_impl()));
    // println!("{:?}", task::block_on(pipeawesome2::junction::test_junction_impl()));
    println!("{:?}", task::block_on(do_stuff()));
    // use pipeawesome2::buffer;



    // let (mut data_storage) = &DataStorage::new();

    // let pre_data_send = PreData { data: [0; 255], len: 0, source: Source (0, Port::TAP) };
    // let post_data_send = data_storage.push(pre_data_send, 1);

    // sender.send(post_data_send);

    // std::thread::spawn(move || {
    //     println!("{:?}", reciever.recv().unwrap());
    // });
}

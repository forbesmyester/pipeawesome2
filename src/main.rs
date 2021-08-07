use std::collections::HashMap;
// use std::time::{ Duration, Instant };
use async_std::io as aio;
use pipeawesome2::{buffer::Buffer, drain::Drain, faucet::Faucet, junction::Junction, launch::Launch, waiter::Waiter, motion::{ Pull, Push, MotionResult } };
use async_std::task;

async fn do_stuff() -> MotionResult<usize> {

    let stdin = aio::stdin();
    let stdout = aio::stdout();
    let stderr = aio::stderr();

    let mut faucet = Faucet::new(Pull::Stdin(stdin));
    let mut junction_0 = Junction::new();
    let mut junction_1 = Junction::new();
    let mut drain = Drain::new(Push::Stderr(stderr));
    let mut buffer = Buffer::new();

    let mut launch_line_numbers: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "awk".to_string(),
        Some(vec![r#"{ print sprintf("%04d"R, NR) " " $0 }"#.to_string()])
    );

    let mut launch_filter_odd_only: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "awk".to_string(),
        Some(vec!["-f".to_string(), "res/modulus_print.awk".to_string(), "-v".to_string(), "rem=1".to_string()])
    );

    let mut launch_filter_even_only: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "awk".to_string(),
        Some(vec!["-f".to_string(), "res/modulus_print.awk".to_string(), "-v".to_string(), "rem=0".to_string()])
    );

    let mut launch_sort: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "sort".to_string(),
        Some(vec![])
    );

    let mut launch_add_letter_o: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "awk".to_string(),
        Some(vec!["-v".to_string(), "letter=O".to_string(), r#"{ printf("%s %s: ", $1, letter); for (i=2; i<=NF; i++) printf("%s", $i); print "" }"#.to_string()])
    );

    let mut launch_add_letter_e: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "awk".to_string(),
        Some(vec!["-v".to_string(), "letter=E".to_string(), r#"{ printf("%s %s: ", $1, letter); for (i=2; i<=NF; i++) printf("%s", $i); print "" }"#.to_string()])
    );


    launch_line_numbers.add_stdin(faucet.add_stdout());
    junction_0.add_stdin(launch_line_numbers.add_stdout());
    launch_add_letter_e.add_stdin(junction_0.add_stdout());
    launch_add_letter_o.add_stdin(junction_0.add_stdout());
    launch_filter_even_only.add_stdin(launch_add_letter_e.add_stdout());
    launch_filter_odd_only.add_stdin(launch_add_letter_o.add_stdout());
    junction_1.add_stdin(launch_filter_even_only.add_stdout());
    junction_1.add_stdin(launch_filter_odd_only.add_stdout());
    launch_sort.add_stdin(junction_1.add_stdout());
    buffer.add_stdin(launch_sort.add_stdout());
    drain.add_stdin(buffer.add_stdout());

    let mut w = Waiter::new();

    w.add_drain(drain);
    w.add_faucet(faucet);
    w.add_launch(launch_line_numbers);
    w.add_launch(launch_filter_odd_only);
    w.add_launch(launch_filter_even_only);
    w.add_launch(launch_sort);
    w.add_launch(launch_add_letter_e);
    w.add_launch(launch_add_letter_o);
    w.add_buffer(buffer);
    w.add_junction(junction_0);
    w.add_junction(junction_1);

    w.start().await

}


fn main() {

    // let (input_chan_snd, input_chan_rcv) = bounded(8);


    // task::block_on(pipeawesome2::tap::test_tap_impl());
    println!("{:?}", task::block_on(do_stuff()));
    // use pipeawesome2::buffer;
    // println!("{:?}", task::block_on(buffer::test_buffer_impl()));



    // let (mut data_storage) = &DataStorage::new();

    // let pre_data_send = PreData { data: [0; 255], len: 0, source: Source (0, Port::TAP) };
    // let post_data_send = data_storage.push(pre_data_send, 1);

    // sender.send(post_data_send);

    // std::thread::spawn(move || {
    //     println!("{:?}", reciever.recv().unwrap());
    // });
}

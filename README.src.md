# Pipeawesome 2

## Table of contents

## As my mum would say... accusingly... "WHAT did YOU do?!?!?!"

I added loops, branches and joins to UNIX pipes.

## Why did you do this?

I feel UNIX pipes are underappreciated and should be considered more often.

## So why do you love UNIX pipes?

UNIX pipes are wonderful as when you write software using them they have:

 * High performance.
 * Back Pressure.
 * Really easy to reason about at the individual UNIX process level (it's just STDIN and STDOUT/STDERR).
 * Insanely light and zero infrastructure compared to a "proper" solution.
 * Easily to integrate with a "proper" solution when the need arises.

## So what does this project add?

Given a UNIX pipeline like `cat myfile | awk 'PAT { do something }' | grep '^good' | awk '$1='good' { print /dev/stdout } else { print /dev/stderr }' | sed 's/good/perfect' | sort` and this is powerful and wonderful.

This could be visualized like the following:

```unixpipe diagram-dot svg readme-img/so-what-does-this-project-add-1.svg
digraph {
    rankdir = LR;
    cat -> awk -> grep2 -> awk2 -> sed -> sort

    cat [label="cat myfile", shape="box"]
    grep2 [label="grep '^good'", shape="box"]
    awk [label="awk 'PAT { do something }'", shape="box"]
    sort [shape="box"]
    sed [label="sed 's/good/perfect/'", shape="box"]
    awk2 [label="awk '\l$1='good' {\l    print /dev/stdout; next\l}{\l    print /dev/stderr\l}'\l", shape="box"]
}
```

However it might be that you want to:

```unixpipe diagram-dot svg readme-img/so-what-does-this-project-add-2.svg
digraph {
    rankdir = LR;
    cat -> awk -> grep1 -> do -> awk2 -> sed -> sort
    awk -> grep2 -> awk2
    awk2 -> awk [label="STDERR"]

    cat [label="cat myfile", shape="box"]
    grep2 [label="grep '^good'", shape="box"]
    awk [label="awk 'PAT { do something }'", shape="box"]
    sort [shape="box"]
    sed [label="sed 's/good/perfect/'", shape="box"]
    awk2 [label="awk '\l$1='good' {\l    print /dev/stdout; next\l}{\l    print /dev/stderr\l}'\l", shape="box"]
    grep1 [label="grep '^bad'", shape="box"]
    do [label="do 'further processing'", shape="box"]
}
```

Some or all of this is possible to do with tools like `mkfifo`, depending on your skill level, but you certainly won't end up with something that is anywhere near as easy for someone to follow as the simple UNIX command shown earlier.

If the idea of being able to put together more complex pipelines easily interests you, read on!

## An example project

I decided to make a tic-tac-toe game to demonstrate the capabilities of this project.

This game would have:

 * A data format which can be translated into **a human recognizable tic-tac-toe grid** with squares filled in.
 * Two **computer players**.
 * They would **take turns to pick a blank square** and fill it with their "X" or "O".
 * A **referee** to decide when the game had been won or was a draw.

### Data Format

I first decided on a data format which looks like the following:

STATUS **:** SQUARE_1 **:** SQUARE_2 **:** SQUARE_3 **:** SQUARE_4 **:** SQUARE_5 **:** SQUARE_6 **:** SQUARE_7 **:** SQUARE_8 **:** SQUARE_9

In this `STATUS` would be either `X`, `O` for player turns or *something else* to denote game draws / wins.

### Drawing the grid

I next coded up something which would show the grid. I coded a lot of this in GNU AWK because it's something that I'm learning off-and-on and (very) simple `STDIN | STDOUT` coding seems ideally suited to the language.

I came up with the following code:

```awk file=./examples/tic-tac-toe/draw.awk
```

You can execute this code with `echo O:::X::O::::X' | gawk -F ':' -f ./examples/tic-tac-toe/draw.awk` and it'll draw you the following grid:

```unixpipe echo 'O:::X::O::::X' | gawk -F ':' -f ./examples/tic-tac-toe/draw.awk | wrap-as-lang text
```

I then wrote a Pipeawesome configuration file which wraps this:

```yaml file=./examples/tic-tac-toe/draw.pa.yaml
```
Which could be visualized as:

```unixpipe ./target/debug/pipeawesome2 graph --config ./examples/tic-tac-toe/draw.pa.yaml -d | diagram-dot svg readme-img/drawing-the-grid.svg
```

<sub>**NOTE**: I got Pipeawesome drew this graph by running `./target/debug/pipeawesome2 graph --config examples/tic-tac-toe/draw.pa.yaml --diagram-only`.</sub>

<sub>**NOTE**: Using `./target/debug/pipeawesome2 graph --config examples/tic-tac-toe/draw.pa.yaml --legend-only` will generate the graphs legend, this is common for all graphs and shown [in the appendix](#pipeawesome-graph-legend).</sub>

In Pipeawesome there are pipes, which connect different types of components. The components types here are `faucet`, `launch` and `drain` with the names of those components being `input`, `draw` and `output`. The names are just names, but they may need to be referenced elsewhere within the configuration file depending on the component type.

You could execute this Pipeawesome configuration file with `echo 'O:::X::O::::X' | ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/draw.pa.yaml`

This is of course, a trivial and pointless example because you'd run awk directly, but it allows me to show you the Pipeawesome file format with minimal complexity.

Lets break it down into it's constituent parts:

#### Connection / Connection Sets

```yaml
connection:
  initial: "faucet:input | launch:draw | drain:output"
```

Connection sets explain how to join components together. There can be multiple connection sets, but here there is just one.

For more information please see [Pipe Variations, Output Types and Input Priorities.](#pipe-variations-output-types-and-input-priorities).

#### Faucet

```yaml
faucet:
  input:
    source: '-'
```

A Faucet is the main way to get data into Pipeawesome from the outside world, the configuration here is for the one named `input`.

For more information please see [Component: Faucet](#component-faucet).

<sub>**NOTE**: It is perfectly valid for a Launch to also generate the initial data, in which case a Faucet would not be required.</sub>

#### Launch

```yaml
launch:
  draw:
    cmd: "awk"
    arg:
      - '-F'
      - ':'
      - '-f'
      - 'examples/tic-tac-toe/draw.awk'
```

This controls how the programs are executed.

For more information please see [Component: Launch](#component-launch).

#### Drain

```yaml
drain:
  output:
    destination: '-'
```

This is how data exits Pipeawesome. Output can be sent to STDOUT, STDERR or a file.

For more information please see [Component: Drain](#component-drain).

<sub>**NOTE**: If writing to a queueing solution such as RabbitMQ or AWS SQS you could use a Launch instead.</sub>

### Having a Go

Let's let the player take their turn.

The following configuration includes new code, but the configuration adds no concepts that you've not already seen:

```yaml file=./examples/tic-tac-toe/have_a_go.pa.yaml
```

Which could be visualized as:

```unixpipe ./target/debug/pipeawesome2 graph --config ./examples/tic-tac-toe/have_a_go.pa.yaml -d | diagram-dot svg readme-img/having-a-go.svg
```

It can be executed with `echo 'O:::X::O::::X' | ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/have_a_go.pa.yaml`

The output from this will be the same as previous but with an extra `O` somewhere on the grid: 

```unixpipe echo 'O:::X::O::::X' | ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/have_a_go.pa.yaml | wrap-as-lang text
```

<sub>**NOTE**: There is one extra `O` than in in the input, this was added by `player.awk`.</sub>

### Picking a random player to start the game

#### Code for generating the random player
I figured out that `echo $((RANDOM % 2))::::::::: | sed "s/1/X/" | sed "s/0/O/"` is a single line BASH snippet for selecting a random first player.

However this still means I have to let the selected player take that turn, which means I must explain what a **Junction** is.

A **Junction** is a prioritized many-to-many connector. Anything that comes into any one of it's inputs will be sent to all of it's outputs.

For more information please see [Component: Junction](#component-junction).

After adding the junctions and supporting changes, the full configuration looks like this:

```yaml file=./examples/tic-tac-toe/random_player.pa.yaml
```

The graphs drawn by Pipeawesome now become much more interesting:

```unixpipe ./target/debug/pipeawesome2 graph --config examples/tic-tac-toe/random_player.pa.yaml -d | diagram-dot svg readme-img/code-for-generating-the-random-player.svg
```

The changes are:

The Faucet configuration has been completely removed (it is not required), in this situation, the initial message comes from `l:random_player`. I have also added / changed some Launch.

The big change is that there are now multiple keys / connections sets / lines within `connection:`. You may notice that the `random_selection:` connection set writes to `junction:turn` but `junction:turn` is read in both the `player_o_branch` and `player_x_branch` connection sets, which in turn both write to `junction:draw`. The connection set names are completely arbitrary, though they must be unique.

<sub>**NOTE**: It is important to know that both `l:player_o_filter` and `l:player_x_filter` both recieve the lines generated by `l:random player`. It is just the case that one of them always filters it out.</sub>

Running this code results in a grid where either an `O` or `X` could be anywhere on the grid:

```unixpipe ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/random_player.pa.yaml | wrap-as-lang text
```

### A complete game

To create the full game, there are two more things that need to happen:

 1. Multiple turns - To complete a game we must have multiple turns take place.
 2. Alternating players - The player that takes the next turn must different from the previous turn.

#### Multiple turns

This is simple, all we need to do is take our previous configuration, add a junction between `launch:referee` and `launch:draw` and feed a new branch all the way back into `junction:turn`. The configuration now looks like:

```yaml file=examples/tic-tac-toe/multiple_turns.pa.yaml -d
```

Which could be visualized as:

```unixpipe ./target/debug/pipeawesome2 graph --config examples/tic-tac-toe/multiple_turns.pa.yaml -d | diagram-dot svg readme-img/multiple-turns.svg
```

<sub>**NOTE:** This graph is identical except the extra `junction:loop` and the line from it that goes all the way back to turn (connection set `looper`).</sub>

This configuration results in a non-thrilling game however as only one player ever gets a go!

```unixpipe ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/multiple_turns.pa.yaml | wrap-as-lang text
```

#### Alternating players

To get the player taking a turn to alternate we just need to put in some code that swaps the first character between "X" and "O between `junction:loop` and `junction:turn`. This component is called `turn_swapper` in the configuration below:

```yaml file=examples/tic-tac-toe/pa.yaml -d
```

Which could be visualized as:

```unixpipe ./target/debug/pipeawesome2 graph --config examples/tic-tac-toe/pa.yaml -d | diagram-dot svg readme-img/alternating-players.svg
```

The end result is a (somewhat) realisic looking game of tic-tac-toe where the players take turns and someone wins (or the game ends in a draw):

```unixpipe ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/pa.yaml | wrap-as-lang text
```

## Component Types

Component types can be:

 * [**Faucet**](#component-faucet): A source of input when it comes from outside.
 * [**Launch**](#component-launch): A running program that can process data.
 * [**Drain**](#component-drain): Data written here exists Pipeawesome.
 * [**Junction**](#component-junction): A many to many connector which can manage priorities of incoming data.
 * [**Buffer / Regulator**](#component-buffer--regulator): Stores an infinite amount of messages / Regulates the amount of messages

<sub>**Note:** There are diagrams in this section, the legend for this is shown at [#component-diagram-legend](#component-diagram-legend)</sub>

### Component: Faucet

```unixpipe diagram-dot svg readme-img/component-faucet.svg
digraph G {
    rankdir=LR
    labeljust=l
    fontsize=20

    subgraph faucet {
        color=lightgrey;
        subgraph cluster_faucet {
            label="Faucet"
            faucet_pull [label="pull: file/stdin"]
            faucet_push [label=push]
            style="rounded"
        }
        fauct_exit_pull [shape=plaintext, label=""]
        faucet_pull -> faucet_push
        faucet_push -> fauct_exit_pull [style=dashed]
    }
}
```

```
faucet:
  tap:
    input: "-",
```

A Faucet is the main way to get data into Pipeawesome. Faucets have a property called `source` which can be "`-`" for STDIN or a filename which will be read from.

### Component: Launch

```unixpipe diagram-dot svg readme-img/component-launch.svg
digraph G {
    rankdir=LR
    labeljust=l
    fontsize=20
    
    subgraph launch {
        color=lightgrey;
        subgraph cluster_launch {
            label="Launch"
            launch_pull [label=pull]
            launch_stdin_recv_push [label=push]
            
            subgraph cluster_spawn_holder {
                color=white
                label=""
                subgraph cluster_launch_spawn {
                    color=grey
                    label="child.spawn"
                    launch_stdin_recv_pull [label="pull:stdin"]
                    launch_stdout_send_pull [label="pull:stdout"]
                    launch_stderr_send_pull [label="pull:stdout"]
                    style="filled"
                }
                launch_exit_send_push [label="push:exit"]
            }
            launch_stderr_recv_push [label="push"]
            launch_stdout_recv_push [label="push"]
            style="rounded"
        }
        launch_input [shape=plaintext, label=""]
        launch_input -> launch_pull [style="dashed"]
        launch_pull -> launch_stdin_recv_push
        launch_stdin_recv_push -> launch_stdin_recv_pull [style=dashed]
        launch_stdin_recv_pull -> launch_stdout_send_pull [style=dotted, arrowhead=none]
        launch_stdin_recv_pull -> launch_stderr_send_pull [style=dotted, arrowhead=none]
        
        launch_stdout_send_pull -> launch_stdout_recv_push
        launch_stderr_send_pull -> launch_stderr_recv_push
        launch_stdout_recv_push -> launch_stdout_outer [style=dashed]
        launch_stderr_recv_push -> launch_stderr_outer [style=dashed]
        launch_exit_send_push -> launch_exit_outer [style=dashed]
        subgraph cluster_launch_outputs {
            color=white
            launch_stderr_outer [shape=plaintext, label=""]
            launch_stdout_outer [shape=plaintext, label=""]
            launch_exit_outer [shape=plaintext, label=""]
        }
    }

}
```

```yaml
launch:
  draw:
    cmd: "awk"
    env:
      AWKLIBPATH: "./lib"
    path: "/home/forbesmyester/project/awesome"
    arg:
      - '-F'
      - ':'
      - '-f'
      - 'examples/tic-tac-toe/draw.awk'
```

This controls how a program is executed.

The following are configurable:

 * **cmd**: The command to run
 * **path**: Where to run it
 * **env**: The environmental variables to run it in
 * **arg**: arguments that will be passed through to the the command

### Component: Drain

```unixpipe diagram-dot svg readme-img/component-drain.svg
digraph G {
    rankdir=LR
    labeljust=l
    fontsize=20
    
    subgraph drain {
        color=lightgrey;
        drain_input [shape=plaintext, label=""]
        drain_input -> drain_pull [style="dashed"]
        subgraph cluster_drain {
            label = "Drain"
            drain_pull [label=pull]
            drain_push [label="push: file/stdout"]
            style="rounded"
        }
        drain_pull -> drain_push
    }
}
```

```yaml
drain:
  output:
    destination: '-'
```

This is the normal way to get data out from Pipeawesome. the output can be sent to "`-`" for STDOUT, "`_`" for STDERR or a file, which is specified by using any other value.

### Component: Junction

```unixpipe diagram-dot svg readme-img/component-junction.svg
digraph G {
    rankdir=LR
    labeljust=l
    fontsize=20
    
    subgraph junction {
        color=lightgrey;
        subgraph cluster_junction {
            label = "Junction"
            junction_push_1 [label=push]
            junction_push_2 [label=push]
            junction_pull_1 [label=pull]
            junction_pull_2 [label=pull]
            style="rounded"
        }
        junction_input_outer_1 [shape="plaintext", label=""]
        junction_input_outer_2 [shape="plaintext", label=""]
        junction_input_outer_1 -> junction_pull_1 [style=dashed]
        junction_input_outer_2 -> junction_pull_2 [style=dashed]
        junction_pull_1 -> junction_push_1
        junction_pull_2 -> junction_push_1
        junction_pull_1 -> junction_push_2
        junction_pull_2 -> junction_push_2
        junction_exit_pull_1 [shape=plaintext, label=""]
        junction_exit_pull_2 [shape=plaintext, label=""]
        junction_push_1 -> junction_exit_pull_1 [style=dashed]
        junction_push_2 -> junction_exit_pull_2 [style=dashed]
    }
}
```

A **Junction** is a many-to-many connector. Anything that comes into one of it's inputs will be sent to all of it's outputs.

There's no configuration for **Junction**, however it is the only component that has any reason to respect input priorities.

<sub>**NOTE:** Messages are considered to be seperated by Windows or UNIX line endings. It would be realitvely easy to make this configurable.</sub>

### Component: Buffer & Regulator

```unixpipe diagram-dot svg readme-img/component-buffer.svg
digraph G {
    rankdir=LR
    labeljust=l
    fontsize=20
    
    subgraph buffer {
        color=lightgrey;
        buffer_input_push [shape=plaintext, label=""]
        subgraph cluster_buffer {
            label = "Buffer"
            buffer_pull [label=pull]
            buffer_push [label=push]
            buffer_push -> buffer_inner_pull [style=dashed, arrowhead=empty]
            buffer_inner_pull [label=pull]
            buffer_inner_push [label=push]
            buffer_inner_pull -> buffer_inner_push
            style="rounded"
        }
        buffer_input_push -> buffer_pull [style=dashed]
        buffer_pull -> buffer_push
        buffer_exit_pull [shape=plaintext, label=""]
        buffer_inner_push -> buffer_exit_pull [style=dashed]
    }
}
```

```unixpipe diagram-dot svg readme-img/component-regulator.svg
digraph G {
    rankdir=LR
    labeljust=l
    fontsize=20
    
    subgraph regulator {
        color=lightgrey;
        regulator_input_push [shape=plaintext, label=""]
        subgraph cluster_regulator {
            label = "Regulator"
            regulator_pull [label=pull]
            regulator_push [label=push]
            regulator_note_point [shape=point, label=""]
            style="rounded"
        }
        regulator_input_push -> regulator_pull [style=dashed]
        regulator_pull -> regulator_note_point [arrowhead=none]
        regulator_note_point -> regulator_push
        regulator_exit_pull [shape=plaintext, label=""]
        regulator_push -> regulator_exit_pull [style=dashed]
        regulator_note [style="filled", fillcolor=lightgoldenrod1, label="Pipeawesome controls the flow\nbetween the pull and the push", shape=note]
        regulator_note -> regulator_note_point [penwidth=0.3, dir=none]
    }
}
```

The only components I have not used in the tic-tac-toe example are the **Buffer** and **Regulator**.

Looking back at some of the tic-tac-toe configurations after [Multiple Turns](#multiple-turns) you can see a message is often sent to near the start of the process from very close to the end of the process.

If there are a finite number of messages coming into the system, as in a single game of tic-tac-toe then this is not a problem. However if more and more messages are being added to the system and all the old messages are still being processed, you can see that may be an issue. This situation is unique to loops and is usually handled via backpressure.

The connectors between components have finite capacity ([they are async version of bounded Rust channels](https://docs.rs/async-std/1.9.0/async_std/channel/fn.bounded.html)), which is how our backpressure works, but it does leave a form of deadlock being possible in configurations that include loops.

Below I describe the two components that can be used to control this situation:

 * A **Buffer** is connector with infinite message capacity (it is an [unbounded Rust channel](https://docs.rs/async-std/1.9.0/async_std/channel/fn.unbounded.html)).
 * The **Regulator** can turn on and off the flow of messages passing through it by observing how many messages are in a configured set of Buffers.

The configuration below is a version of the tic-tac-toe configuration above, but modified to run 100,000 games:

```yaml file=./examples/tic-tac-toe/many_games.pa.yaml
```

Which could be visualized as:

```unixpipe ./target/debug/pipeawesome2 graph --config ./examples/tic-tac-toe/many_games.pa.yaml -d | diagram-dot svg readme-img/loop-buffer-and-regulator.svg
```

In this situation, the `l:random_player` is creating 100,000 messages instead of just 1, but every one of these will loop back around until the game is complete, so we will have a significant amount of messages. In this configuration, when there are more than 100 messages in `buffer:reprocess`, the `regulator:regulate_flow` will stop accepting messages, but when the amount of messages in the buffer drops below 10, it will resume. Using these two components can solve the problem of issue described above.

## Pipe Variations, Output Types and Input Priorities.

### Pipe Variations

Thinking about passing data between programs, when the recieving program dies or closes it's input, we end up with data coming out from the sending program, but with nowhere for it to go. In this situation I think we have three options:

 1. Terminate (T): Terminate Pipeawesome.
 1. Finish (F): Close the pipe - letting the sending program deal with the problem itself (this will likely cause a cascade effect).
 2. Consume (C): Pipeawesome will keep the pipe open by consuming the data itself (but discarding it).

You can specify the the pipe variation using:

 * `l:sender |T| l:reciever` - Terminate.
 * `l:sender |F| l:reciever` - Finish.
 * `l:sender |C| l:reciever` - Consume.

The normal, single pipe symbol (`l:sender | l:reciever`) is merely a quick way of writing "`|T|`".

### Output Types

A running program may not output all it's output to STDOUT, it can also send data to STDERR.

Pipeawesome allows you to capture when programs output to STDOUT and STDERR but also the EXIT code when the program finishes. This is done by using `[O]`, `[E]` and `[X]` just after the component in the connection set, for example:

```yaml file=./examples/ls/pa.yaml
```

<sub>**NOTE:** Infact on UNIX programs can also read & write to `/dev/fdN` where N can be 0 (STDIN), 1 (STDOUT), 2 (STDERR) or other values of N. These other values of N are not currently directly supported.</sub>

### Input Priorities

All components except Junction only have one input, but a Junction can have multiple. To control which input to read from we can add priorities, these are specified like the following:

```yaml
connection:
  high_priority: "launch:one_thing | [5]junction:many_to_many"
  low_priority: "launch:something_else | [1]junction:many_to_many"
```

In this example 5 and 1 are priorities, when priorities are not specified, they will be 0. Priorities can also be negative.

## Appendix

### Pipeawesome Graph Legend

You can draw a graph legend by running the command `./target/debug/pipeawesome2 graph --config [YOUR_CONFIG_HERE] --legend-only`. The output will be Graphviz DOT.

```unixpipe ./target/debug/pipeawesome2 graph --config examples/tic-tac-toe/random_player.pa.yaml --legend-only | diagram-dot svg readme-img/pipeawesome-graph-legend.svg
```

### Component Diagram Legend

This is the legend for diagrams shown in the [Component Types](#component-types) section.

```unixpipe diagram-dot svg readme-img/component-legends.svg
digraph G {
    rankdir=LR
    subgraph cluster_legend {
        label="Legend"
        a [label="", width=0.3, height=0.3]
        b [label="", width=0.3, height=0.3]
        c [label="", width=0.3, height=0.3]
        d [label="", width=0.3, height=0.3]
        e [label="", width=0.3, height=0.3]
        f [label="", width=0.3, height=0.3]
        g [label="", width=0.3, height=0.3]
        h [label="", width=0.3, height=0.3]
        a -> b [label=motion]
        c -> d [label=io, style=dotted, arrowhead=none]
        e -> f [label="channel synchronous", style=dashed]
        g -> h [label="channel asynchronous", style=dashed, arrowhead=empty]
    }
}
```



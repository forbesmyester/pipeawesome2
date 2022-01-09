# Pipeawesome 2

## As my mum would say... accusingly... "WHAT did YOU do?!?!?!"

I added loops, branches and joins to UNIX pipes.

## Why did you do this?

I feel UNIX pipes are underappreciated and could be destined for much more.

## So why do you love UNIX pipes?

UNIX pipes are wonderful as when you write software using them they have:

*   High performance.
*   Back Pressure.
*   Really easy to reason about at the individual UNIX process level (it's just STDIN and STDOUT/STDERR).
*   Easy to reason about what data enters individual processes.
*   Insanely light and zero infrastructure compared to a "proper" solution.
*   Easily to integrate with a "proper" solution when the need arises.

## So what does this project add?

So we write a UNIX pipeline like `cat myfile | awk 'PAT { do something }' | grep '^good' | awk '$1='good' { print /dev/stdout } else { print /dev/stderr }' | sed 's/good/perfect' | sort` and this is powerful and wonderful.

This could be visualized like the following:

<svg width="220pt" height="466pt" viewBox="0.00 0.00 220.00 466.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 462)"><title>%3</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-462 216,-462 216,4 -4,4"/><!-- cat --><g id="node1" class="node"><title>cat</title><polygon fill="none" stroke="black" points="149,-458 63,-458 63,-422 149,-422 149,-458"/><text text-anchor="middle" x="106" y="-436.3" font-family="Times,serif" font-size="14.00">cat myfile</text></g><!-- awk --><g id="node2" class="node"><title>awk</title><polygon fill="none" stroke="black" points="212,-386 0,-386 0,-350 212,-350 212,-386"/><text text-anchor="middle" x="106" y="-364.3" font-family="Times,serif" font-size="14.00">awk 'PAT { do something }'</text></g><!-- cat&#45;&gt;awk --><g id="edge1" class="edge"><title>cat->awk</title><path fill="none" stroke="black" d="M106,-421.7C106,-413.98 106,-404.71 106,-396.11"/><polygon fill="black" stroke="black" points="109.5,-396.1 106,-386.1 102.5,-396.1 109.5,-396.1"/></g><!-- grep2 --><g id="node3" class="node"><title>grep2</title><polygon fill="none" stroke="black" points="160,-314 52,-314 52,-278 160,-278 160,-314"/><text text-anchor="middle" x="106" y="-292.3" font-family="Times,serif" font-size="14.00">grep '^good'</text></g><!-- awk&#45;&gt;grep2 --><g id="edge2" class="edge"><title>awk->grep2</title><path fill="none" stroke="black" d="M106,-349.7C106,-341.98 106,-332.71 106,-324.11"/><polygon fill="black" stroke="black" points="109.5,-324.1 106,-314.1 102.5,-324.1 109.5,-324.1"/></g><!-- awk2 --><g id="node4" class="node"><title>awk2</title><polygon fill="none" stroke="black" points="203.5,-242 8.5,-242 8.5,-144 203.5,-144 203.5,-242"/><text text-anchor="start" x="16.5" y="-226.8" font-family="Times,serif" font-size="14.00">awk '</text><text text-anchor="start" x="16.5" y="-211.8" font-family="Times,serif" font-size="14.00">$1='good' {</text><text text-anchor="start" x="16.5" y="-196.8" font-family="Times,serif" font-size="14.00">    print /dev/stdout; next</text><text text-anchor="start" x="16.5" y="-181.8" font-family="Times,serif" font-size="14.00">}{</text><text text-anchor="start" x="16.5" y="-166.8" font-family="Times,serif" font-size="14.00">    print /dev/stderr</text><text text-anchor="start" x="16.5" y="-151.8" font-family="Times,serif" font-size="14.00">}'</text></g><!-- grep2&#45;&gt;awk2 --><g id="edge3" class="edge"><title>grep2->awk2</title><path fill="none" stroke="black" d="M106,-277.87C106,-270.59 106,-261.65 106,-252.34"/><polygon fill="black" stroke="black" points="109.5,-252.15 106,-242.15 102.5,-252.15 109.5,-252.15"/></g><!-- sed --><g id="node5" class="node"><title>sed</title><polygon fill="none" stroke="black" points="186,-108 26,-108 26,-72 186,-72 186,-108"/><text text-anchor="middle" x="106" y="-86.3" font-family="Times,serif" font-size="14.00">sed 's/good/perfect/'</text></g><!-- awk2&#45;&gt;sed --><g id="edge4" class="edge"><title>awk2->sed</title><path fill="none" stroke="black" d="M106,-143.87C106,-135.06 106,-126.17 106,-118.24"/><polygon fill="black" stroke="black" points="109.5,-118.15 106,-108.15 102.5,-118.15 109.5,-118.15"/></g><!-- sort --><g id="node6" class="node"><title>sort</title><polygon fill="none" stroke="black" points="133,-36 79,-36 79,0 133,0 133,-36"/><text text-anchor="middle" x="106" y="-14.3" font-family="Times,serif" font-size="14.00">sort</text></g><!-- sed&#45;&gt;sort --><g id="edge5" class="edge"><title>sed->sort</title><path fill="none" stroke="black" d="M106,-71.7C106,-63.98 106,-54.71 106,-46.11"/><polygon fill="black" stroke="black" points="109.5,-46.1 106,-36.1 102.5,-46.1 109.5,-46.1"/></g></g></svg>

However it might be that you want to:

<svg width="313pt" height="579pt" viewBox="0.00 0.00 313.00 579.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 575)"><title>%3</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-575 309,-575 309,4 -4,4"/><!-- cat --><g id="node1" class="node"><title>cat</title><polygon fill="none" stroke="black" points="211.5,-571 125.5,-571 125.5,-535 211.5,-535 211.5,-571"/><text text-anchor="middle" x="168.5" y="-549.3" font-family="Times,serif" font-size="14.00">cat myfile</text></g><!-- awk --><g id="node2" class="node"><title>awk</title><polygon fill="none" stroke="black" points="274.5,-498 62.5,-498 62.5,-462 274.5,-462 274.5,-498"/><text text-anchor="middle" x="168.5" y="-476.3" font-family="Times,serif" font-size="14.00">awk 'PAT { do something }'</text></g><!-- cat&#45;&gt;awk --><g id="edge1" class="edge"><title>cat->awk</title><path fill="none" stroke="black" d="M168.5,-534.81C168.5,-526.79 168.5,-517.05 168.5,-508.07"/><polygon fill="black" stroke="black" points="172,-508.03 168.5,-498.03 165,-508.03 172,-508.03"/></g><!-- grep1 --><g id="node3" class="node"><title>grep1</title><polygon fill="none" stroke="black" points="140.5,-425 40.5,-425 40.5,-389 140.5,-389 140.5,-425"/><text text-anchor="middle" x="90.5" y="-403.3" font-family="Times,serif" font-size="14.00">grep '^bad'</text></g><!-- awk&#45;&gt;grep1 --><g id="edge2" class="edge"><title>awk->grep1</title><path fill="none" stroke="black" d="M149.62,-461.81C139.75,-452.83 127.52,-441.69 116.74,-431.89"/><polygon fill="black" stroke="black" points="118.96,-429.17 109.21,-425.03 114.25,-434.35 118.96,-429.17"/></g><!-- grep2 --><g id="node8" class="node"><title>grep2</title><polygon fill="none" stroke="black" points="222.5,-371 114.5,-371 114.5,-335 222.5,-335 222.5,-371"/><text text-anchor="middle" x="168.5" y="-349.3" font-family="Times,serif" font-size="14.00">grep '^good'</text></g><!-- awk&#45;&gt;grep2 --><g id="edge7" class="edge"><title>awk->grep2</title><path fill="none" stroke="black" d="M168.5,-461.88C168.5,-441.15 168.5,-405.95 168.5,-381.42"/><polygon fill="black" stroke="black" points="172,-381.31 168.5,-371.31 165,-381.31 172,-381.31"/></g><!-- do --><g id="node4" class="node"><title>do</title><polygon fill="none" stroke="black" points="179,-317 0,-317 0,-281 179,-281 179,-317"/><text text-anchor="middle" x="89.5" y="-295.3" font-family="Times,serif" font-size="14.00">do 'further processing'</text></g><!-- grep1&#45;&gt;do --><g id="edge3" class="edge"><title>grep1->do</title><path fill="none" stroke="black" d="M90.34,-388.97C90.18,-372.38 89.94,-346.88 89.76,-327.43"/><polygon fill="black" stroke="black" points="93.26,-327.31 89.66,-317.34 86.26,-327.37 93.26,-327.31"/></g><!-- awk2 --><g id="node5" class="node"><title>awk2</title><polygon fill="none" stroke="black" points="305,-244 110,-244 110,-146 305,-146 305,-244"/><text text-anchor="start" x="118" y="-228.8" font-family="Times,serif" font-size="14.00">awk '</text><text text-anchor="start" x="118" y="-213.8" font-family="Times,serif" font-size="14.00">$1='good' {</text><text text-anchor="start" x="118" y="-198.8" font-family="Times,serif" font-size="14.00">    print /dev/stdout; next</text><text text-anchor="start" x="118" y="-183.8" font-family="Times,serif" font-size="14.00">}{</text><text text-anchor="start" x="118" y="-168.8" font-family="Times,serif" font-size="14.00">    print /dev/stderr</text><text text-anchor="start" x="118" y="-153.8" font-family="Times,serif" font-size="14.00">}'</text></g><!-- do&#45;&gt;awk2 --><g id="edge4" class="edge"><title>do->awk2</title><path fill="none" stroke="black" d="M109.25,-280.93C118.98,-272.52 131.31,-261.86 143.92,-250.96"/><polygon fill="black" stroke="black" points="146.51,-253.35 151.78,-244.16 141.93,-248.05 146.51,-253.35"/></g><!-- awk2&#45;&gt;awk --><g id="edge9" class="edge"><title>awk2->awk</title><path fill="none" stroke="black" d="M220.21,-244.14C227.58,-279.64 233.61,-328.87 223.5,-371 216.31,-400.94 199.37,-431.94 186.17,-453.01"/><polygon fill="black" stroke="black" points="183.07,-451.37 180.6,-461.67 188.96,-455.15 183.07,-451.37"/><text text-anchor="middle" x="258.5" y="-349.3" font-family="Times,serif" font-size="14.00">STDERR</text></g><!-- sed --><g id="node6" class="node"><title>sed</title><polygon fill="none" stroke="black" points="287.5,-109 127.5,-109 127.5,-73 287.5,-73 287.5,-109"/><text text-anchor="middle" x="207.5" y="-87.3" font-family="Times,serif" font-size="14.00">sed 's/good/perfect/'</text></g><!-- awk2&#45;&gt;sed --><g id="edge5" class="edge"><title>awk2->sed</title><path fill="none" stroke="black" d="M207.5,-145.99C207.5,-136.79 207.5,-127.46 207.5,-119.21"/><polygon fill="black" stroke="black" points="211,-119.07 207.5,-109.07 204,-119.07 211,-119.07"/></g><!-- sort --><g id="node7" class="node"><title>sort</title><polygon fill="none" stroke="black" points="234.5,-36 180.5,-36 180.5,0 234.5,0 234.5,-36"/><text text-anchor="middle" x="207.5" y="-14.3" font-family="Times,serif" font-size="14.00">sort</text></g><!-- sed&#45;&gt;sort --><g id="edge6" class="edge"><title>sed->sort</title><path fill="none" stroke="black" d="M207.5,-72.81C207.5,-64.79 207.5,-55.05 207.5,-46.07"/><polygon fill="black" stroke="black" points="211,-46.03 207.5,-36.03 204,-46.03 211,-46.03"/></g><!-- grep2&#45;&gt;awk2 --><g id="edge8" class="edge"><title>grep2->awk2</title><path fill="none" stroke="black" d="M179.36,-334.72C182.4,-329.23 185.44,-323.02 187.5,-317 194.36,-297.02 198.9,-274.34 201.9,-254.07"/><polygon fill="black" stroke="black" points="205.39,-254.41 203.29,-244.02 198.45,-253.45 205.39,-254.41"/></g></g></svg>

Some or all of this is possible to do with tools like `mkfifo`, depending on your skill level, but you certainly won't end up with something that is anywhere near as easy for someone to follow as the simple UNIX command we initially wrote out.

## An example project

I decided to make a tic-tac-toe game to demonstrate the capabilities of this project.

This game would have:

*   A data format which can be translated into **a human recognizable tic-tac-toe grid** with squares filled in.
*   Two **computer players**.
*   They would **take turns to pick a blank square** and fill it with their "X" or "O".
*   A **referee** to decide when the game had been won or was a draw.

### Data Format

I first decided on a data format which looks like the following:

STATUS **:** SQUARE\_1 **:** SQUARE\_2 **:** SQUARE\_3 **:** SQUARE\_4$ **:** SQUARE\_5 **:** SQUARE\_6 **:** SQUARE\_7 **:** SQUARE\_8 **:** SQUARE\_9

In this `STATUS` would be either `X`, `O` for player turns or *something else* to denote game draws / wins.

### Drawing the grid

I next coded up something which would show the grid. I coded a lot of this in GNU AWK because it's something that I'm learning off-and-on and (very) simple `STDIN | STDOUT` coding seems ideally suited to the language.

I came up with the following code:

```awk file=./examples/tic-tac-toe/draw.awk
function get_player_text(player_O_or_X) {
    extra = ""
    if (player_O_or_X == "D") {
        return "DRAW!";
    }
    if (player_O_or_X ~ "^W") {
        extra = "WON!"
        player_O_or_X = substr(player_O_or_X, 2)
    }
    return "Player " player_O_or_X " " extra
}
function draw_turn_line(pos_1, pos_2, pos_3) {
    return sprintf(" %-1s | %-1s | %-1s ", pos_1, pos_2, pos_3)
}
function draw_line_line() {
    return "---+---+---"
}
{
    print get_player_text($1) # " (" $0 ")"
    print ""
    print draw_turn_line($2, $3, $4)
    print draw_line_line()
    print draw_turn_line($5, $6, $7)
    print draw_line_line()
    print draw_turn_line($8, $9, $10)
    print ""
    fflush()
}
```

You can execute this code with `echo 'X:::X::O' | gawk -F ':' -f ./examples/tic-tac-toe/draw.awk` and it'll draw you the following grid:

```text
Player O 

   |   | X 
---+---+---
   | O |   
---+---+---
   |   | X 
```

I then wrote a Pipeawesome configuration file:

```yaml file=./examples/tic-tac-toe/draw.pa.yaml
connection:
  initial_word: "faucet:input | launch:draw | drain:output"
drain:
  output:
    destination: '-'
faucet:
  input:
    source: '-'
launch:
  draw:
    cmd: "awk"
    arg:
      - '-F'
      - ':'
      - '-f'
      - 'examples/tic-tac-toe/draw.awk'
```

You could execute this Pipeawesome configuration file with `echo 'O:::X::O::::X' | ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/draw.pa.yaml`

This is of course, a simple, even pointless example, but it allows me to explain the Pipeawesome file format without having you, the reader, have to consider too much complexity.

Lets break it down into it's constituent parts:

#### connection

```yaml
connection:
  initial_word: "faucet:input | launch:draw | drain:output"
```

In this there are pipes, which connect different types of components. The components types here are `faucet`, `launch` and `drain` with the names of those components being `input`, `draw` and `output`.

The names are just names, but they may need to be referenced elsewhere depending on the component type.

Component types can be:

*   **Faucet**: A source of input.
*   **Launch**: A running program.
*   **Drain**: A destination were final data will be sent to.
*   **Buffer**: A buffer can be used to regulate data flow
*   **Junction**: A many to many connector which can manage priorities of incoming data.

The pipes can be extended to configure how to handle broken pipes (when a recieving program exits before a sending program) and you can also control whether they're sending STDIN, STDOUT or EXIT statuses.

#### Component: Faucet

<svg width="366pt" height="106pt" viewBox="0.00 0.00 365.78 106.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 102)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-102 361.78,-102 361.78,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_faucet</title><path fill="none" stroke="lightgrey" d="M20,-8C20,-8 263.78,-8 263.78,-8 269.78,-8 275.78,-14 275.78,-20 275.78,-20 275.78,-78 275.78,-78 275.78,-84 269.78,-90 263.78,-90 263.78,-90 20,-90 20,-90 14,-90 8,-84 8,-78 8,-78 8,-20 8,-20 8,-14 14,-8 20,-8"/><text text-anchor="middle" x="51" y="-70" font-family="Times,serif" font-size="20.00">Faucet</text></g><!-- faucet_pull --><g id="node1" class="node"><title>faucet_pull</title><ellipse fill="none" stroke="black" cx="90.74" cy="-34" rx="74.99" ry="18"/><text text-anchor="middle" x="90.74" y="-30.3" font-family="Times,serif" font-size="14.00">pull: file/stdin</text></g><!-- faucet_push --><g id="node2" class="node"><title>faucet_push</title><ellipse fill="none" stroke="black" cx="234.63" cy="-34" rx="33.29" ry="18"/><text text-anchor="middle" x="234.63" y="-30.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- faucet_pull&#45;&gt;faucet_push --><g id="edge1" class="edge"><title>faucet_pull->faucet_push</title><path fill="none" stroke="black" d="M165.6,-34C174.31,-34 182.96,-34 191.02,-34"/><polygon fill="black" stroke="black" points="191.24,-37.5 201.24,-34 191.24,-30.5 191.24,-37.5"/></g><!-- fauct_exit_pull --><g id="node3" class="node"><title>fauct_exit_pull</title></g><!-- faucet_push&#45;&gt;fauct_exit_pull --><g id="edge2" class="edge"><title>faucet_push->fauct_exit_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M268.03,-34C276.19,-34 284.99,-34 293.33,-34"/><polygon fill="black" stroke="black" points="293.55,-37.5 303.55,-34 293.55,-30.5 293.55,-37.5"/></g></g></svg>

```yaml
faucet:
  input:
    source: '-'
```

The Faucet configuration here is for the one named `input`. It has a property called `source` which can be `-` for STDIN or a filename which will be read from.

#### Component: Launch

<svg width="769pt" height="266pt" viewBox="0.00 0.00 769.16 266.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 262)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-262 765.16,-262 765.16,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_launch</title><path fill="none" stroke="lightgrey" d="M94,-8C94,-8 651.16,-8 651.16,-8 657.16,-8 663.16,-14 663.16,-20 663.16,-20 663.16,-238 663.16,-238 663.16,-244 657.16,-250 651.16,-250 651.16,-250 94,-250 94,-250 88,-250 82,-244 82,-238 82,-238 82,-20 82,-20 82,-14 88,-8 94,-8"/><text text-anchor="middle" x="128" y="-230" font-family="Times,serif" font-size="20.00">Launch</text></g><g id="clust3" class="cluster"><title>cluster_spawn_holder</title><polygon fill="none" stroke="white" points="269.49,-16 269.49,-212 568.87,-212 568.87,-16 269.49,-16"/></g><g id="clust4" class="cluster"><title>cluster_launch_spawn</title><polygon fill="grey" stroke="grey" points="277.49,-68 277.49,-204 560.87,-204 560.87,-68 277.49,-68"/><text text-anchor="middle" x="346.49" y="-184" font-family="Times,serif" font-size="20.00">child.spawn</text></g><g id="clust5" class="cluster"><title>cluster_launch_outputs</title><polygon fill="none" stroke="white" points="683.16,-14 683.16,-174 753.16,-174 753.16,-14 683.16,-14"/></g><!-- launch_pull --><g id="node1" class="node"><title>launch_pull</title><ellipse fill="none" stroke="black" cx="118.6" cy="-122" rx="28.7" ry="18"/><text text-anchor="middle" x="118.6" y="-118.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- launch_stdin_recv_push --><g id="node2" class="node"><title>launch_stdin_recv_push</title><ellipse fill="none" stroke="black" cx="216.34" cy="-122" rx="33.29" ry="18"/><text text-anchor="middle" x="216.34" y="-118.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- launch_pull&#45;&gt;launch_stdin_recv_push --><g id="edge2" class="edge"><title>launch_pull->launch_stdin_recv_push</title><path fill="none" stroke="black" d="M147.29,-122C155.31,-122 164.25,-122 172.95,-122"/><polygon fill="black" stroke="black" points="172.98,-125.5 182.98,-122 172.98,-118.5 172.98,-125.5"/></g><!-- launch_stdin_recv_pull --><g id="node3" class="node"><title>launch_stdin_recv_pull</title><ellipse fill="none" stroke="black" cx="340.08" cy="-122" rx="54.69" ry="18"/><text text-anchor="middle" x="340.08" y="-118.3" font-family="Times,serif" font-size="14.00">pull:stdin</text></g><!-- launch_stdin_recv_push&#45;&gt;launch_stdin_recv_pull --><g id="edge3" class="edge"><title>launch_stdin_recv_push->launch_stdin_recv_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M249.65,-122C257.63,-122 266.46,-122 275.38,-122"/><polygon fill="black" stroke="black" points="275.43,-125.5 285.43,-122 275.43,-118.5 275.43,-125.5"/></g><!-- launch_stdout_send_pull --><g id="node4" class="node"><title>launch_stdout_send_pull</title><ellipse fill="none" stroke="black" cx="491.77" cy="-94" rx="61.19" ry="18"/><text text-anchor="middle" x="491.77" y="-90.3" font-family="Times,serif" font-size="14.00">pull:stdout</text></g><!-- launch_stdin_recv_pull&#45;&gt;launch_stdout_send_pull --><g id="edge4" class="edge"><title>launch_stdin_recv_pull->launch_stdout_send_pull</title><path fill="none" stroke="black" stroke-dasharray="1,5" d="M388.04,-113.22C404.5,-110.14 423.06,-106.67 439.83,-103.53"/></g><!-- launch_stderr_send_pull --><g id="node5" class="node"><title>launch_stderr_send_pull</title><ellipse fill="none" stroke="black" cx="491.77" cy="-148" rx="61.19" ry="18"/><text text-anchor="middle" x="491.77" y="-144.3" font-family="Times,serif" font-size="14.00">pull:stdout</text></g><!-- launch_stdin_recv_pull&#45;&gt;launch_stderr_send_pull --><g id="edge5" class="edge"><title>launch_stdin_recv_pull->launch_stderr_send_pull</title><path fill="none" stroke="black" stroke-dasharray="1,5" d="M388.86,-130.3C404.77,-133.06 422.57,-136.15 438.79,-138.97"/></g><!-- launch_stdout_recv_push --><g id="node8" class="node"><title>launch_stdout_recv_push</title><ellipse fill="none" stroke="black" cx="622.02" cy="-94" rx="33.29" ry="18"/><text text-anchor="middle" x="622.02" y="-90.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- launch_stdout_send_pull&#45;&gt;launch_stdout_recv_push --><g id="edge6" class="edge"><title>launch_stdout_send_pull->launch_stdout_recv_push</title><path fill="none" stroke="black" d="M553.14,-94C561.63,-94 570.21,-94 578.28,-94"/><polygon fill="black" stroke="black" points="578.55,-97.5 588.55,-94 578.55,-90.5 578.55,-97.5"/></g><!-- launch_stderr_recv_push --><g id="node7" class="node"><title>launch_stderr_recv_push</title><ellipse fill="none" stroke="black" cx="622.02" cy="-148" rx="33.29" ry="18"/><text text-anchor="middle" x="622.02" y="-144.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- launch_stderr_send_pull&#45;&gt;launch_stderr_recv_push --><g id="edge7" class="edge"><title>launch_stderr_send_pull->launch_stderr_recv_push</title><path fill="none" stroke="black" d="M553.14,-148C561.63,-148 570.21,-148 578.28,-148"/><polygon fill="black" stroke="black" points="578.55,-151.5 588.55,-148 578.55,-144.5 578.55,-151.5"/></g><!-- launch_exit_send_push --><g id="node6" class="node"><title>launch_exit_send_push</title><ellipse fill="none" stroke="black" cx="340.08" cy="-42" rx="53.89" ry="18"/><text text-anchor="middle" x="340.08" y="-38.3" font-family="Times,serif" font-size="14.00">push:exit</text></g><!-- launch_exit_outer --><g id="node12" class="node"><title>launch_exit_outer</title></g><!-- launch_exit_send_push&#45;&gt;launch_exit_outer --><g id="edge10" class="edge"><title>launch_exit_send_push->launch_exit_outer</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M394.17,-41.72C471.81,-41.3 614.34,-40.55 681,-40.19"/><polygon fill="black" stroke="black" points="681.05,-43.69 691.03,-40.14 681.01,-36.69 681.05,-43.69"/></g><!-- launch_stderr_outer --><g id="node11" class="node"><title>launch_stderr_outer</title></g><!-- launch_stderr_recv_push&#45;&gt;launch_stderr_outer --><g id="edge9" class="edge"><title>launch_stderr_recv_push->launch_stderr_outer</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M655.42,-148C663.57,-148 672.37,-148 680.71,-148"/><polygon fill="black" stroke="black" points="680.94,-151.5 690.94,-148 680.94,-144.5 680.94,-151.5"/></g><!-- launch_stdout_outer --><g id="node10" class="node"><title>launch_stdout_outer</title></g><!-- launch_stdout_recv_push&#45;&gt;launch_stdout_outer --><g id="edge8" class="edge"><title>launch_stdout_recv_push->launch_stdout_outer</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M655.42,-94C663.57,-94 672.37,-94 680.71,-94"/><polygon fill="black" stroke="black" points="680.94,-97.5 690.94,-94 680.94,-90.5 680.94,-97.5"/></g><!-- launch_input --><g id="node9" class="node"><title>launch_input</title></g><!-- launch_input&#45;&gt;launch_pull --><g id="edge1" class="edge"><title>launch_input->launch_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.4,-122C62.32,-122 71.18,-122 79.7,-122"/><polygon fill="black" stroke="black" points="79.83,-125.5 89.83,-122 79.83,-118.5 79.83,-125.5"/></g></g></svg>

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

This controls how a program is executed. Further configuration enables the configuration of environmental variables (`env`) and path (`path`).

#### Component: Drain

<svg width="379pt" height="106pt" viewBox="0.00 0.00 378.78 106.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 102)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-102 374.78,-102 374.78,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_drain</title><path fill="none" stroke="lightgrey" d="M94,-8C94,-8 350.78,-8 350.78,-8 356.78,-8 362.78,-14 362.78,-20 362.78,-20 362.78,-78 362.78,-78 362.78,-84 356.78,-90 350.78,-90 350.78,-90 94,-90 94,-90 88,-90 82,-84 82,-78 82,-78 82,-20 82,-20 82,-14 88,-8 94,-8"/><text text-anchor="middle" x="119" y="-70" font-family="Times,serif" font-size="20.00">Drain</text></g><!-- drain_input --><g id="node1" class="node"><title>drain_input</title></g><!-- drain_pull --><g id="node2" class="node"><title>drain_pull</title><ellipse fill="none" stroke="black" cx="118.6" cy="-34" rx="28.7" ry="18"/><text text-anchor="middle" x="118.6" y="-30.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- drain_input&#45;&gt;drain_pull --><g id="edge1" class="edge"><title>drain_input->drain_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.4,-34C62.32,-34 71.18,-34 79.7,-34"/><polygon fill="black" stroke="black" points="79.83,-37.5 89.83,-34 79.83,-30.5 79.83,-37.5"/></g><!-- drain_push --><g id="node3" class="node"><title>drain_push</title><ellipse fill="none" stroke="black" cx="268.99" cy="-34" rx="85.59" ry="18"/><text text-anchor="middle" x="268.99" y="-30.3" font-family="Times,serif" font-size="14.00">push: file/stdout</text></g><!-- drain_pull&#45;&gt;drain_push --><g id="edge2" class="edge"><title>drain_pull->drain_push</title><path fill="none" stroke="black" d="M147.51,-34C155.09,-34 163.75,-34 172.87,-34"/><polygon fill="black" stroke="black" points="172.91,-37.5 182.91,-34 172.91,-30.5 172.91,-37.5"/></g></g></svg>

```yaml
drain:
  output:
    destination: '-'
```

Similar to Faucet, output can be sent to `-` for STDOUT, `_` for STDERR, anything else is taken as a filename where data will be wrote to.

## Having a Go

The following configuration includes all the code actual code, but the configuration adds nothing you haven't seen so far

```yaml file=./examples/tic-tac-toe/have_a_go.pa.yaml
connection:
  initial_word: "f:input | l:player | l:referee | l:draw | d:output"
drain:
  output: { destination: '-' }
faucet:
  input: { source: '-' }
launch:
  player:
    command: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=O', '-f', 'examples/tic-tac-toe/player.awk' ]
  referee:
    command: "gawk"
    arg: ['-F', ':', '-f', './examples/tic-tac-toe/referee.awk', 'NF=10', 'OFS=:']
  draw:
    command: "gawk"
    arg: [ '-F', ':', '-f', 'examples/tic-tac-toe/draw.awk' ]
```

It can be executed with `echo 'O:::X::O::::X' | ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/have_a_go.pa.yaml`

The output from this will be the same as previous but with an extra `O` on the grid:

```text
Player O 

   |   | X 
---+---+---
   | O |   
---+---+---
   | O | X 
```

## Picking a random player to start the game

#### Code for generating the random player

I figured out that `echo $((RANDOM % 2))::::::::: | sed "s/1/X/" | sed "s/0/O/"` is a single line BASH snippet for selecting a random first player. Putting this into the configuration file would give me:

```yaml
launch:
  random_player:
    command: "bash"
    arg:
      - '-c'
      - 'echo $((RANDOM % 2))::::::::: | sed "s/1/X/" | sed "s/0/O/"'
```

But this still means I have to let that player take a turn. This means that I'm going to have to explain how to use Pipeawesome branches.

There are basically two ways to do branching, one is to use a **Launch** STDIN and STDOUT to do the actual splitting and the other is to use a **Junction** with `grep` running on either branch. If you wish to join the branches back together you must use a **Junction**.

#### Component: Junction

(a brief interlude)

<svg width="347pt" height="160pt" viewBox="0.00 0.00 347.49 160.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 156)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-156 343.49,-156 343.49,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_junction</title><path fill="none" stroke="lightgrey" d="M94,-8C94,-8 245.49,-8 245.49,-8 251.49,-8 257.49,-14 257.49,-20 257.49,-20 257.49,-132 257.49,-132 257.49,-138 251.49,-144 245.49,-144 245.49,-144 94,-144 94,-144 88,-144 82,-138 82,-132 82,-132 82,-20 82,-20 82,-14 88,-8 94,-8"/><text text-anchor="middle" x="132.5" y="-124" font-family="Times,serif" font-size="20.00">Junction</text></g><!-- junction_push_1 --><g id="node1" class="node"><title>junction_push\_1</title><ellipse fill="none" stroke="black" cx="216.34" cy="-88" rx="33.29" ry="18"/><text text-anchor="middle" x="216.34" y="-84.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- junction_exit_pull_1 --><g id="node7" class="node"><title>junction_exit_pull\_1</title></g><!-- junction_push_1&#45;&gt;junction_exit_pull_1 --><g id="edge7" class="edge"><title>junction_push\_1->junction_exit_pull\_1</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M249.74,-88C257.9,-88 266.7,-88 275.04,-88"/><polygon fill="black" stroke="black" points="275.26,-91.5 285.26,-88 275.26,-84.5 275.26,-91.5"/></g><!-- junction_push_2 --><g id="node2" class="node"><title>junction_push\_2</title><ellipse fill="none" stroke="black" cx="216.34" cy="-34" rx="33.29" ry="18"/><text text-anchor="middle" x="216.34" y="-30.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- junction_exit_pull_2 --><g id="node8" class="node"><title>junction_exit_pull\_2</title></g><!-- junction_push_2&#45;&gt;junction_exit_pull_2 --><g id="edge8" class="edge"><title>junction_push\_2->junction_exit_pull\_2</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M249.74,-34C257.9,-34 266.7,-34 275.04,-34"/><polygon fill="black" stroke="black" points="275.26,-37.5 285.26,-34 275.26,-30.5 275.26,-37.5"/></g><!-- junction_pull_1 --><g id="node3" class="node"><title>junction_pull\_1</title><ellipse fill="none" stroke="black" cx="118.6" cy="-88" rx="28.7" ry="18"/><text text-anchor="middle" x="118.6" y="-84.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- junction_pull_1&#45;&gt;junction_push_1 --><g id="edge3" class="edge"><title>junction_pull\_1->junction_push\_1</title><path fill="none" stroke="black" d="M147.29,-88C155.31,-88 164.25,-88 172.95,-88"/><polygon fill="black" stroke="black" points="172.98,-91.5 182.98,-88 172.98,-84.5 172.98,-91.5"/></g><!-- junction_pull_1&#45;&gt;junction_push_2 --><g id="edge5" class="edge"><title>junction_pull\_1->junction_push\_2</title><path fill="none" stroke="black" d="M140.37,-76.28C153.01,-69.16 169.39,-59.92 183.62,-51.89"/><polygon fill="black" stroke="black" points="185.61,-54.79 192.6,-46.83 182.17,-48.69 185.61,-54.79"/></g><!-- junction_pull_2 --><g id="node4" class="node"><title>junction_pull\_2</title><ellipse fill="none" stroke="black" cx="118.6" cy="-34" rx="28.7" ry="18"/><text text-anchor="middle" x="118.6" y="-30.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- junction_pull_2&#45;&gt;junction_push_1 --><g id="edge4" class="edge"><title>junction_pull\_2->junction_push\_1</title><path fill="none" stroke="black" d="M140.37,-45.72C153.01,-52.84 169.39,-62.08 183.62,-70.11"/><polygon fill="black" stroke="black" points="182.17,-73.31 192.6,-75.17 185.61,-67.21 182.17,-73.31"/></g><!-- junction_pull_2&#45;&gt;junction_push_2 --><g id="edge6" class="edge"><title>junction_pull\_2->junction_push\_2</title><path fill="none" stroke="black" d="M147.29,-34C155.31,-34 164.25,-34 172.95,-34"/><polygon fill="black" stroke="black" points="172.98,-37.5 182.98,-34 172.98,-30.5 172.98,-37.5"/></g><!-- junction_input_outer_1 --><g id="node5" class="node"><title>junction_input_outer\_1</title></g><!-- junction_input_outer_1&#45;&gt;junction_pull_1 --><g id="edge1" class="edge"><title>junction_input_outer\_1->junction_pull\_1</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.4,-88C62.32,-88 71.18,-88 79.7,-88"/><polygon fill="black" stroke="black" points="79.83,-91.5 89.83,-88 79.83,-84.5 79.83,-91.5"/></g><!-- junction_input_outer_2 --><g id="node6" class="node"><title>junction_input_outer\_2</title></g><!-- junction_input_outer_2&#45;&gt;junction_pull_2 --><g id="edge2" class="edge"><title>junction_input_outer\_2->junction_pull\_2</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.4,-34C62.32,-34 71.18,-34 79.7,-34"/><polygon fill="black" stroke="black" points="79.83,-37.5 89.83,-34 79.83,-30.5 79.83,-37.5"/></g></g></svg>

A **Junction** is a prioritized many-to-many connector. Anything that comes into one of it's inputs will be sent to all of it's outputs. There's no configuration for **Junction**, but you can add a priority to it's inputs when you connect them, this is done by using `launch:something | [5]junction:many_to_many` where 5 is the priority, if it is not specified the priority is 0 (negative priorities are allowed).

NOTE: A single message between a windows or unix line ending (\r\n or \n) and includes the line ending itself at the end.

#### Implementation

Adding the Junctions is easy:

##### Diagram

<svg width="308pt" height="681pt" viewBox="0.00 0.00 308.00 681.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 677)"><title>g_get_graph</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-677 304,-677 304,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_nodes_random_selection</title><polygon fill="none" stroke="black" points="8,-518 8,-665 154,-665 154,-518 8,-518"/><text text-anchor="middle" x="81" y="-649.8" font-family="Times,serif" font-size="14.00">random_selection:</text></g><g id="clust3" class="cluster"><title>cluster_nodes_referee_join</title><polygon fill="none" stroke="black" points="89,-8 89,-227 223,-227 223,-8 89,-8"/><text text-anchor="middle" x="142" y="-211.8" font-family="Times,serif" font-size="14.00">referee_join:</text></g><g id="clust4" class="cluster"><title>cluster_nodes_player_x_branch</title><polygon fill="none" stroke="black" points="10,-299 10,-446 147,-446 147,-299 10,-299"/><text text-anchor="middle" x="78.5" y="-430.8" font-family="Times,serif" font-size="14.00">player_x_branch:</text></g><g id="clust5" class="cluster"><title>cluster_nodes_player_o_branch</title><polygon fill="none" stroke="black" points="155,-299 155,-446 292,-446 292,-299 155,-299"/><text text-anchor="middle" x="223.5" y="-430.8" font-family="Times,serif" font-size="14.00">player_o_branch:</text></g><!-- l_random_player --><g id="node1" class="node"><title>l_random_player</title><polygon fill="lightgrey" stroke="black" points="142.5,-562 19.5,-562 19.5,-526 142.5,-526 142.5,-562"/><text text-anchor="middle" x="81" y="-540.3" font-family="Times,serif" font-size="14.00">random_player</text></g><!-- j_turn --><g id="node9" class="node"><title>j_turn</title><ellipse fill="lightgrey" stroke="black" cx="81" cy="-472" rx="30.59" ry="18"/><text text-anchor="middle" x="81" y="-468.3" font-family="Times,serif" font-size="14.00">turn</text></g><!-- l_random_player&#45;&gt;j_turn --><g id="edge5" class="edge"><title>l_random_player->j_turn</title><path fill="none" stroke="black" d="M81,-525.7C81,-517.98 81,-508.71 81,-500.11"/><polygon fill="black" stroke="black" points="84.5,-500.1 81,-490.1 77.5,-500.1 84.5,-500.1"/></g><!-- f_input --><g id="node2" class="node"><title>f_input</title><polygon fill="lightgrey" stroke="black" points="110.07,-634 51.93,-634 31.23,-598 130.77,-598 110.07,-634"/><text text-anchor="middle" x="81" y="-612.3" font-family="Times,serif" font-size="14.00">input</text></g><!-- f_input&#45;&gt;l_random_player --><g id="edge4" class="edge"><title>f_input->l_random_player</title><path fill="none" stroke="black" d="M81,-597.7C81,-589.98 81,-580.71 81,-572.11"/><polygon fill="black" stroke="black" points="84.5,-572.1 81,-562.1 77.5,-572.1 84.5,-572.1"/></g><!-- l_referee --><g id="node3" class="node"><title>l_referee</title><polygon fill="lightgrey" stroke="black" points="190,-196 122,-196 122,-160 190,-160 190,-196"/><text text-anchor="middle" x="156" y="-174.3" font-family="Times,serif" font-size="14.00">referee</text></g><!-- l_draw --><g id="node4" class="node"><title>l_draw</title><polygon fill="lightgrey" stroke="black" points="183,-124 129,-124 129,-88 183,-88 183,-124"/><text text-anchor="middle" x="156" y="-102.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- l_referee&#45;&gt;l_draw --><g id="edge2" class="edge"><title>l_referee->l_draw</title><path fill="none" stroke="black" d="M156,-159.7C156,-151.98 156,-142.71 156,-134.11"/><polygon fill="black" stroke="black" points="159.5,-134.1 156,-124.1 152.5,-134.1 159.5,-134.1"/></g><!-- d_output --><g id="node5" class="node"><title>d_output</title><polygon fill="lightgrey" stroke="black" points="121.75,-16 190.25,-16 214.65,-52 97.35,-52 121.75,-16"/><text text-anchor="middle" x="156" y="-30.3" font-family="Times,serif" font-size="14.00">output</text></g><!-- l_draw&#45;&gt;d_output --><g id="edge3" class="edge"><title>l_draw->d_output</title><path fill="none" stroke="black" d="M156,-87.7C156,-79.98 156,-70.71 156,-62.11"/><polygon fill="black" stroke="black" points="159.5,-62.1 156,-52.1 152.5,-62.1 159.5,-62.1"/></g><!-- l_player_x_filter --><g id="node6" class="node"><title>l_player_x_filter</title><polygon fill="lightgrey" stroke="black" points="139,-415 21,-415 21,-379 139,-379 139,-415"/><text text-anchor="middle" x="80" y="-393.3" font-family="Times,serif" font-size="14.00">player_x_filter</text></g><!-- l_player_x --><g id="node7" class="node"><title>l_player_x</title><polygon fill="lightgrey" stroke="black" points="128.5,-343 51.5,-343 51.5,-307 128.5,-307 128.5,-343"/><text text-anchor="middle" x="90" y="-321.3" font-family="Times,serif" font-size="14.00">player_x</text></g><!-- l_player_x_filter&#45;&gt;l_player_x --><g id="edge10" class="edge"><title>l_player_x_filter->l_player_x</title><path fill="none" stroke="black" d="M82.47,-378.7C83.57,-370.98 84.9,-361.71 86.13,-353.11"/><polygon fill="black" stroke="black" points="89.61,-353.5 87.56,-343.1 82.68,-352.51 89.61,-353.5"/></g><!-- j_referee --><g id="node8" class="node"><title>j_referee</title><ellipse fill="lightgrey" stroke="black" cx="156" cy="-253" rx="44.39" ry="18"/><text text-anchor="middle" x="156" y="-249.3" font-family="Times,serif" font-size="14.00">referee</text></g><!-- l_player_x&#45;&gt;j_referee --><g id="edge11" class="edge"><title>l_player_x->j_referee</title><path fill="none" stroke="black" d="M106.31,-306.7C114.63,-297.88 124.86,-287.03 133.87,-277.47"/><polygon fill="black" stroke="black" points="136.46,-279.82 140.78,-270.14 131.37,-275.02 136.46,-279.82"/></g><!-- j_referee&#45;&gt;l_referee --><g id="edge1" class="edge"><title>j_referee->l_referee</title><path fill="none" stroke="black" d="M156,-234.7C156,-226.25 156,-215.87 156,-206.37"/><polygon fill="black" stroke="black" points="159.5,-206.18 156,-196.18 152.5,-206.18 159.5,-206.18"/></g><!-- j_turn&#45;&gt;l_player_x_filter --><g id="edge9" class="edge"><title>j_turn->l_player_x_filter</title><path fill="none" stroke="black" d="M80.76,-453.7C80.65,-445.25 80.51,-434.87 80.37,-425.37"/><polygon fill="black" stroke="black" points="83.87,-425.13 80.24,-415.18 76.87,-425.22 83.87,-425.13"/></g><!-- l_player_o_filter --><g id="node10" class="node"><title>l_player_o_filter</title><polygon fill="lightgrey" stroke="black" points="281,-415 163,-415 163,-379 281,-379 281,-415"/><text text-anchor="middle" x="222" y="-393.3" font-family="Times,serif" font-size="14.00">player_o_filter</text></g><!-- j_turn&#45;&gt;l_player_o_filter --><g id="edge6" class="edge"><title>j_turn->l_player_o_filter</title><path fill="none" stroke="black" d="M107.91,-463.14C121.21,-458.83 137.3,-452.94 151,-446 164.89,-438.97 179.28,-429.67 191.45,-421.13"/><polygon fill="black" stroke="black" points="193.78,-423.76 199.87,-415.09 189.71,-418.07 193.78,-423.76"/></g><!-- l_player_o --><g id="node11" class="node"><title>l_player_o</title><polygon fill="lightgrey" stroke="black" points="250.5,-343 173.5,-343 173.5,-307 250.5,-307 250.5,-343"/><text text-anchor="middle" x="212" y="-321.3" font-family="Times,serif" font-size="14.00">player_o</text></g><!-- l_player_o_filter&#45;&gt;l_player_o --><g id="edge7" class="edge"><title>l_player_o_filter->l_player_o</title><path fill="none" stroke="black" d="M219.53,-378.7C218.43,-370.98 217.1,-361.71 215.87,-353.11"/><polygon fill="black" stroke="black" points="219.32,-352.51 214.44,-343.1 212.39,-353.5 219.32,-352.51"/></g><!-- l_player_o&#45;&gt;j_referee --><g id="edge8" class="edge"><title>l_player_o->j_referee</title><path fill="none" stroke="black" d="M198.16,-306.7C191.34,-298.18 183.01,-287.76 175.56,-278.45"/><polygon fill="black" stroke="black" points="178.28,-276.24 169.3,-270.62 172.81,-280.62 178.28,-276.24"/></g></g></svg>

##### Legend

<svg width="368pt" height="85pt" viewBox="0.00 0.00 368.00 85.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 81)"><title>g_get_graph</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-81 364,-81 364,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_legend_launch</title><polygon fill="none" stroke="black" points="8,-8 8,-69 72,-69 72,-8 8,-8"/><text text-anchor="middle" x="40" y="-53.8" font-family="Times,serif" font-size="14.00">launch</text></g><g id="clust3" class="cluster"><title>cluster_legend_buffer</title><polygon fill="none" stroke="black" points="80,-8 80,-69 139,-69 139,-8 80,-8"/><text text-anchor="middle" x="109.5" y="-53.8" font-family="Times,serif" font-size="14.00">buffer</text></g><g id="clust4" class="cluster"><title>cluster_legend_junction</title><polygon fill="none" stroke="black" points="147,-8 147,-69 221,-69 221,-8 147,-8"/><text text-anchor="middle" x="184" y="-53.8" font-family="Times,serif" font-size="14.00">junction</text></g><g id="clust5" class="cluster"><title>cluster_legend_faucet</title><polygon fill="none" stroke="black" points="229,-8 229,-69 290,-69 290,-8 229,-8"/><text text-anchor="middle" x="259.5" y="-53.8" font-family="Times,serif" font-size="14.00">faucet</text></g><g id="clust6" class="cluster"><title>cluster_legend_drain</title><polygon fill="none" stroke="black" points="298,-8 298,-69 352,-69 352,-8 298,-8"/><text text-anchor="middle" x="325" y="-53.8" font-family="Times,serif" font-size="14.00">drain</text></g><!-- legend_launch --><g id="node1" class="node"><title>legend_launch</title><polygon fill="lightgrey" stroke="black" points="51,-38 29,-38 29,-16 51,-16 51,-38"/></g><!-- legend_buffer --><g id="node2" class="node"><title>legend_buffer</title><polygon fill="lightgrey" stroke="black" points="98,-23.6 109,-16 120,-23.6 119.99,-35.9 98.01,-35.9 98,-23.6"/></g><!-- legend_junction --><g id="node3" class="node"><title>legend_junction</title><ellipse fill="lightgrey" stroke="black" cx="184" cy="-27" rx="11" ry="11"/></g><!-- legend_faucet --><g id="node4" class="node"><title>legend_faucet</title><polygon fill="lightgrey" stroke="black" points="265.42,-38 252.58,-38 248,-16 270,-16 265.42,-38"/></g><!-- legend_drain --><g id="node5" class="node"><title>legend_drain</title><polygon fill="lightgrey" stroke="black" points="318.58,-16 331.42,-16 336,-38 314,-38 318.58,-16"/></g></g></svg>

In doing this diagram, I cheated, I wrote the configuration file first and then told Pipeawsome to draw the graph (`./target/debug/pipeawesome2 graph --config examples/tic-tac-toe/random_player.pa.yaml`). But writing the configuration was relatively simple, see below:

```yaml file=./examples/tic-tac-toe/random_player.pa.yaml
connection:
  random_selection: "f:input | l:random_player | junction:turn"
  player_o_branch: "junction:turn | l:player_o_filter | l:player_o | junction:referee"
  player_x_branch: "junction:turn | l:player_x_filter | l:player_x | junction:referee"
  referee_join: "junction:referee | l:referee | l:draw | d:output"
drain:
  output: { destination: '-' }
faucet:
  input: { source: '/dev/null' }
launch:
  random_player:
    command: "bash"
    arg:
      - '-c'
      - 'echo $((RANDOM % 2))::::::::: | sed "s/1/X/" | sed "s/0/O/"'
  player_o_filter: { command: "grep", arg: [ "--line-buffered", "^O" ] }
  player_o:
    command: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=O', '-f', 'examples/tic-tac-toe/player.awk' ]
  player_x_filter: { command: "grep", arg: [ "--line-buffered", "^X" ] }
  player_x:
    command: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=X', '-f', 'examples/tic-tac-toe/player.awk' ]
  referee:
    command: "gawk"
    arg: ['-F', ':', '-f', './examples/tic-tac-toe/referee.awk', 'NF=10', 'OFS=:']
  draw:
    command: "gawk"
    arg: [ '-F', ':', '-f', 'examples/tic-tac-toe/draw.awk' ]
```

The changes are:

```yaml
faucet:
  input: { source: '/dev/null' }
```

The Faucet configuration has changed to read input from `/dev/null`. This causes the input close immediately but will feed that empty line into the `random_player` launch.

I did of course add the `random_player` launch as well as `player_o_filter`, `player_x_filter`, `referee` and changed `player` into `player_o` and `player_x`, but the format of Launch should be unsurprising by now.

The big change is that there are now multiple keys / lines within `connection:`. If you look the `random_selection:` connection set writes to `junction:turn` but `junction:turn` is read in both `player_o_branch` and `player_x_branch`, which in turn both write to `junction:referee`. The connection set names are completely arbitrary, though the must be unique, but this is how branching is achieved

## A complete game

Imagine you own a restaurant and you want to check the temperature of soup as it leaves the kitchen:

# cargo build && cat bats-tests/branches/input.txt | ./target/debug/pipeawesome2 graph --config bats-tests/loops/pa.yaml > out.dot &&  cat out.dot | dot -T png >out.png && xdg-open out.png

```dot
digraph {
    rankdir = LR;
    
}
```

To run this file you do the following:

```bash
$  echo -e "44\n90\n92\n99\n33" | pipeawesome \
        --inline --pipeline "$(cat examples/temperature_prope.paspec.json5 | json5)" \
        --input KITCHEN=- --output RESTAURANT=-
TOO_COLD:44
JUST_RIGHT:90
JUST_RIGHT:92
TOO_HOT:99
TOO_COLD:33
```

NOTE 1: `-o RESTAURANT=-` means the output "RESTAURANT" should go to STDOUT.
You can also use an underscore (`_`) to mean STDERR. If it is neither of
these it will be interpreted as a filename which will be wrote to.

NOTE 2: Without `--inline` the `--pipeline` parameter is a filename which must be valid JSON. I wanted to use JSON5 here so I could add meaningful comments, hence the `--inline --pipeline $(cat … | json5)` combination.

## An example which shows how real value could be realized.

Of course, the `TEMPERATURE_CHECKER` probably should send soup that is either too hot, or too cold back to the kitchen where it will either be left to cool or heated up some more:

```dot
digraph {
    0 [ label = "PREPERATION" ]
    2 [ label = "INGREDIENTS" ]
    4 [ label = "TEMPERATURE_CHECKER_MATHS" ]
    7 [ label = "LEAVING_TO_COOL" ]
    9 [ label = "ADDING_MORE_HEAT" ]
    11 [ label = "TEMPERATURE_CHECKER_QA" ]
    14 [ label = "JUST_RIGHT" ]
    17 [ label = "TOO_HOT_FILTER" ]
    19 [ label = "TOO_COLD_FILTER" ]
    25 [ label = "RESTAURANT" ]
    0 -> 4 [ label = "" ]
    7 -> 4 [ label = "" ]
    4 -> 11 [ label = "" ]
    17 -> 7 [ label = "" ]
    11 -> 19 [ label = "" ]
    9 -> 4 [ label = "" ]
    14 -> 25 [ label = "" ]
    19 -> 9 [ label = "" ]
    11 -> 14 [ label = "" ]
    11 -> 17 [ label = "" ]
    2 -> 0 [ label = "" ]
}
```

```bash
$ ./target/debug/pipeawesome -p "$(cat examples/soup_back_to_kitchen.paspec.json5 | json5)" \
        -i INGREDIENTS=tests/pipeawesome/soup_temperature.input.txt \
        -o RESTAURANT=-
4: JUST_RIGHT: 4 + 51 + 2 + 33: 90
3: JUST_RIGHT: 54 + 9 + 26: 89
2: JUST_RIGHT: 56 + 22 + 66 - 6 - 4 - 3 - 7 - 1 - 5 - 3 - 7 - 7 - 4 - 2 - 3: 92
5: JUST_RIGHT: 1 + 1 + 1 + 26 + 29 + 29 + 18 - 4 - 2 - 7: 92
1: JUST_RIGHT: 12 + 5 + 25 + 8 + 16 + 5 + 28 - 5 - 6: 88
^C
```

NOTE: We explain why we had to CTRL-C later on.

### But why did we have to CTRL-C when all the data had been processed?

Pipeawesome will exit when all outputs have been closed (or programs which have no outgoing connections have exited).

In Pipeawesome, when a programs exits, it's output is closed, so we close STDIN for the next program, which could (when it has finished processing) then exit and its output is closed. When each program actually finishes processing and flushes its output is at some indeterminate point in the future and not necessarily related to it's input.

The "problem" occurs when there is a loop. In the example above "PREPERATION" has closed it's own STDOUT but "TEMPERATURE_CHECKER_MATHS" has two more inputs. If you follow it back for one branch:

*   To close "LEAVING_TO_COOL", "TOO_HOT_FILTER" would need to close.
*   To close "TOO_HOT_FILTER", "TEMPERATURE_CHECKER_QA" would need to close.
*   To close "TEMPERATURE_CHECKER_QA", it's own input needs to be closed, which is "TEMPERATURE_CHECKER_MATHS"... which is not going to happen.

This may not be a problem for some software (like server or desktop software) which you may want to keep open. But for tools on the command line it doesn't work very well.

In the above example "PREPERATION" added line numbers to the very beginning of a line. If we want Pipeawesome to exit all we need to do is add a program between "JUST_RIGHT" and "RESTAURANT" which compares those numbers from "PREPERATION" with the ones flowing through itself and closes when it has seen them all.

There are trade-offs writing this program, as we don't know how many numbers there will be but there is a trivial implementation in `pipeawesome-terminator` as well as an example usage which adds just 3 commands in [./examples/perfect_soup.paspec.json](./examples/perfect_soup.paspec.json).

## What are you hoping to achieve?

### My original reason for building this

I have recently created two projects:

*   [eSQLate](https://github.com/forbesmyester/esqlate) - A relatively successful web front end for parameterized SQL queries.
*   [WV Linewise](https://github.com/forbesmyester/wv-linewise) - A pretty much ignored way of putting a [web-view](https://github.com/Boscop/web-view) in the middle of a UNIX pipeline (I think this is awesome but the world does not yet agree).

My plan is to join these two pieces of software together so eSQLate no longer has to run as a web service, but more like ordinary locally installed software.

### A more exciting example

A more graphical and / or exciting way to describe my idea is thinking of a simple turn based strategy game, where:

*   There are two players - each looking at a web-view/GUI window controlled by a separate UNIX processes.
*   There is one "server" which itself is a UNIX process, but without an associated window.

The programs are connected by UNIX pipes as shown below:

```dot
digraph {
    rankdir = LR;
    player_1 [label="player 1" color="red"]
    player_2 [label="player 2" color="blue"]
    player_1 -> engine -> player_2 -> engine -> player_1
}
```

When "player 1" moves, the "player 1" software sends that action out over STDOUT which is received by "engine". The "engine" will then send the new game world to both "player 1" and "player 2"

Of course, adding more players is trivial and making it a network based game could be achieved via `netcat`, `ssh` or integration with a proper queueing solution.

## A philosophical question... Are UNIX pipelines microservices?

There has been a big push towards microservices and these are often wired together using Queues. This got me thinking:

1.  Are UNIX pipes actually Queues?
2.  Can we view individual programs as microservices?

For me, while there are caveats, the answers to these questions is YES. I also believe that it would be cheaper, more reliable and faster to build (some) software in this way.

## A more detailed description of the configuration file

The configuration file forms a directed graph.  In the end I designed a JSON (groan) file format as it is somewhat easy to parse for both humans and computers.

For simple, and even at it's most complicated, the configuration looks like the following:

In this file format:

*   Outputs are listed in the `outputs` property of the JSON file.
*   Inputs are simply found by finding the `src`'s of the `commands` which are themselves not commands. In the example above "KITCHEN" does not exist as a command so it has become an input. It is perfectly permissible to have no inputs and start with commands that themselves produce data.

Because file format forms a directed graph, the following is possible:

*   If you want to do a branch, you just list multiple commands coming from the same "src".
*   If you want to do a join you have one command with multiple "src".
*   Loops are achieved by a branch and a join back onto itself.

The only other thing to note is that commands have three outputs "OUT" "ERR" and "EXIT". Which are STDOUT, STDERR and the exit status of a command.

## Building

This is a [Rust](https://www.rust-lang.org/) project. I used [RustUp](https://rustup.rs/) to install Rust (1.45) which is very easy to nowadays.

I have developed and tested this on Ubuntu 20.04 and have done rudimentary testing on Windows. I have not tested it on MacOS yet but it compiled and ran first time on Windows so it's probably going to be OK... If not PR's are always welcome 😃.

## Generating Graphs

    TMP_DIR=$(mktemp -d)
    ./target/debug/pipeawesome2 graph --legend-only  --config bats-tests/word_ladders_basic/pa.yaml | dot -Tpng > "${TMP_DIR}/legend.png"
    ./target/debug/pipeawesome2 graph --diagram-only --config bats-tests/word_ladders_basic/pa.yaml | dot -Tpng > "${TMP_DIR}/diagram.png"
    convert "${TMP_DIR}/diagram.png" "${TMP_DIR}/legend.png" -gravity east -append full_diagram.png
    xdg-open full_diagram.png
    rm -rf "${TMP_DIR}"

<svg width="343pt" height="261pt" viewBox="0.00 0.00 343.00 261.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 257)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-257 339,-257 339,4 -4,4"/><g id="clust1" class="cluster"><title>cluster_legend</title><polygon fill="none" stroke="black" points="8,-8 8,-245 327,-245 327,-8 8,-8"/><text text-anchor="middle" x="167.5" y="-229.8" font-family="Times,serif" font-size="14.00">Legend</text></g><!-- a --><g id="node1" class="node"><title>a</title><ellipse fill="none" stroke="black" cx="43" cy="-196" rx="27" ry="18"/></g><!-- b --><g id="node2" class="node"><title>b</title><ellipse fill="none" stroke="black" cx="292" cy="-196" rx="27" ry="18"/></g><!-- a&#45;&gt;b --><g id="edge1" class="edge"><title>a->b</title><path fill="none" stroke="black" d="M70.43,-196C114.82,-196 204.38,-196 254.66,-196"/><polygon fill="black" stroke="black" points="254.76,-199.5 264.76,-196 254.76,-192.5 254.76,-199.5"/><text text-anchor="middle" x="167.5" y="-199.8" font-family="Times,serif" font-size="14.00">motion</text></g><!-- c --><g id="node3" class="node"><title>c</title><ellipse fill="none" stroke="black" cx="43" cy="-142" rx="27" ry="18"/></g><!-- d --><g id="node4" class="node"><title>d</title><ellipse fill="none" stroke="black" cx="292" cy="-142" rx="27" ry="18"/></g><!-- c&#45;&gt;d --><g id="edge2" class="edge"><title>c->d</title><path fill="none" stroke="black" stroke-dasharray="1,5" d="M70.43,-142C117.97,-142 217.35,-142 264.76,-142"/><text text-anchor="middle" x="167.5" y="-145.8" font-family="Times,serif" font-size="14.00">io</text></g><!-- e --><g id="node5" class="node"><title>e</title><ellipse fill="none" stroke="black" cx="43" cy="-88" rx="27" ry="18"/></g><!-- f --><g id="node6" class="node"><title>f</title><ellipse fill="none" stroke="black" cx="292" cy="-88" rx="27" ry="18"/></g><!-- e&#45;&gt;f --><g id="edge3" class="edge"><title>e->f</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M70.43,-88C114.82,-88 204.38,-88 254.66,-88"/><polygon fill="black" stroke="black" points="254.76,-91.5 264.76,-88 254.76,-84.5 254.76,-91.5"/><text text-anchor="middle" x="167.5" y="-91.8" font-family="Times,serif" font-size="14.00">channel synchronous</text></g><!-- g --><g id="node7" class="node"><title>g</title><ellipse fill="none" stroke="black" cx="43" cy="-34" rx="27" ry="18"/></g><!-- h --><g id="node8" class="node"><title>h</title><ellipse fill="none" stroke="black" cx="292" cy="-34" rx="27" ry="18"/></g><!-- g&#45;&gt;h --><g id="edge4" class="edge"><title>g->h</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M70.43,-34C114.82,-34 204.38,-34 254.66,-34"/><polygon fill="none" stroke="black" points="254.76,-37.5 264.76,-34 254.76,-30.5 254.76,-37.5"/><text text-anchor="middle" x="167.5" y="-37.8" font-family="Times,serif" font-size="14.00">channel asynchronous</text></g></g></svg>

Generate this documentation with: `matts-markdown -m README.src.md > README.md && matts-markdown README.src.md > README.html && refresh-browser firefox`

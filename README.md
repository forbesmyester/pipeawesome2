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

<svg width="1003pt" height="106pt" viewBox="0.00 0.00 1003.00 106.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 102)"><title>%3</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-102 999,-102 999,4 -4,4"/><!-- cat --><g id="node1" class="node"><title>cat</title><polygon fill="none" stroke="black" points="86,-67 0,-67 0,-31 86,-31 86,-67"/><text text-anchor="middle" x="43" y="-45.3" font-family="Times,serif" font-size="14.00">cat myfile</text></g><!-- awk --><g id="node2" class="node"><title>awk</title><polygon fill="none" stroke="black" points="334,-67 122,-67 122,-31 334,-31 334,-67"/><text text-anchor="middle" x="228" y="-45.3" font-family="Times,serif" font-size="14.00">awk 'PAT { do something }'</text></g><!-- cat&#45;&gt;awk --><g id="edge1" class="edge"><title>cat->awk</title><path fill="none" stroke="black" d="M86.44,-49C94.27,-49 102.77,-49 111.6,-49"/><polygon fill="black" stroke="black" points="111.67,-52.5 121.67,-49 111.67,-45.5 111.67,-52.5"/></g><!-- grep2 --><g id="node3" class="node"><title>grep2</title><polygon fill="none" stroke="black" points="478,-67 370,-67 370,-31 478,-31 478,-67"/><text text-anchor="middle" x="424" y="-45.3" font-family="Times,serif" font-size="14.00">grep '^good'</text></g><!-- awk&#45;&gt;grep2 --><g id="edge2" class="edge"><title>awk->grep2</title><path fill="none" stroke="black" d="M334.23,-49C342.88,-49 351.43,-49 359.62,-49"/><polygon fill="black" stroke="black" points="359.78,-52.5 369.78,-49 359.78,-45.5 359.78,-52.5"/></g><!-- awk2 --><g id="node4" class="node"><title>awk2</title><polygon fill="none" stroke="black" points="709,-98 514,-98 514,0 709,0 709,-98"/><text text-anchor="start" x="522" y="-82.8" font-family="Times,serif" font-size="14.00">awk '</text><text text-anchor="start" x="522" y="-67.8" font-family="Times,serif" font-size="14.00">$1='good' {</text><text text-anchor="start" x="522" y="-52.8" font-family="Times,serif" font-size="14.00">    print /dev/stdout; next</text><text text-anchor="start" x="522" y="-37.8" font-family="Times,serif" font-size="14.00">}{</text><text text-anchor="start" x="522" y="-22.8" font-family="Times,serif" font-size="14.00">    print /dev/stderr</text><text text-anchor="start" x="522" y="-7.8" font-family="Times,serif" font-size="14.00">}'</text></g><!-- grep2&#45;&gt;awk2 --><g id="edge3" class="edge"><title>grep2->awk2</title><path fill="none" stroke="black" d="M478.16,-49C486.32,-49 495,-49 503.84,-49"/><polygon fill="black" stroke="black" points="503.88,-52.5 513.88,-49 503.88,-45.5 503.88,-52.5"/></g><!-- sed --><g id="node5" class="node"><title>sed</title><polygon fill="none" stroke="black" points="905,-67 745,-67 745,-31 905,-31 905,-67"/><text text-anchor="middle" x="825" y="-45.3" font-family="Times,serif" font-size="14.00">sed 's/good/perfect/'</text></g><!-- awk2&#45;&gt;sed --><g id="edge4" class="edge"><title>awk2->sed</title><path fill="none" stroke="black" d="M709.27,-49C717.7,-49 726.21,-49 734.58,-49"/><polygon fill="black" stroke="black" points="734.7,-52.5 744.7,-49 734.7,-45.5 734.7,-52.5"/></g><!-- sort --><g id="node6" class="node"><title>sort</title><polygon fill="none" stroke="black" points="995,-67 941,-67 941,-31 995,-31 995,-67"/><text text-anchor="middle" x="968" y="-45.3" font-family="Times,serif" font-size="14.00">sort</text></g><!-- sed&#45;&gt;sort --><g id="edge5" class="edge"><title>sed->sort</title><path fill="none" stroke="black" d="M905.16,-49C914.15,-49 922.95,-49 930.96,-49"/><polygon fill="black" stroke="black" points="930.99,-52.5 940.99,-49 930.99,-45.5 930.99,-52.5"/></g></g></svg>

However it might be that you want to:

<svg width="1323pt" height="121pt" viewBox="0.00 0.00 1323.00 121.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 117)"><title>%3</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-117 1319,-117 1319,4 -4,4"/><!-- cat --><g id="node1" class="node"><title>cat</title><polygon fill="none" stroke="black" points="86,-67 0,-67 0,-31 86,-31 86,-67"/><text text-anchor="middle" x="43" y="-45.3" font-family="Times,serif" font-size="14.00">cat myfile</text></g><!-- awk --><g id="node2" class="node"><title>awk</title><polygon fill="none" stroke="black" points="335,-67 123,-67 123,-31 335,-31 335,-67"/><text text-anchor="middle" x="229" y="-45.3" font-family="Times,serif" font-size="14.00">awk 'PAT { do something }'</text></g><!-- cat&#45;&gt;awk --><g id="edge1" class="edge"><title>cat->awk</title><path fill="none" stroke="black" d="M86.21,-49C94.39,-49 103.33,-49 112.6,-49"/><polygon fill="black" stroke="black" points="112.78,-52.5 122.78,-49 112.78,-45.5 112.78,-52.5"/></g><!-- grep1 --><g id="node3" class="node"><title>grep1</title><polygon fill="none" stroke="black" points="472,-113 372,-113 372,-77 472,-77 472,-113"/><text text-anchor="middle" x="422" y="-91.3" font-family="Times,serif" font-size="14.00">grep '^bad'</text></g><!-- awk&#45;&gt;grep1 --><g id="edge2" class="edge"><title>awk->grep1</title><path fill="none" stroke="black" d="M304.94,-67.05C323.9,-71.61 343.99,-76.45 362.04,-80.8"/><polygon fill="black" stroke="black" points="361.29,-84.22 371.83,-83.16 362.93,-77.41 361.29,-84.22"/></g><!-- grep2 --><g id="node8" class="node"><title>grep2</title><polygon fill="none" stroke="black" points="598,-67 490,-67 490,-31 598,-31 598,-67"/><text text-anchor="middle" x="544" y="-45.3" font-family="Times,serif" font-size="14.00">grep '^good'</text></g><!-- awk&#45;&gt;grep2 --><g id="edge7" class="edge"><title>awk->grep2</title><path fill="none" stroke="black" d="M335.1,-49C383.07,-49 438.29,-49 479.75,-49"/><polygon fill="black" stroke="black" points="479.99,-52.5 489.99,-49 479.99,-45.5 479.99,-52.5"/></g><!-- do --><g id="node4" class="node"><title>do</title><polygon fill="none" stroke="black" points="795,-113 616,-113 616,-77 795,-77 795,-113"/><text text-anchor="middle" x="705.5" y="-91.3" font-family="Times,serif" font-size="14.00">do 'further processing'</text></g><!-- grep1&#45;&gt;do --><g id="edge3" class="edge"><title>grep1->do</title><path fill="none" stroke="black" d="M472.03,-95C508.86,-95 560.54,-95 605.81,-95"/><polygon fill="black" stroke="black" points="605.97,-98.5 615.97,-95 605.97,-91.5 605.97,-98.5"/></g><!-- awk2 --><g id="node5" class="node"><title>awk2</title><polygon fill="none" stroke="black" points="1027,-98 832,-98 832,0 1027,0 1027,-98"/><text text-anchor="start" x="840" y="-82.8" font-family="Times,serif" font-size="14.00">awk '</text><text text-anchor="start" x="840" y="-67.8" font-family="Times,serif" font-size="14.00">$1='good' {</text><text text-anchor="start" x="840" y="-52.8" font-family="Times,serif" font-size="14.00">    print /dev/stdout; next</text><text text-anchor="start" x="840" y="-37.8" font-family="Times,serif" font-size="14.00">}{</text><text text-anchor="start" x="840" y="-22.8" font-family="Times,serif" font-size="14.00">    print /dev/stderr</text><text text-anchor="start" x="840" y="-7.8" font-family="Times,serif" font-size="14.00">}'</text></g><!-- do&#45;&gt;awk2 --><g id="edge4" class="edge"><title>do->awk2</title><path fill="none" stroke="black" d="M793.6,-76.95C802.89,-75.03 812.41,-73.05 821.89,-71.09"/><polygon fill="black" stroke="black" points="822.65,-74.51 831.74,-69.05 821.23,-67.65 822.65,-74.51"/></g><!-- awk2&#45;&gt;awk --><g id="edge9" class="edge"><title>awk2->awk</title><path fill="none" stroke="black" d="M831.91,-29.97C743.11,-14.8 607.78,2.45 490,-7 436.6,-11.29 377.62,-20.45 329.35,-29.12"/><polygon fill="black" stroke="black" points="328.43,-25.73 319.22,-30.96 329.68,-32.62 328.43,-25.73"/><text text-anchor="middle" x="544" y="-10.8" font-family="Times,serif" font-size="14.00">STDERR</text></g><!-- sed --><g id="node6" class="node"><title>sed</title><polygon fill="none" stroke="black" points="1224,-67 1064,-67 1064,-31 1224,-31 1224,-67"/><text text-anchor="middle" x="1144" y="-45.3" font-family="Times,serif" font-size="14.00">sed 's/good/perfect/'</text></g><!-- awk2&#45;&gt;sed --><g id="edge5" class="edge"><title>awk2->sed</title><path fill="none" stroke="black" d="M1027.11,-49C1035.94,-49 1044.85,-49 1053.61,-49"/><polygon fill="black" stroke="black" points="1053.81,-52.5 1063.81,-49 1053.81,-45.5 1053.81,-52.5"/></g><!-- sort --><g id="node7" class="node"><title>sort</title><polygon fill="none" stroke="black" points="1315,-67 1261,-67 1261,-31 1315,-31 1315,-67"/><text text-anchor="middle" x="1288" y="-45.3" font-family="Times,serif" font-size="14.00">sort</text></g><!-- sed&#45;&gt;sort --><g id="edge6" class="edge"><title>sed->sort</title><path fill="none" stroke="black" d="M1224.3,-49C1233.49,-49 1242.49,-49 1250.68,-49"/><polygon fill="black" stroke="black" points="1250.91,-52.5 1260.91,-49 1250.91,-45.5 1250.91,-52.5"/></g><!-- grep2&#45;&gt;awk2 --><g id="edge8" class="edge"><title>grep2->awk2</title><path fill="none" stroke="black" d="M598.32,-49C655.82,-49 748.87,-49 821.56,-49"/><polygon fill="black" stroke="black" points="821.67,-52.5 831.67,-49 821.67,-45.5 821.67,-52.5"/></g></g></svg>

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

STATUS **:** SQUARE\_1 **:** SQUARE\_2 **:** SQUARE\_3 **:** SQUARE\_4 **:** SQUARE\_5 **:** SQUARE\_6 **:** SQUARE\_7 **:** SQUARE\_8 **:** SQUARE\_9

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
BEGIN { GAME_NUMBER = 0 }
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

You can execute this code with `echo O:::X::O::::X' | gawk -F ':' -f ./examples/tic-tac-toe/draw.awk` and it'll draw you the following grid:

```text
Player O 

   |   | X 
---+---+---
   | O |   
---+---+---
   |   | X 
```

I then wrote a Pipeawesome configuration file which wraps this:

```yaml file=./examples/tic-tac-toe/draw.pa.yaml
connection:
  initial: "faucet:input | launch:draw | drain:output"
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

Which could be visualized as:

<svg width="348pt" height="99pt" viewBox="0.00 0.00 348.00 99.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 95)"><title>g_get_graph</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-95 344,-95 344,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_nodes_initial</title><polygon fill="none" stroke="black" points="8,-8 8,-83 332,-83 332,-8 8,-8"/><text text-anchor="middle" x="39" y="-67.8" font-family="Times,serif" font-size="14.00">initial:</text></g><!-- f_input --><g id="node1" class="node"><title>f_input</title><polygon fill="#c1ffc1" stroke="black" points="95.07,-52 36.93,-52 16.23,-16 115.77,-16 95.07,-52"/><text text-anchor="middle" x="66" y="-30.3" font-family="Times,serif" font-size="14.00">input</text></g><!-- l_draw --><g id="node2" class="node"><title>l_draw</title><polygon fill="lightblue" stroke="black" points="188,-52 134,-52 134,-16 188,-16 188,-52"/><text text-anchor="middle" x="161" y="-30.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- f_input&#45;&gt;l_draw --><g id="edge1" class="edge"><title>f_input->l_draw</title><path fill="none" stroke="black" d="M105.71,-34C111.7,-34 117.7,-34 123.69,-34"/><polygon fill="black" stroke="black" points="123.79,-37.5 133.79,-34 123.79,-30.5 123.79,-37.5"/></g><!-- d_output --><g id="node3" class="node"><title>d_output</title><polygon fill="lightpink" stroke="black" points="230.75,-16 299.25,-16 323.65,-52 206.35,-52 230.75,-16"/><text text-anchor="middle" x="265" y="-30.3" font-family="Times,serif" font-size="14.00">output</text></g><!-- l_draw&#45;&gt;d_output --><g id="edge2" class="edge"><title>l_draw->d_output</title><path fill="none" stroke="black" d="M188.22,-34C194.85,-34 201.47,-34 208.1,-34"/><polygon fill="black" stroke="black" points="208.51,-37.5 218.51,-34 208.51,-30.5 208.51,-37.5"/></g></g></svg>

<sub>**NOTE**: I got Pipeawesome drew this graph by running `./target/debug/pipeawesome2 graph --config examples/tic-tac-toe/draw.pa.yaml --diagram-only`.</sub>

<sub>**NOTE**: Using `./target/debug/pipeawesome2 graph --config examples/tic-tac-toe/draw.pa.yaml --legend-only` will generate the graphs legend, this is common for all graphs and shown at [near the bottom of page](#pipeawesome-graph-legend).</sub>

In Pipeawesome there are pipes, which connect different types of components. The components types here are `faucet`, `launch` and `drain` with the names of those components being `input`, `draw` and `output`. The names are just names, but they may need to be referenced elsewhere within the configuration file depending on the component type.

You could execute this Pipeawesome configuration file with `echo 'O:::X::O::::X' | ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/draw.pa.yaml`

This is of course, a trivial and pointless example, but it allows me to show you the Pipeawesome file format with minimal complexity.

Lets break it down into it's constituent parts:

#### Connection / Connection Sets

```yaml
connection:
  initial: "faucet:input | launch:draw | drain:output"
```

Connection sets explain how to join components together. There can be multiple connection sets, but here there is just one.

For more information please see [\*\*Pipe Variations, Output Types and Input Priorities](#pipe-variations-output-types-and-input-priorities).

#### Faucet

```yaml
faucet:
  input:
    source: '-'
```

A Faucet is the main way to get data into Pipeawesome from the outside world, the configuration here is for the one named `input`.

For more information please see [**Components > Faucet**](#faucet).

<sub>**NOTE**: It is perfectly valid for a Launch to also generate the initial data.</sub>

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

For more information please see [**Components > Launch**](#launch).

#### Drain

```yaml
drain:
  output:
    destination: '-'
```

This is how data exits Pipeawesome. Output can be sent to STDOUT, STDERR or a file.

For more information please see [**Components > Drain**](#drain).

### Having a Go

The following configuration includes new code, but the configuration adds no concepts that you've not already seen:

```yaml file=./examples/tic-tac-toe/have_a_go.pa.yaml
connection:
  initial_word: "f:input | l:player | l:referee | l:draw | d:output"
drain:
  output: { destination: '-' }
faucet:
  input: { source: '-' }
launch:
  player:
    cmd: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=O', '-f', 'examples/tic-tac-toe/player.awk' ]
  referee:
    cmd: "gawk"
    arg: ['-F', ':', '-f', './examples/tic-tac-toe/referee.awk', 'NF=10', 'OFS=:']
  draw:
    cmd: "gawk"
    arg: [ '-F', ':', '-f', 'examples/tic-tac-toe/draw.awk' ]
```

Which could be visualized as:

<svg width="514pt" height="99pt" viewBox="0.00 0.00 514.00 99.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 95)"><title>g_get_graph</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-95 510,-95 510,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_nodes_initial_word</title><polygon fill="none" stroke="black" points="8,-8 8,-83 498,-83 498,-8 8,-8"/><text text-anchor="middle" x="60.5" y="-67.8" font-family="Times,serif" font-size="14.00">initial_word:</text></g><!-- f_input --><g id="node1" class="node"><title>f_input</title><polygon fill="#c1ffc1" stroke="black" points="95.07,-52 36.93,-52 16.23,-16 115.77,-16 95.07,-52"/><text text-anchor="middle" x="66" y="-30.3" font-family="Times,serif" font-size="14.00">input</text></g><!-- l_player --><g id="node3" class="node"><title>l_player</title><polygon fill="lightblue" stroke="black" points="196,-52 134,-52 134,-16 196,-16 196,-52"/><text text-anchor="middle" x="165" y="-30.3" font-family="Times,serif" font-size="14.00">player</text></g><!-- f_input&#45;&gt;l_player --><g id="edge1" class="edge"><title>f_input->l_player</title><path fill="none" stroke="black" d="M105.45,-34C111.51,-34 117.57,-34 123.63,-34"/><polygon fill="black" stroke="black" points="123.83,-37.5 133.83,-34 123.83,-30.5 123.83,-37.5"/></g><!-- d_output --><g id="node2" class="node"><title>d_output</title><polygon fill="lightpink" stroke="black" points="396.75,-16 465.25,-16 489.65,-52 372.35,-52 396.75,-16"/><text text-anchor="middle" x="431" y="-30.3" font-family="Times,serif" font-size="14.00">output</text></g><!-- l_referee --><g id="node4" class="node"><title>l_referee</title><polygon fill="lightblue" stroke="black" points="282,-52 214,-52 214,-16 282,-16 282,-52"/><text text-anchor="middle" x="248" y="-30.3" font-family="Times,serif" font-size="14.00">referee</text></g><!-- l_player&#45;&gt;l_referee --><g id="edge2" class="edge"><title>l_player->l_referee</title><path fill="none" stroke="black" d="M196.12,-34C198.73,-34 201.33,-34 203.93,-34"/><polygon fill="black" stroke="black" points="203.96,-37.5 213.96,-34 203.96,-30.5 203.96,-37.5"/></g><!-- l_draw --><g id="node5" class="node"><title>l_draw</title><polygon fill="lightblue" stroke="black" points="354,-52 300,-52 300,-16 354,-16 354,-52"/><text text-anchor="middle" x="327" y="-30.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- l_referee&#45;&gt;l_draw --><g id="edge3" class="edge"><title>l_referee->l_draw</title><path fill="none" stroke="black" d="M282.25,-34C284.71,-34 287.17,-34 289.63,-34"/><polygon fill="black" stroke="black" points="289.73,-37.5 299.73,-34 289.73,-30.5 289.73,-37.5"/></g><!-- l_draw&#45;&gt;d_output --><g id="edge4" class="edge"><title>l_draw->d_output</title><path fill="none" stroke="black" d="M354.22,-34C360.85,-34 367.47,-34 374.1,-34"/><polygon fill="black" stroke="black" points="374.51,-37.5 384.51,-34 374.51,-30.5 374.51,-37.5"/></g></g></svg>

It can be executed with `echo 'O:::X::O::::X' | ./target/debug/pipeawesome2 process --config examples/tic-tac-toe/have_a_go.pa.yaml`

The output from this will be the same as previous but with an extra `O` on the grid:

```text
Player O 

   | O | X 
---+---+---
   | O |   
---+---+---
   |   | X 
```

<sub>**NOTE**: There is one extra `O` than in in the input, this was added by `player.awk`.</sub>

### Picking a random player to start the game

#### Code for generating the random player

I figured out that `echo $((RANDOM % 2))::::::::: | sed "s/1/X/" | sed "s/0/O/"` is a single line BASH snippet for selecting a random first player.

However this still means I have to let the selected player take that turn. This means that I must explain what a **Junction** is.

A **Junction** is a prioritized many-to-many connector. Anything that comes into any one of it's inputs will be sent to all of it's outputs.

For more information please see [**Components > Junction**](#junction).

After adding the junctions and supporting changes, the full configuration looks like this:

```yaml file=./examples/tic-tac-toe/random_player.pa.yaml
connection:
  random_selection: "l:random_player | junction:turn"
  player_o_branch: "junction:turn | l:player_o_filter | l:player_o | junction:draw"
  player_x_branch: "junction:turn | l:player_x_filter | l:player_x | junction:draw"
  last_draw: "junction:draw | l:referee | l:draw | d:output"
drain:
  output: { destination: '-' }
launch:
  random_player:
    cmd: "bash"
    arg: [ '-c', 'echo $((RANDOM % 2))::::::::: | sed "s/1/X/" | sed "s/0/O/"' ]
  player_o_filter: { cmd: "grep", arg: [ "--line-buffered", "^O" ] }
  player_o:
    cmd: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=O', '-f', 'examples/tic-tac-toe/player.awk' ]
  player_x_filter: { cmd: "grep", arg: [ "--line-buffered", "^X" ] }
  player_x:
    cmd: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=X', '-f', 'examples/tic-tac-toe/player.awk' ]
  referee:
    cmd: "gawk"
    arg: ['-F', ':', '-f', './examples/tic-tac-toe/referee.awk', 'NF=10', 'OFS=:']
  draw:
    cmd: "gawk"
    arg: [ '-F', ':', '-f', 'examples/tic-tac-toe/draw.awk' ]
```

The graphs drawn by Pipeawesome now become much more interesting:

<svg width="495pt" height="393pt" viewBox="0.00 0.00 495.00 393.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 389)"><title>g_get_graph</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-389 491,-389 491,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_nodes_player_o_branch</title><polygon fill="none" stroke="black" points="11,-155 11,-230 241,-230 241,-155 11,-155"/><text text-anchor="middle" x="79.5" y="-214.8" font-family="Times,serif" font-size="14.00">player_o_branch:</text></g><g id="clust3" class="cluster"><title>cluster_nodes_player_x_branch</title><polygon fill="none" stroke="black" points="249,-155 249,-230 479,-230 479,-155 249,-155"/><text text-anchor="middle" x="317.5" y="-214.8" font-family="Times,serif" font-size="14.00">player_x_branch:</text></g><g id="clust4" class="cluster"><title>cluster_nodes_random_selection</title><polygon fill="none" stroke="black" points="8,-302 8,-377 154,-377 154,-302 8,-302"/><text text-anchor="middle" x="81" y="-361.8" font-family="Times,serif" font-size="14.00">random_selection:</text></g><g id="clust5" class="cluster"><title>cluster_nodes_last_draw</title><polygon fill="none" stroke="black" points="169,-8 169,-83 461,-83 461,-8 169,-8"/><text text-anchor="middle" x="214" y="-67.8" font-family="Times,serif" font-size="14.00">last_draw:</text></g><!-- j_draw --><g id="node1" class="node"><title>j_draw</title><ellipse fill="papayawhip" stroke="black" cx="211" cy="-109" rx="34.39" ry="18"/><text text-anchor="middle" x="211" y="-105.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- l_referee --><g id="node8" class="node"><title>l_referee</title><polygon fill="lightblue" stroke="black" points="245,-52 177,-52 177,-16 245,-16 245,-52"/><text text-anchor="middle" x="211" y="-30.3" font-family="Times,serif" font-size="14.00">referee</text></g><!-- j_draw&#45;&gt;l_referee --><g id="edge1" class="edge"><title>j_draw->l_referee</title><path fill="none" stroke="black" d="M211,-90.7C211,-82.25 211,-71.87 211,-62.37"/><polygon fill="black" stroke="black" points="214.5,-62.18 211,-52.18 207.5,-62.18 214.5,-62.18"/></g><!-- j_turn --><g id="node2" class="node"><title>j_turn</title><ellipse fill="papayawhip" stroke="black" cx="81" cy="-256" rx="30.59" ry="18"/><text text-anchor="middle" x="81" y="-252.3" font-family="Times,serif" font-size="14.00">turn</text></g><!-- l_player_o_filter --><g id="node3" class="node"><title>l_player_o_filter</title><polygon fill="lightblue" stroke="black" points="137,-199 19,-199 19,-163 137,-163 137,-199"/><text text-anchor="middle" x="78" y="-177.3" font-family="Times,serif" font-size="14.00">player_o_filter</text></g><!-- j_turn&#45;&gt;l_player_o_filter --><g id="edge4" class="edge"><title>j_turn->l_player_o_filter</title><path fill="none" stroke="black" d="M80.29,-237.7C79.94,-229.25 79.52,-218.87 79.12,-209.37"/><polygon fill="black" stroke="black" points="82.61,-209.02 78.71,-199.18 75.62,-209.31 82.61,-209.02"/></g><!-- l_player_x_filter --><g id="node5" class="node"><title>l_player_x_filter</title><polygon fill="lightblue" stroke="black" points="375,-199 257,-199 257,-163 375,-163 375,-199"/><text text-anchor="middle" x="316" y="-177.3" font-family="Times,serif" font-size="14.00">player_x_filter</text></g><!-- j_turn&#45;&gt;l_player_x_filter --><g id="edge7" class="edge"><title>j_turn->l_player_x_filter</title><path fill="none" stroke="black" d="M111.66,-254.33C145.41,-252.69 200.74,-247.31 245,-230 260.2,-224.05 275.41,-214.39 287.83,-205.29"/><polygon fill="black" stroke="black" points="290.1,-207.96 295.95,-199.13 285.86,-202.39 290.1,-207.96"/></g><!-- l_player_o --><g id="node4" class="node"><title>l_player_o</title><polygon fill="lightblue" stroke="black" points="232.5,-199 155.5,-199 155.5,-163 232.5,-163 232.5,-199"/><text text-anchor="middle" x="194" y="-177.3" font-family="Times,serif" font-size="14.00">player_o</text></g><!-- l_player_o_filter&#45;&gt;l_player_o --><g id="edge5" class="edge"><title>l_player_o_filter->l_player_o</title><path fill="none" stroke="black" d="M137.36,-181C139.85,-181 142.34,-181 144.83,-181"/><polygon fill="black" stroke="black" points="145.06,-184.5 155.06,-181 145.06,-177.5 145.06,-184.5"/></g><!-- l_player_o&#45;&gt;j_draw --><g id="edge6" class="edge"><title>l_player_o->j_draw</title><path fill="none" stroke="black" d="M198.2,-162.7C200.1,-154.9 202.38,-145.51 204.48,-136.83"/><polygon fill="black" stroke="black" points="207.89,-137.65 206.85,-127.1 201.08,-136 207.89,-137.65"/></g><!-- l_player_x --><g id="node6" class="node"><title>l_player_x</title><polygon fill="lightblue" stroke="black" points="470.5,-199 393.5,-199 393.5,-163 470.5,-163 470.5,-199"/><text text-anchor="middle" x="432" y="-177.3" font-family="Times,serif" font-size="14.00">player_x</text></g><!-- l_player_x_filter&#45;&gt;l_player_x --><g id="edge8" class="edge"><title>l_player_x_filter->l_player_x</title><path fill="none" stroke="black" d="M375.36,-181C377.85,-181 380.34,-181 382.83,-181"/><polygon fill="black" stroke="black" points="383.06,-184.5 393.06,-181 383.06,-177.5 383.06,-184.5"/></g><!-- l_player_x&#45;&gt;j_draw --><g id="edge9" class="edge"><title>l_player_x->j_draw</title><path fill="none" stroke="black" d="M401.75,-162.94C395.96,-160.05 389.88,-157.26 384,-155 340.5,-138.26 288.5,-125.68 252.95,-118.13"/><polygon fill="black" stroke="black" points="253.43,-114.66 242.92,-116.04 252,-121.51 253.43,-114.66"/></g><!-- l_random_player --><g id="node7" class="node"><title>l_random_player</title><polygon fill="lightblue" stroke="black" points="142.5,-346 19.5,-346 19.5,-310 142.5,-310 142.5,-346"/><text text-anchor="middle" x="81" y="-324.3" font-family="Times,serif" font-size="14.00">random_player</text></g><!-- l_random_player&#45;&gt;j_turn --><g id="edge10" class="edge"><title>l_random_player->j_turn</title><path fill="none" stroke="black" d="M81,-309.7C81,-301.98 81,-292.71 81,-284.11"/><polygon fill="black" stroke="black" points="84.5,-284.1 81,-274.1 77.5,-284.1 84.5,-284.1"/></g><!-- l_draw --><g id="node9" class="node"><title>l_draw</title><polygon fill="lightblue" stroke="black" points="317,-52 263,-52 263,-16 317,-16 317,-52"/><text text-anchor="middle" x="290" y="-30.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- l_referee&#45;&gt;l_draw --><g id="edge2" class="edge"><title>l_referee->l_draw</title><path fill="none" stroke="black" d="M245.25,-34C247.71,-34 250.17,-34 252.63,-34"/><polygon fill="black" stroke="black" points="252.73,-37.5 262.73,-34 252.73,-30.5 252.73,-37.5"/></g><!-- d_output --><g id="node10" class="node"><title>d_output</title><polygon fill="lightpink" stroke="black" points="359.75,-16 428.25,-16 452.65,-52 335.35,-52 359.75,-16"/><text text-anchor="middle" x="394" y="-30.3" font-family="Times,serif" font-size="14.00">output</text></g><!-- l_draw&#45;&gt;d_output --><g id="edge3" class="edge"><title>l_draw->d_output</title><path fill="none" stroke="black" d="M317.22,-34C323.85,-34 330.47,-34 337.1,-34"/><polygon fill="black" stroke="black" points="337.51,-37.5 347.51,-34 337.51,-30.5 337.51,-37.5"/></g></g></svg>

The changes are:

The Faucet configuration has been completely removed (it is not required), in this situation, the initial message comes from `l:random_player`

As well as adding `l:random_player` I also added `l:player_o_filter`, `l:player_x_filter`, `l:referee` and changed `l:player` into `l:player_o` and `l:player_x`, but the overall format should not be too surprising by now.

The big change is that there are now multiple keys / connections sets / lines within `connection:`. You may notice that the `random_selection:` connection set writes to `junction:turn` but `junction:turn` is read in both the `player_o_branch` and `player_x_branch` connection sets, which in turn both write to `junction:draw`. The connection set names are completely arbitrary, though the must be unique.

<sub>**NOTE**: It is important to know that both `player_o_filter` and `player_x_filter` both recieve the line generated by l:random player. It is just the case that one of them always filters it out.</sub>

Running this code results in a grid where either an `O` or `X` could be anywhere on the grid:

```text
Player X 

   |   | X 
---+---+---
   |   |   
---+---+---
   |   |   
```

### A complete game

To create the full game, there are two more things that need to happen:

1.  Multiple turns - To complete a game we must have multiple turns take place.
2.  Alternating players - The player that takes the next turn must different from the previous turn.

#### Multiple turns

This is simple, all we need to do is take our previous configuration, add a junction between `launch:referee` and `launch:draw` and feed a new branch all the way back into `junction:turn`. The configuration now looks like:

```yaml file=examples/tic-tac-toe/multiple_turns.pa.yaml -d | diagram-dot svg
connection:
  random_selection: "l:random_player | j:turn"
  player_o_branch: "j:turn | l:player_o_filter | l:player_o | j:draw"
  player_x_branch: "j:turn | l:player_x_filter | l:player_x | j:draw"
  last_draw: "j:draw | l:referee | j:loop | l:draw | d:output"
  looper: "j:loop | [5]j:turn"
drain:
  output: { destination: '-' }
launch:
  random_player:
    cmd: "bash"
    arg:
      - '-c'
      - 'echo $((RANDOM % 2))::::::::: | sed "s/1/X/" | sed "s/0/O/"'
  player_o_filter: { cmd: "grep", arg: [ "--line-buffered", "^O" ] }
  player_o:
    cmd: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=O', '-f', 'examples/tic-tac-toe/player.awk' ]
  player_x_filter: { cmd: "grep", arg: [ "--line-buffered", "^X" ] }
  player_x:
    cmd: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=X', '-f', 'examples/tic-tac-toe/player.awk' ]
  referee:
    cmd: "gawk"
    arg: ['-F', ':', '-f', './examples/tic-tac-toe/referee.awk', 'NF=10', 'OFS=:']
  referee:
    cmd: "gawk"
    arg: ['-F', ':', '-f', './examples/tic-tac-toe/referee.awk', 'NF=10', 'OFS=:']
  draw:
    cmd: "gawk"
    arg: [ '-F', ':', '-f', 'examples/tic-tac-toe/draw.awk' ]
```

Which could be visualized as:

<svg width="759pt" height="393pt" viewBox="0.00 0.00 759.00 393.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 389)"><title>g_get_graph</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-389 755,-389 755,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_nodes_player_o_branch</title><polygon fill="none" stroke="black" points="8,-155 8,-230 238,-230 238,-155 8,-155"/><text text-anchor="middle" x="76.5" y="-214.8" font-family="Times,serif" font-size="14.00">player_o_branch:</text></g><g id="clust3" class="cluster"><title>cluster_nodes_player_x_branch</title><polygon fill="none" stroke="black" points="246,-155 246,-230 476,-230 476,-155 246,-155"/><text text-anchor="middle" x="314.5" y="-214.8" font-family="Times,serif" font-size="14.00">player_x_branch:</text></g><g id="clust4" class="cluster"><title>cluster_nodes_random_selection</title><polygon fill="none" stroke="black" points="240,-302 240,-377 386,-377 386,-302 240,-302"/><text text-anchor="middle" x="313" y="-361.8" font-family="Times,serif" font-size="14.00">random_selection:</text></g><g id="clust5" class="cluster"><title>cluster_nodes_last_draw</title><polygon fill="none" stroke="black" points="371,-8 371,-83 743,-83 743,-8 371,-8"/><text text-anchor="middle" x="416" y="-67.8" font-family="Times,serif" font-size="14.00">last_draw:</text></g><!-- j_draw --><g id="node1" class="node"><title>j_draw</title><ellipse fill="papayawhip" stroke="black" cx="413" cy="-109" rx="34.39" ry="18"/><text text-anchor="middle" x="413" y="-105.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- l_referee --><g id="node10" class="node"><title>l_referee</title><polygon fill="lightblue" stroke="black" points="447,-52 379,-52 379,-16 447,-16 447,-52"/><text text-anchor="middle" x="413" y="-30.3" font-family="Times,serif" font-size="14.00">referee</text></g><!-- j_draw&#45;&gt;l_referee --><g id="edge1" class="edge"><title>j_draw->l_referee</title><path fill="none" stroke="black" d="M413,-90.7C413,-82.25 413,-71.87 413,-62.37"/><polygon fill="black" stroke="black" points="416.5,-62.18 413,-52.18 409.5,-62.18 416.5,-62.18"/></g><!-- j_turn --><g id="node2" class="node"><title>j_turn</title><ellipse fill="papayawhip" stroke="black" cx="313" cy="-256" rx="30.59" ry="18"/><text text-anchor="middle" x="313" y="-252.3" font-family="Times,serif" font-size="14.00">turn</text></g><!-- l_player_o_filter --><g id="node3" class="node"><title>l_player_o_filter</title><polygon fill="lightblue" stroke="black" points="134,-199 16,-199 16,-163 134,-163 134,-199"/><text text-anchor="middle" x="75" y="-177.3" font-family="Times,serif" font-size="14.00">player_o_filter</text></g><!-- j_turn&#45;&gt;l_player_o_filter --><g id="edge6" class="edge"><title>j_turn->l_player_o_filter</title><path fill="none" stroke="black" d="M282.42,-254.74C247.51,-253.5 189.26,-248.49 143,-230 128.42,-224.17 113.97,-214.65 102.17,-205.61"/><polygon fill="black" stroke="black" points="104.08,-202.66 94.07,-199.18 99.72,-208.14 104.08,-202.66"/></g><!-- l_player_x_filter --><g id="node5" class="node"><title>l_player_x_filter</title><polygon fill="lightblue" stroke="black" points="372,-199 254,-199 254,-163 372,-163 372,-199"/><text text-anchor="middle" x="313" y="-177.3" font-family="Times,serif" font-size="14.00">player_x_filter</text></g><!-- j_turn&#45;&gt;l_player_x_filter --><g id="edge9" class="edge"><title>j_turn->l_player_x_filter</title><path fill="none" stroke="black" d="M313,-237.7C313,-229.25 313,-218.87 313,-209.37"/><polygon fill="black" stroke="black" points="316.5,-209.18 313,-199.18 309.5,-209.18 316.5,-209.18"/></g><!-- l_player_o --><g id="node4" class="node"><title>l_player_o</title><polygon fill="lightblue" stroke="black" points="229.5,-199 152.5,-199 152.5,-163 229.5,-163 229.5,-199"/><text text-anchor="middle" x="191" y="-177.3" font-family="Times,serif" font-size="14.00">player_o</text></g><!-- l_player_o_filter&#45;&gt;l_player_o --><g id="edge7" class="edge"><title>l_player_o_filter->l_player_o</title><path fill="none" stroke="black" d="M134.36,-181C136.85,-181 139.34,-181 141.83,-181"/><polygon fill="black" stroke="black" points="142.06,-184.5 152.06,-181 142.06,-177.5 142.06,-184.5"/></g><!-- l_player_o&#45;&gt;j_draw --><g id="edge8" class="edge"><title>l_player_o->j_draw</title><path fill="none" stroke="black" d="M223.5,-162.88C229.56,-160.03 235.9,-157.27 242,-155 285.07,-138.96 336.25,-126.29 371.32,-118.53"/><polygon fill="black" stroke="black" points="372.19,-121.93 381.22,-116.38 370.7,-115.09 372.19,-121.93"/></g><!-- l_player_x --><g id="node6" class="node"><title>l_player_x</title><polygon fill="lightblue" stroke="black" points="467.5,-199 390.5,-199 390.5,-163 467.5,-163 467.5,-199"/><text text-anchor="middle" x="429" y="-177.3" font-family="Times,serif" font-size="14.00">player_x</text></g><!-- l_player_x_filter&#45;&gt;l_player_x --><g id="edge10" class="edge"><title>l_player_x_filter->l_player_x</title><path fill="none" stroke="black" d="M372.36,-181C374.85,-181 377.34,-181 379.83,-181"/><polygon fill="black" stroke="black" points="380.06,-184.5 390.06,-181 380.06,-177.5 380.06,-184.5"/></g><!-- l_player_x&#45;&gt;j_draw --><g id="edge11" class="edge"><title>l_player_x->j_draw</title><path fill="none" stroke="black" d="M425.04,-162.7C423.28,-154.98 421.16,-145.71 419.2,-137.11"/><polygon fill="black" stroke="black" points="422.55,-136.07 416.91,-127.1 415.73,-137.63 422.55,-136.07"/></g><!-- l_random_player --><g id="node7" class="node"><title>l_random_player</title><polygon fill="lightblue" stroke="black" points="374.5,-346 251.5,-346 251.5,-310 374.5,-310 374.5,-346"/><text text-anchor="middle" x="313" y="-324.3" font-family="Times,serif" font-size="14.00">random_player</text></g><!-- l_random_player&#45;&gt;j_turn --><g id="edge12" class="edge"><title>l_random_player->j_turn</title><path fill="none" stroke="black" d="M313,-309.7C313,-301.98 313,-292.71 313,-284.11"/><polygon fill="black" stroke="black" points="316.5,-284.1 313,-274.1 309.5,-284.1 316.5,-284.1"/></g><!-- d_output --><g id="node8" class="node"><title>d_output</title><polygon fill="lightpink" stroke="black" points="641.75,-16 710.25,-16 734.65,-52 617.35,-52 641.75,-16"/><text text-anchor="middle" x="676" y="-30.3" font-family="Times,serif" font-size="14.00">output</text></g><!-- j_loop --><g id="node9" class="node"><title>j_loop</title><ellipse fill="papayawhip" stroke="black" cx="496" cy="-34" rx="30.59" ry="18"/><text text-anchor="middle" x="496" y="-30.3" font-family="Times,serif" font-size="14.00">loop</text></g><!-- j_loop&#45;&gt;j_turn --><g id="edge5" class="edge"><title>j_loop->j_turn</title><path fill="none" stroke="black" d="M500.7,-51.79C510.07,-89.28 526.41,-180.39 480,-230 463.31,-247.84 397.59,-253.05 353.74,-254.51"/><polygon fill="black" stroke="black" points="353.55,-251.01 343.65,-254.79 353.75,-258.01 353.55,-251.01"/></g><!-- l_draw --><g id="node11" class="node"><title>l_draw</title><polygon fill="lightblue" stroke="black" points="599,-52 545,-52 545,-16 599,-16 599,-52"/><text text-anchor="middle" x="572" y="-30.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- j_loop&#45;&gt;l_draw --><g id="edge3" class="edge"><title>j_loop->l_draw</title><path fill="none" stroke="black" d="M526.58,-34C529.22,-34 531.86,-34 534.5,-34"/><polygon fill="black" stroke="black" points="534.68,-37.5 544.68,-34 534.68,-30.5 534.68,-37.5"/></g><!-- l_referee&#45;&gt;j_loop --><g id="edge2" class="edge"><title>l_referee->j_loop</title><path fill="none" stroke="black" d="M447.04,-34C449.82,-34 452.59,-34 455.36,-34"/><polygon fill="black" stroke="black" points="455.4,-37.5 465.4,-34 455.4,-30.5 455.4,-37.5"/></g><!-- l_draw&#45;&gt;d_output --><g id="edge4" class="edge"><title>l_draw->d_output</title><path fill="none" stroke="black" d="M599.22,-34C605.85,-34 612.47,-34 619.1,-34"/><polygon fill="black" stroke="black" points="619.51,-37.5 629.51,-34 619.51,-30.5 619.51,-37.5"/></g></g></svg>

**NOTE:** This graph is identical except the extra `junction:loop` and the line from it that goes all the way back to turn.

This configuration however results to a non-thrilling game however as only one player ever gets a go!

```text
Player X 

   |   | X 
---+---+---
   |   |   
---+---+---
   |   |   

Player X 

   |   | X 
---+---+---
 X |   |   
---+---+---
   |   |   

Player X 

   | X | X 
---+---+---
 X |   |   
---+---+---
   |   |   

Player X 

   | X | X 
---+---+---
 X |   |   
---+---+---
 X |   |   

Player X 

   | X | X 
---+---+---
 X |   | X 
---+---+---
 X |   |   

Player X WON!

 X | X | X 
---+---+---
 X |   | X 
---+---+---
 X |   |   
```

#### Alternating players

To get the player taking a turn to alternate we just need to put in some code that swaps the first character between "X" and "O between `junction:loop` and `junction:turn`. This component is called `turn_swapper` in the configuration below:

```yaml file=examples/tic-tac-toe/pa.yaml -d | diagram-dot svg
connection:
  random_selection: "l:random_player | j:turn"
  player_o_branch: "j:turn | l:player_o_filter | l:player_o | j:draw"
  player_x_branch: "j:turn | l:player_x_filter | l:player_x | j:draw"
  last_draw: "j:draw | l:referee | j:loop | l:draw | d:output"
  looper: "j:loop | l:turn_swapper | j:turn"
drain:
  output: { destination: '-' }
launch:
  random_player:
    cmd: "bash"
    arg:
      - '-c'
      - 'echo $((RANDOM % 2))::::::::: | sed "s/1/X/" | sed "s/0/O/"'
  player_o_filter: { cmd: "grep", arg: [ "--line-buffered", "^O" ] }
  player_o:
    cmd: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=O', '-f', 'examples/tic-tac-toe/player.awk' ]
  player_x_filter: { cmd: "grep", arg: [ "--line-buffered", "^X" ] }
  player_x:
    cmd: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=X', '-f', 'examples/tic-tac-toe/player.awk' ]
  referee:
    cmd: "gawk"
    arg: ['-F', ':', '-f', './examples/tic-tac-toe/referee.awk', 'NF=10', 'OFS=:']
  draw:
    cmd: "gawk"
    arg: [ '-F', ':', '-f', 'examples/tic-tac-toe/draw.awk' ]
  turn_swapper:
    cmd: "sed"
    arg:
      - "--unbuffered"
      - |
        s/^O/9/
        s/^X/O/
        s/^9/X/
```

Which could be visualized as:

<svg width="496pt" height="365pt" viewBox="0.00 0.00 496.00 365.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 361)"><title>g_get_graph</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-361 492,-361 492,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_nodes_looper</title><polygon fill="none" stroke="black" points="117,-155 117,-230 249,-230 249,-155 117,-155"/><text text-anchor="middle" x="150" y="-214.8" font-family="Times,serif" font-size="14.00">looper:</text></g><g id="clust3" class="cluster"><title>cluster_nodes_player_o_branch</title><polygon fill="none" stroke="black" points="8,-8 8,-83 238,-83 238,-8 8,-8"/><text text-anchor="middle" x="76.5" y="-67.8" font-family="Times,serif" font-size="14.00">player_o_branch:</text></g><g id="clust4" class="cluster"><title>cluster_nodes_player_x_branch</title><polygon fill="none" stroke="black" points="250,-8 250,-83 480,-83 480,-8 250,-8"/><text text-anchor="middle" x="318.5" y="-67.8" font-family="Times,serif" font-size="14.00">player_x_branch:</text></g><g id="clust5" class="cluster"><title>cluster_nodes_random_selection</title><polygon fill="none" stroke="black" points="257,-155 257,-230 403,-230 403,-155 257,-155"/><text text-anchor="middle" x="330" y="-214.8" font-family="Times,serif" font-size="14.00">random_selection:</text></g><g id="clust6" class="cluster"><title>cluster_nodes_last_draw</title><polygon fill="none" stroke="black" points="58,-238 58,-313 430,-313 430,-238 58,-238"/><text text-anchor="middle" x="103" y="-297.8" font-family="Times,serif" font-size="14.00">last_draw:</text></g><!-- j_draw --><g id="node1" class="node"><title>j_draw</title><ellipse fill="papayawhip" stroke="black" cx="100" cy="-339" rx="34.39" ry="18"/><text text-anchor="middle" x="100" y="-335.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- l_referee --><g id="node10" class="node"><title>l_referee</title><polygon fill="lightblue" stroke="black" points="134,-282 66,-282 66,-246 134,-246 134,-282"/><text text-anchor="middle" x="100" y="-260.3" font-family="Times,serif" font-size="14.00">referee</text></g><!-- j_draw&#45;&gt;l_referee --><g id="edge1" class="edge"><title>j_draw->l_referee</title><path fill="none" stroke="black" d="M100,-320.7C100,-312.25 100,-301.87 100,-292.37"/><polygon fill="black" stroke="black" points="103.5,-292.18 100,-282.18 96.5,-292.18 103.5,-292.18"/></g><!-- j_turn --><g id="node2" class="node"><title>j_turn</title><ellipse fill="papayawhip" stroke="black" cx="240" cy="-109" rx="30.59" ry="18"/><text text-anchor="middle" x="240" y="-105.3" font-family="Times,serif" font-size="14.00">turn</text></g><!-- l_player_o_filter --><g id="node4" class="node"><title>l_player_o_filter</title><polygon fill="lightblue" stroke="black" points="230,-52 112,-52 112,-16 230,-16 230,-52"/><text text-anchor="middle" x="171" y="-30.3" font-family="Times,serif" font-size="14.00">player_o_filter</text></g><!-- j_turn&#45;&gt;l_player_o_filter --><g id="edge7" class="edge"><title>j_turn->l_player_o_filter</title><path fill="none" stroke="black" d="M225.7,-92.87C216.59,-83.24 204.59,-70.53 194.14,-59.48"/><polygon fill="black" stroke="black" points="196.63,-57.02 187.21,-52.15 191.54,-61.82 196.63,-57.02"/></g><!-- l_player_x_filter --><g id="node6" class="node"><title>l_player_x_filter</title><polygon fill="lightblue" stroke="black" points="376,-52 258,-52 258,-16 376,-16 376,-52"/><text text-anchor="middle" x="317" y="-30.3" font-family="Times,serif" font-size="14.00">player_x_filter</text></g><!-- j_turn&#45;&gt;l_player_x_filter --><g id="edge10" class="edge"><title>j_turn->l_player_x_filter</title><path fill="none" stroke="black" d="M255.58,-93.23C265.89,-83.45 279.65,-70.41 291.53,-59.15"/><polygon fill="black" stroke="black" points="294.16,-61.47 299.01,-52.05 289.35,-56.39 294.16,-61.47"/></g><!-- l_turn_swapper --><g id="node3" class="node"><title>l_turn_swapper</title><polygon fill="lightblue" stroke="black" points="240.5,-199 125.5,-199 125.5,-163 240.5,-163 240.5,-199"/><text text-anchor="middle" x="183" y="-177.3" font-family="Times,serif" font-size="14.00">turn_swapper</text></g><!-- l_turn_swapper&#45;&gt;j_turn --><g id="edge6" class="edge"><title>l_turn_swapper->j_turn</title><path fill="none" stroke="black" d="M197.09,-162.7C204.23,-153.93 213.01,-143.15 220.76,-133.63"/><polygon fill="black" stroke="black" points="223.64,-135.64 227.24,-125.67 218.21,-131.22 223.64,-135.64"/></g><!-- l_player_o --><g id="node5" class="node"><title>l_player_o</title><polygon fill="lightblue" stroke="black" points="93.5,-52 16.5,-52 16.5,-16 93.5,-16 93.5,-52"/><text text-anchor="middle" x="55" y="-30.3" font-family="Times,serif" font-size="14.00">player_o</text></g><!-- l_player_o_filter&#45;&gt;l_player_o --><g id="edge8" class="edge"><title>l_player_o_filter->l_player_o</title><path fill="none" stroke="black" d="M111.98,-34C109.19,-34 106.4,-34 103.61,-34"/><polygon fill="black" stroke="black" points="103.52,-30.5 93.52,-34 103.52,-37.5 103.52,-30.5"/></g><!-- l_player_o&#45;&gt;j_draw --><g id="edge9" class="edge"><title>l_player_o->j_draw</title><path fill="none" stroke="black" d="M49.29,-52.22C35.11,-98.36 2.94,-227.13 54,-313 56.25,-316.78 59.29,-320.02 62.74,-322.8"/><polygon fill="black" stroke="black" points="61.09,-325.9 71.37,-328.46 64.93,-320.05 61.09,-325.9"/></g><!-- l_player_x --><g id="node7" class="node"><title>l_player_x</title><polygon fill="lightblue" stroke="black" points="471.5,-52 394.5,-52 394.5,-16 471.5,-16 471.5,-52"/><text text-anchor="middle" x="433" y="-30.3" font-family="Times,serif" font-size="14.00">player_x</text></g><!-- l_player_x_filter&#45;&gt;l_player_x --><g id="edge11" class="edge"><title>l_player_x_filter->l_player_x</title><path fill="none" stroke="black" d="M376.36,-34C378.85,-34 381.34,-34 383.83,-34"/><polygon fill="black" stroke="black" points="384.06,-37.5 394.06,-34 384.06,-30.5 384.06,-37.5"/></g><!-- l_player_x&#45;&gt;j_draw --><g id="edge12" class="edge"><title>l_player_x->j_draw</title><path fill="none" stroke="black" d="M440.03,-52.19C458.53,-100.56 502.61,-239.06 434,-313 414.71,-333.79 230.89,-337.41 144.48,-337.96"/><polygon fill="black" stroke="black" points="144.42,-334.46 134.44,-338.01 144.46,-341.46 144.42,-334.46"/></g><!-- l_random_player --><g id="node8" class="node"><title>l_random_player</title><polygon fill="lightblue" stroke="black" points="388.5,-199 265.5,-199 265.5,-163 388.5,-163 388.5,-199"/><text text-anchor="middle" x="327" y="-177.3" font-family="Times,serif" font-size="14.00">random_player</text></g><!-- l_random_player&#45;&gt;j_turn --><g id="edge13" class="edge"><title>l_random_player->j_turn</title><path fill="none" stroke="black" d="M305.49,-162.7C293.36,-152.93 278.12,-140.67 265.4,-130.44"/><polygon fill="black" stroke="black" points="267.45,-127.59 257.46,-124.05 263.06,-133.05 267.45,-127.59"/></g><!-- d_output --><g id="node9" class="node"><title>d_output</title><polygon fill="lightpink" stroke="black" points="328.75,-246 397.25,-246 421.65,-282 304.35,-282 328.75,-246"/><text text-anchor="middle" x="363" y="-260.3" font-family="Times,serif" font-size="14.00">output</text></g><!-- j_loop --><g id="node12" class="node"><title>j_loop</title><ellipse fill="papayawhip" stroke="black" cx="183" cy="-264" rx="30.59" ry="18"/><text text-anchor="middle" x="183" y="-260.3" font-family="Times,serif" font-size="14.00">loop</text></g><!-- l_referee&#45;&gt;j_loop --><g id="edge2" class="edge"><title>l_referee->j_loop</title><path fill="none" stroke="black" d="M134.04,-264C136.82,-264 139.59,-264 142.36,-264"/><polygon fill="black" stroke="black" points="142.4,-267.5 152.4,-264 142.4,-260.5 142.4,-267.5"/></g><!-- l_draw --><g id="node11" class="node"><title>l_draw</title><polygon fill="lightblue" stroke="black" points="286,-282 232,-282 232,-246 286,-246 286,-282"/><text text-anchor="middle" x="259" y="-260.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- l_draw&#45;&gt;d_output --><g id="edge4" class="edge"><title>l_draw->d_output</title><path fill="none" stroke="black" d="M286.22,-264C292.85,-264 299.47,-264 306.1,-264"/><polygon fill="black" stroke="black" points="306.51,-267.5 316.51,-264 306.51,-260.5 306.51,-267.5"/></g><!-- j_loop&#45;&gt;l_turn_swapper --><g id="edge5" class="edge"><title>j_loop->l_turn_swapper</title><path fill="none" stroke="black" d="M183,-245.82C183,-235.19 183,-221.31 183,-209.2"/><polygon fill="black" stroke="black" points="186.5,-209.15 183,-199.15 179.5,-209.15 186.5,-209.15"/></g><!-- j_loop&#45;&gt;l_draw --><g id="edge3" class="edge"><title>j_loop->l_draw</title><path fill="none" stroke="black" d="M213.58,-264C216.22,-264 218.86,-264 221.5,-264"/><polygon fill="black" stroke="black" points="221.68,-267.5 231.68,-264 221.68,-260.5 221.68,-267.5"/></g></g></svg>

The end result is a (somewhat) realisic looking game of tic-tac-toe where the players take turns and someone wins (or it is a draw):

```text
Player O 

   |   | O 
---+---+---
   |   |   
---+---+---
   |   |   

Player X 

   |   | O 
---+---+---
 X |   |   
---+---+---
   |   |   

Player O 

   |   | O 
---+---+---
 X | O |   
---+---+---
   |   |   

Player X 

   | X | O 
---+---+---
 X | O |   
---+---+---
   |   |   

Player O 

 O | X | O 
---+---+---
 X | O |   
---+---+---
   |   |   

Player X 

 O | X | O 
---+---+---
 X | O | X 
---+---+---
   |   |   

Player O 

 O | X | O 
---+---+---
 X | O | X 
---+---+---
   | O |   

Player X 

 O | X | O 
---+---+---
 X | O | X 
---+---+---
   | O | X 

DRAW!

 O | X | O 
---+---+---
 X | O | X 
---+---+---
 O | O | X 
```

### Pipeawesome Graph Legend

You can draw a graph legend by running the command `./target/debug/pipeawesome2 graph --config [YOUR_CONFIG_HERE] --legend-only`. The output will be Graphviz DOT.

<svg width="459pt" height="85pt" viewBox="0.00 0.00 459.00 85.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 81)"><title>g_get_graph</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-81 455,-81 455,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_legend_launch</title><polygon fill="none" stroke="black" points="8,-8 8,-69 72,-69 72,-8 8,-8"/><text text-anchor="middle" x="40" y="-53.8" font-family="Times,serif" font-size="14.00">launch</text></g><g id="clust3" class="cluster"><title>cluster_legend_buffer</title><polygon fill="none" stroke="black" points="80,-8 80,-69 139,-69 139,-8 80,-8"/><text text-anchor="middle" x="109.5" y="-53.8" font-family="Times,serif" font-size="14.00">buffer</text></g><g id="clust4" class="cluster"><title>cluster_legend_regulator</title><polygon fill="none" stroke="black" points="147,-8 147,-69 230,-69 230,-8 147,-8"/><text text-anchor="middle" x="188.5" y="-53.8" font-family="Times,serif" font-size="14.00">regulator</text></g><g id="clust5" class="cluster"><title>cluster_legend_junction</title><polygon fill="none" stroke="black" points="238,-8 238,-69 312,-69 312,-8 238,-8"/><text text-anchor="middle" x="275" y="-53.8" font-family="Times,serif" font-size="14.00">junction</text></g><g id="clust6" class="cluster"><title>cluster_legend_faucet</title><polygon fill="none" stroke="black" points="320,-8 320,-69 381,-69 381,-8 320,-8"/><text text-anchor="middle" x="350.5" y="-53.8" font-family="Times,serif" font-size="14.00">faucet</text></g><g id="clust7" class="cluster"><title>cluster_legend_drain</title><polygon fill="none" stroke="black" points="389,-8 389,-69 443,-69 443,-8 389,-8"/><text text-anchor="middle" x="416" y="-53.8" font-family="Times,serif" font-size="14.00">drain</text></g><!-- legend_launch --><g id="node1" class="node"><title>legend_launch</title><polygon fill="lightblue" stroke="black" points="51,-38 29,-38 29,-16 51,-16 51,-38"/></g><!-- legend_buffer --><g id="node2" class="node"><title>legend_buffer</title><polygon fill="lightgray" stroke="black" points="98,-23.6 109,-16 120,-23.6 119.99,-35.9 98.01,-35.9 98,-23.6"/></g><!-- legend_regulator --><g id="node3" class="node"><title>legend_regulator</title><polygon fill="lightgray" stroke="black" points="199,-30.4 188,-38 177,-30.4 177.01,-18.1 198.99,-18.1 199,-30.4"/></g><!-- legend_junction --><g id="node4" class="node"><title>legend_junction</title><ellipse fill="papayawhip" stroke="black" cx="275" cy="-27" rx="11" ry="11"/></g><!-- legend_faucet --><g id="node5" class="node"><title>legend_faucet</title><polygon fill="#c1ffc1" stroke="black" points="356.42,-38 343.58,-38 339,-16 361,-16 356.42,-38"/></g><!-- legend_drain --><g id="node6" class="node"><title>legend_drain</title><polygon fill="lightpink" stroke="black" points="409.58,-16 422.42,-16 427,-38 405,-38 409.58,-16"/></g></g></svg>

## Component Types

Component types can be:

*   [**Faucet**](#faucet): A source of input.
*   [**Launch**](#launch): A running program.
*   [**Drain**](#drain): A destination were final data will be sent to.
*   [**Junction**](#junction): A many to many connector which can manage priorities of incoming data.
*   [**Buffer / Regulator**](#buffer-and-regulator): Stores an infinite amount of messages / Regulates the amount of messages

The pipes can be extended to configure how to handle broken pipes (when a recieving program exits before a sending program) and you can also control whether they're sending STDIN, STDOUT or EXIT statuses.

<sub>Note: There are diagrams in this section, the legend for this is shown at [#component-diagram-legend](#component-diagram-legend)</sub>

### Component: Faucet

<svg width="366pt" height="106pt" viewBox="0.00 0.00 365.78 106.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 102)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-102 361.78,-102 361.78,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_faucet</title><path fill="none" stroke="lightgrey" d="M20,-8C20,-8 263.78,-8 263.78,-8 269.78,-8 275.78,-14 275.78,-20 275.78,-20 275.78,-78 275.78,-78 275.78,-84 269.78,-90 263.78,-90 263.78,-90 20,-90 20,-90 14,-90 8,-84 8,-78 8,-78 8,-20 8,-20 8,-14 14,-8 20,-8"/><text text-anchor="middle" x="51" y="-70" font-family="Times,serif" font-size="20.00">Faucet</text></g><!-- faucet_pull --><g id="node1" class="node"><title>faucet_pull</title><ellipse fill="none" stroke="black" cx="90.74" cy="-34" rx="74.99" ry="18"/><text text-anchor="middle" x="90.74" y="-30.3" font-family="Times,serif" font-size="14.00">pull: file/stdin</text></g><!-- faucet_push --><g id="node2" class="node"><title>faucet_push</title><ellipse fill="none" stroke="black" cx="234.63" cy="-34" rx="33.29" ry="18"/><text text-anchor="middle" x="234.63" y="-30.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- faucet_pull&#45;&gt;faucet_push --><g id="edge1" class="edge"><title>faucet_pull->faucet_push</title><path fill="none" stroke="black" d="M165.6,-34C174.31,-34 182.96,-34 191.02,-34"/><polygon fill="black" stroke="black" points="191.24,-37.5 201.24,-34 191.24,-30.5 191.24,-37.5"/></g><!-- fauct_exit_pull --><g id="node3" class="node"><title>fauct_exit_pull</title></g><!-- faucet_push&#45;&gt;fauct_exit_pull --><g id="edge2" class="edge"><title>faucet_push->fauct_exit_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M268.03,-34C276.19,-34 284.99,-34 293.33,-34"/><polygon fill="black" stroke="black" points="293.55,-37.5 303.55,-34 293.55,-30.5 293.55,-37.5"/></g></g></svg>

    faucet:
      tap:
        input: "-",

A Faucet is the main way to get data into Pipeawesome. Faucets have a property called `source` which can be "`-`" for STDIN or a filename which will be read from.

### Component: Launch

<svg width="769pt" height="266pt" viewBox="0.00 0.00 769.16 266.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 262)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-262 765.16,-262 765.16,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_launch</title><path fill="none" stroke="lightgrey" d="M94,-8C94,-8 651.16,-8 651.16,-8 657.16,-8 663.16,-14 663.16,-20 663.16,-20 663.16,-238 663.16,-238 663.16,-244 657.16,-250 651.16,-250 651.16,-250 94,-250 94,-250 88,-250 82,-244 82,-238 82,-238 82,-20 82,-20 82,-14 88,-8 94,-8"/><text text-anchor="middle" x="128" y="-230" font-family="Times,serif" font-size="20.00">Launch</text></g><g id="clust3" class="cluster"><title>cluster_spawn_holder</title><polygon fill="none" stroke="white" points="269.49,-16 269.49,-212 568.87,-212 568.87,-16 269.49,-16"/></g><g id="clust4" class="cluster"><title>cluster_launch_spawn</title><polygon fill="grey" stroke="grey" points="277.49,-68 277.49,-204 560.87,-204 560.87,-68 277.49,-68"/><text text-anchor="middle" x="346.49" y="-184" font-family="Times,serif" font-size="20.00">child.spawn</text></g><g id="clust5" class="cluster"><title>cluster_launch_outputs</title><polygon fill="none" stroke="white" points="683.16,-14 683.16,-174 753.16,-174 753.16,-14 683.16,-14"/></g><!-- launch_pull --><g id="node1" class="node"><title>launch_pull</title><ellipse fill="none" stroke="black" cx="118.6" cy="-122" rx="28.7" ry="18"/><text text-anchor="middle" x="118.6" y="-118.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- launch_stdin_recv_push --><g id="node2" class="node"><title>launch_stdin_recv_push</title><ellipse fill="none" stroke="black" cx="216.34" cy="-122" rx="33.29" ry="18"/><text text-anchor="middle" x="216.34" y="-118.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- launch_pull&#45;&gt;launch_stdin_recv_push --><g id="edge2" class="edge"><title>launch_pull->launch_stdin_recv_push</title><path fill="none" stroke="black" d="M147.29,-122C155.31,-122 164.25,-122 172.95,-122"/><polygon fill="black" stroke="black" points="172.98,-125.5 182.98,-122 172.98,-118.5 172.98,-125.5"/></g><!-- launch_stdin_recv_pull --><g id="node3" class="node"><title>launch_stdin_recv_pull</title><ellipse fill="none" stroke="black" cx="340.08" cy="-122" rx="54.69" ry="18"/><text text-anchor="middle" x="340.08" y="-118.3" font-family="Times,serif" font-size="14.00">pull:stdin</text></g><!-- launch_stdin_recv_push&#45;&gt;launch_stdin_recv_pull --><g id="edge3" class="edge"><title>launch_stdin_recv_push->launch_stdin_recv_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M249.65,-122C257.63,-122 266.46,-122 275.38,-122"/><polygon fill="black" stroke="black" points="275.43,-125.5 285.43,-122 275.43,-118.5 275.43,-125.5"/></g><!-- launch_stdout_send_pull --><g id="node4" class="node"><title>launch_stdout_send_pull</title><ellipse fill="none" stroke="black" cx="491.77" cy="-94" rx="61.19" ry="18"/><text text-anchor="middle" x="491.77" y="-90.3" font-family="Times,serif" font-size="14.00">pull:stdout</text></g><!-- launch_stdin_recv_pull&#45;&gt;launch_stdout_send_pull --><g id="edge4" class="edge"><title>launch_stdin_recv_pull->launch_stdout_send_pull</title><path fill="none" stroke="black" stroke-dasharray="1,5" d="M388.04,-113.22C404.5,-110.14 423.06,-106.67 439.83,-103.53"/></g><!-- launch_stderr_send_pull --><g id="node5" class="node"><title>launch_stderr_send_pull</title><ellipse fill="none" stroke="black" cx="491.77" cy="-148" rx="61.19" ry="18"/><text text-anchor="middle" x="491.77" y="-144.3" font-family="Times,serif" font-size="14.00">pull:stdout</text></g><!-- launch_stdin_recv_pull&#45;&gt;launch_stderr_send_pull --><g id="edge5" class="edge"><title>launch_stdin_recv_pull->launch_stderr_send_pull</title><path fill="none" stroke="black" stroke-dasharray="1,5" d="M388.86,-130.3C404.77,-133.06 422.57,-136.15 438.79,-138.97"/></g><!-- launch_stdout_recv_push --><g id="node8" class="node"><title>launch_stdout_recv_push</title><ellipse fill="none" stroke="black" cx="622.02" cy="-94" rx="33.29" ry="18"/><text text-anchor="middle" x="622.02" y="-90.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- launch_stdout_send_pull&#45;&gt;launch_stdout_recv_push --><g id="edge6" class="edge"><title>launch_stdout_send_pull->launch_stdout_recv_push</title><path fill="none" stroke="black" d="M553.14,-94C561.63,-94 570.21,-94 578.28,-94"/><polygon fill="black" stroke="black" points="578.55,-97.5 588.55,-94 578.55,-90.5 578.55,-97.5"/></g><!-- launch_stderr_recv_push --><g id="node7" class="node"><title>launch_stderr_recv_push</title><ellipse fill="none" stroke="black" cx="622.02" cy="-148" rx="33.29" ry="18"/><text text-anchor="middle" x="622.02" y="-144.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- launch_stderr_send_pull&#45;&gt;launch_stderr_recv_push --><g id="edge7" class="edge"><title>launch_stderr_send_pull->launch_stderr_recv_push</title><path fill="none" stroke="black" d="M553.14,-148C561.63,-148 570.21,-148 578.28,-148"/><polygon fill="black" stroke="black" points="578.55,-151.5 588.55,-148 578.55,-144.5 578.55,-151.5"/></g><!-- launch_exit_send_push --><g id="node6" class="node"><title>launch_exit_send_push</title><ellipse fill="none" stroke="black" cx="340.08" cy="-42" rx="53.89" ry="18"/><text text-anchor="middle" x="340.08" y="-38.3" font-family="Times,serif" font-size="14.00">push:exit</text></g><!-- launch_exit_outer --><g id="node12" class="node"><title>launch_exit_outer</title></g><!-- launch_exit_send_push&#45;&gt;launch_exit_outer --><g id="edge10" class="edge"><title>launch_exit_send_push->launch_exit_outer</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M394.17,-41.72C471.81,-41.3 614.34,-40.55 681,-40.19"/><polygon fill="black" stroke="black" points="681.05,-43.69 691.03,-40.14 681.01,-36.69 681.05,-43.69"/></g><!-- launch_stderr_outer --><g id="node11" class="node"><title>launch_stderr_outer</title></g><!-- launch_stderr_recv_push&#45;&gt;launch_stderr_outer --><g id="edge9" class="edge"><title>launch_stderr_recv_push->launch_stderr_outer</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M655.42,-148C663.57,-148 672.37,-148 680.71,-148"/><polygon fill="black" stroke="black" points="680.94,-151.5 690.94,-148 680.94,-144.5 680.94,-151.5"/></g><!-- launch_stdout_outer --><g id="node10" class="node"><title>launch_stdout_outer</title></g><!-- launch_stdout_recv_push&#45;&gt;launch_stdout_outer --><g id="edge8" class="edge"><title>launch_stdout_recv_push->launch_stdout_outer</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M655.42,-94C663.57,-94 672.37,-94 680.71,-94"/><polygon fill="black" stroke="black" points="680.94,-97.5 690.94,-94 680.94,-90.5 680.94,-97.5"/></g><!-- launch_input --><g id="node9" class="node"><title>launch_input</title></g><!-- launch_input&#45;&gt;launch_pull --><g id="edge1" class="edge"><title>launch_input->launch_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.4,-122C62.32,-122 71.18,-122 79.7,-122"/><polygon fill="black" stroke="black" points="79.83,-125.5 89.83,-122 79.83,-118.5 79.83,-125.5"/></g></g></svg>

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

*   **cmd**: The command to run
*   **path**: Where to run it
*   **env**: The environmentl variables to run it in
*   **arg**: arguments that will be passed through to the the command

### Component: Drain

<svg width="379pt" height="106pt" viewBox="0.00 0.00 378.78 106.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 102)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-102 374.78,-102 374.78,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_drain</title><path fill="none" stroke="lightgrey" d="M94,-8C94,-8 350.78,-8 350.78,-8 356.78,-8 362.78,-14 362.78,-20 362.78,-20 362.78,-78 362.78,-78 362.78,-84 356.78,-90 350.78,-90 350.78,-90 94,-90 94,-90 88,-90 82,-84 82,-78 82,-78 82,-20 82,-20 82,-14 88,-8 94,-8"/><text text-anchor="middle" x="119" y="-70" font-family="Times,serif" font-size="20.00">Drain</text></g><!-- drain_input --><g id="node1" class="node"><title>drain_input</title></g><!-- drain_pull --><g id="node2" class="node"><title>drain_pull</title><ellipse fill="none" stroke="black" cx="118.6" cy="-34" rx="28.7" ry="18"/><text text-anchor="middle" x="118.6" y="-30.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- drain_input&#45;&gt;drain_pull --><g id="edge1" class="edge"><title>drain_input->drain_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.4,-34C62.32,-34 71.18,-34 79.7,-34"/><polygon fill="black" stroke="black" points="79.83,-37.5 89.83,-34 79.83,-30.5 79.83,-37.5"/></g><!-- drain_push --><g id="node3" class="node"><title>drain_push</title><ellipse fill="none" stroke="black" cx="268.99" cy="-34" rx="85.59" ry="18"/><text text-anchor="middle" x="268.99" y="-30.3" font-family="Times,serif" font-size="14.00">push: file/stdout</text></g><!-- drain_pull&#45;&gt;drain_push --><g id="edge2" class="edge"><title>drain_pull->drain_push</title><path fill="none" stroke="black" d="M147.51,-34C155.09,-34 163.75,-34 172.87,-34"/><polygon fill="black" stroke="black" points="172.91,-37.5 182.91,-34 172.91,-30.5 172.91,-37.5"/></g></g></svg>

```yaml
drain:
  output:
    destination: '-'
```

This is the normal way to get output from Pipeawesome. the output can be sent to "`-`" for STDOUT, "`_`" for STDERR, anything else is taken as a filename where data will be wrote to.

### Component: Junction

(a brief interlude)

<svg width="347pt" height="160pt" viewBox="0.00 0.00 347.49 160.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 156)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-156 343.49,-156 343.49,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_junction</title><path fill="none" stroke="lightgrey" d="M94,-8C94,-8 245.49,-8 245.49,-8 251.49,-8 257.49,-14 257.49,-20 257.49,-20 257.49,-132 257.49,-132 257.49,-138 251.49,-144 245.49,-144 245.49,-144 94,-144 94,-144 88,-144 82,-138 82,-132 82,-132 82,-20 82,-20 82,-14 88,-8 94,-8"/><text text-anchor="middle" x="132.5" y="-124" font-family="Times,serif" font-size="20.00">Junction</text></g><!-- junction_push_1 --><g id="node1" class="node"><title>junction_push\_1</title><ellipse fill="none" stroke="black" cx="216.34" cy="-88" rx="33.29" ry="18"/><text text-anchor="middle" x="216.34" y="-84.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- junction_exit_pull_1 --><g id="node7" class="node"><title>junction_exit_pull\_1</title></g><!-- junction_push_1&#45;&gt;junction_exit_pull_1 --><g id="edge7" class="edge"><title>junction_push\_1->junction_exit_pull\_1</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M249.74,-88C257.9,-88 266.7,-88 275.04,-88"/><polygon fill="black" stroke="black" points="275.26,-91.5 285.26,-88 275.26,-84.5 275.26,-91.5"/></g><!-- junction_push_2 --><g id="node2" class="node"><title>junction_push\_2</title><ellipse fill="none" stroke="black" cx="216.34" cy="-34" rx="33.29" ry="18"/><text text-anchor="middle" x="216.34" y="-30.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- junction_exit_pull_2 --><g id="node8" class="node"><title>junction_exit_pull\_2</title></g><!-- junction_push_2&#45;&gt;junction_exit_pull_2 --><g id="edge8" class="edge"><title>junction_push\_2->junction_exit_pull\_2</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M249.74,-34C257.9,-34 266.7,-34 275.04,-34"/><polygon fill="black" stroke="black" points="275.26,-37.5 285.26,-34 275.26,-30.5 275.26,-37.5"/></g><!-- junction_pull_1 --><g id="node3" class="node"><title>junction_pull\_1</title><ellipse fill="none" stroke="black" cx="118.6" cy="-88" rx="28.7" ry="18"/><text text-anchor="middle" x="118.6" y="-84.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- junction_pull_1&#45;&gt;junction_push_1 --><g id="edge3" class="edge"><title>junction_pull\_1->junction_push\_1</title><path fill="none" stroke="black" d="M147.29,-88C155.31,-88 164.25,-88 172.95,-88"/><polygon fill="black" stroke="black" points="172.98,-91.5 182.98,-88 172.98,-84.5 172.98,-91.5"/></g><!-- junction_pull_1&#45;&gt;junction_push_2 --><g id="edge5" class="edge"><title>junction_pull\_1->junction_push\_2</title><path fill="none" stroke="black" d="M140.37,-76.28C153.01,-69.16 169.39,-59.92 183.62,-51.89"/><polygon fill="black" stroke="black" points="185.61,-54.79 192.6,-46.83 182.17,-48.69 185.61,-54.79"/></g><!-- junction_pull_2 --><g id="node4" class="node"><title>junction_pull\_2</title><ellipse fill="none" stroke="black" cx="118.6" cy="-34" rx="28.7" ry="18"/><text text-anchor="middle" x="118.6" y="-30.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- junction_pull_2&#45;&gt;junction_push_1 --><g id="edge4" class="edge"><title>junction_pull\_2->junction_push\_1</title><path fill="none" stroke="black" d="M140.37,-45.72C153.01,-52.84 169.39,-62.08 183.62,-70.11"/><polygon fill="black" stroke="black" points="182.17,-73.31 192.6,-75.17 185.61,-67.21 182.17,-73.31"/></g><!-- junction_pull_2&#45;&gt;junction_push_2 --><g id="edge6" class="edge"><title>junction_pull\_2->junction_push\_2</title><path fill="none" stroke="black" d="M147.29,-34C155.31,-34 164.25,-34 172.95,-34"/><polygon fill="black" stroke="black" points="172.98,-37.5 182.98,-34 172.98,-30.5 172.98,-37.5"/></g><!-- junction_input_outer_1 --><g id="node5" class="node"><title>junction_input_outer\_1</title></g><!-- junction_input_outer_1&#45;&gt;junction_pull_1 --><g id="edge1" class="edge"><title>junction_input_outer\_1->junction_pull\_1</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.4,-88C62.32,-88 71.18,-88 79.7,-88"/><polygon fill="black" stroke="black" points="79.83,-91.5 89.83,-88 79.83,-84.5 79.83,-91.5"/></g><!-- junction_input_outer_2 --><g id="node6" class="node"><title>junction_input_outer\_2</title></g><!-- junction_input_outer_2&#45;&gt;junction_pull_2 --><g id="edge2" class="edge"><title>junction_input_outer\_2->junction_pull\_2</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.4,-34C62.32,-34 71.18,-34 79.7,-34"/><polygon fill="black" stroke="black" points="79.83,-37.5 89.83,-34 79.83,-30.5 79.83,-37.5"/></g></g></svg>

A **Junction** is a many-to-many connector. Anything that comes into one of it's inputs will be sent to all of it's outputs.

There's no configuration for **Junction** but it is the only component that has any reason to respect input priorities.

<sub>\*\*NOTE: Messages are considered to be seperated by Windows or UNIX line endings. This may become configurable in future.</sub>

### Components - Buffer & Regulator

<svg width="543pt" height="106pt" viewBox="0.00 0.00 542.98 106.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 102)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-102 538.98,-102 538.98,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_buffer</title><path fill="none" stroke="lightgrey" d="M94,-8C94,-8 440.98,-8 440.98,-8 446.98,-8 452.98,-14 452.98,-20 452.98,-20 452.98,-78 452.98,-78 452.98,-84 446.98,-90 440.98,-90 440.98,-90 94,-90 94,-90 88,-90 82,-84 82,-78 82,-78 82,-20 82,-20 82,-14 88,-8 94,-8"/><text text-anchor="middle" x="122" y="-70" font-family="Times,serif" font-size="20.00">Buffer</text></g><!-- buffer_input_push --><g id="node1" class="node"><title>buffer_input_push</title></g><!-- buffer_pull --><g id="node2" class="node"><title>buffer_pull</title><ellipse fill="none" stroke="black" cx="118.6" cy="-34" rx="28.7" ry="18"/><text text-anchor="middle" x="118.6" y="-30.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- buffer_input_push&#45;&gt;buffer_pull --><g id="edge3" class="edge"><title>buffer_input_push->buffer_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.4,-34C62.32,-34 71.18,-34 79.7,-34"/><polygon fill="black" stroke="black" points="79.83,-37.5 89.83,-34 79.83,-30.5 79.83,-37.5"/></g><!-- buffer_push --><g id="node3" class="node"><title>buffer_push</title><ellipse fill="none" stroke="black" cx="216.34" cy="-34" rx="33.29" ry="18"/><text text-anchor="middle" x="216.34" y="-30.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- buffer_pull&#45;&gt;buffer_push --><g id="edge4" class="edge"><title>buffer_pull->buffer_push</title><path fill="none" stroke="black" d="M147.29,-34C155.31,-34 164.25,-34 172.95,-34"/><polygon fill="black" stroke="black" points="172.98,-37.5 182.98,-34 172.98,-30.5 172.98,-37.5"/></g><!-- buffer_inner_pull --><g id="node4" class="node"><title>buffer_inner_pull</title><ellipse fill="none" stroke="black" cx="314.09" cy="-34" rx="28.7" ry="18"/><text text-anchor="middle" x="314.09" y="-30.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- buffer_push&#45;&gt;buffer_inner_pull --><g id="edge1" class="edge"><title>buffer_push->buffer_inner_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M249.49,-34C257.76,-34 266.72,-34 275.25,-34"/><polygon fill="none" stroke="black" points="275.35,-37.5 285.35,-34 275.35,-30.5 275.35,-37.5"/></g><!-- buffer_inner_push --><g id="node5" class="node"><title>buffer_inner_push</title><ellipse fill="none" stroke="black" cx="411.83" cy="-34" rx="33.29" ry="18"/><text text-anchor="middle" x="411.83" y="-30.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- buffer_inner_pull&#45;&gt;buffer_inner_push --><g id="edge2" class="edge"><title>buffer_inner_pull->buffer_inner_push</title><path fill="none" stroke="black" d="M342.78,-34C350.8,-34 359.74,-34 368.44,-34"/><polygon fill="black" stroke="black" points="368.47,-37.5 378.47,-34 368.47,-30.5 368.47,-37.5"/></g><!-- buffer_exit_pull --><g id="node6" class="node"><title>buffer_exit_pull</title></g><!-- buffer_inner_push&#45;&gt;buffer_exit_pull --><g id="edge5" class="edge"><title>buffer_inner_push->buffer_exit_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M445.23,-34C453.38,-34 462.19,-34 470.53,-34"/><polygon fill="black" stroke="black" points="470.75,-37.5 480.75,-34 470.75,-30.5 470.75,-37.5"/></g></g></svg>

<svg width="568pt" height="146pt" viewBox="0.00 0.00 567.89 146.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 142)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-142 563.89,-142 563.89,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_regulator</title><path fill="none" stroke="lightgrey" d="M184.4,-48C184.4,-48 465.89,-48 465.89,-48 471.89,-48 477.89,-54 477.89,-60 477.89,-60 477.89,-118 477.89,-118 477.89,-124 471.89,-130 465.89,-130 465.89,-130 184.4,-130 184.4,-130 178.4,-130 172.4,-124 172.4,-118 172.4,-118 172.4,-60 172.4,-60 172.4,-54 178.4,-48 184.4,-48"/><text text-anchor="middle" x="231.4" y="-110" font-family="Times,serif" font-size="20.00">Regulator</text></g><!-- regulator_input_push --><g id="node1" class="node"><title>regulator_input_push</title></g><!-- regulator_pull --><g id="node2" class="node"><title>regulator_pull</title><ellipse fill="none" stroke="black" cx="209" cy="-74" rx="28.7" ry="18"/><text text-anchor="middle" x="209" y="-70.3" font-family="Times,serif" font-size="14.00">pull</text></g><!-- regulator_input_push&#45;&gt;regulator_pull --><g id="edge1" class="edge"><title>regulator_input_push->regulator_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M54.17,-74C84.66,-74 135.27,-74 170.07,-74"/><polygon fill="black" stroke="black" points="170.32,-77.5 180.32,-74 170.32,-70.5 170.32,-77.5"/></g><!-- regulator_note_point --><g id="node4" class="node"><title>regulator_note_point</title><ellipse fill="black" stroke="black" cx="365.8" cy="-74" rx="1.8" ry="1.8"/></g><!-- regulator_pull&#45;&gt;regulator_note_point --><g id="edge2" class="edge"><title>regulator_pull->regulator_note_point</title><path fill="none" stroke="black" d="M237.68,-74C279.42,-74 354.62,-74 363.86,-74"/></g><!-- regulator_push --><g id="node3" class="node"><title>regulator_push</title><ellipse fill="none" stroke="black" cx="436.75" cy="-74" rx="33.29" ry="18"/><text text-anchor="middle" x="436.75" y="-70.3" font-family="Times,serif" font-size="14.00">push</text></g><!-- regulator_exit_pull --><g id="node5" class="node"><title>regulator_exit_pull</title></g><!-- regulator_push&#45;&gt;regulator_exit_pull --><g id="edge4" class="edge"><title>regulator_push->regulator_exit_pull</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M470.15,-74C478.3,-74 487.11,-74 495.44,-74"/><polygon fill="black" stroke="black" points="495.67,-77.5 505.67,-74 495.67,-70.5 495.67,-77.5"/></g><!-- regulator_note_point&#45;&gt;regulator_push --><g id="edge3" class="edge"><title>regulator_note_point->regulator_push</title><path fill="none" stroke="black" d="M367.77,-74C370.95,-74 381.45,-74 393.28,-74"/><polygon fill="black" stroke="black" points="393.41,-77.5 403.41,-74 393.41,-70.5 393.41,-77.5"/></g><!-- regulator_note --><g id="node6" class="node"><title>regulator_note</title><polygon fill="#ffec8b" stroke="black" points="322,-38 90,-38 90,0 328,0 328,-32 322,-38"/><polyline fill="none" stroke="black" points="322,-38 322,-32 "/><polyline fill="none" stroke="black" points="328,-32 322,-32 "/><text text-anchor="middle" x="209" y="-22.8" font-family="Times,serif" font-size="14.00">Pipeawesome controls the flow</text><text text-anchor="middle" x="209" y="-7.8" font-family="Times,serif" font-size="14.00">between the pull and the push</text></g><!-- regulator_note&#45;&gt;regulator_note_point --><g id="edge5" class="edge"><title>regulator_note->regulator_note_point</title><path fill="none" stroke="black" stroke-width="0.3" d="M313.91,-38.04C318.75,-39.85 323.47,-41.83 328,-44 344.65,-51.97 360.34,-68.96 364,-73.08"/></g></g></svg>

The only components I have not used in the tic-tac-toe example are the **Buffer** and **Regulator**.

Looking back at some of the tic-tac-toe configurations after [Multiple Turns](#multiple-turns) you can see a message is often sent to near the start of the process from very close to the end of the process. If there are a finite number of messages coming into the system, as in a single game of tic-tac-toe, this is not a problem but if more and more messages are being added to the system and all the old messages are being processed, you can see that may be an issue. This situation is unique to loops.

The connectors between components have finite capacity ([they are async version of bounded Rust channels](https://docs.rs/async-std/1.9.0/async_std/channel/fn.bounded.html)), which is pretty much required for back pressure to work, but it does leave a form of deadlock being possible in configurations that include loops.

Below I describe the two components that can be used to control this situation:

*   A **Buffer** is connector with infinite message capacity (it is an [unbounded Rust channel](https://docs.rs/async-std/1.9.0/async_std/channel/fn.unbounded.html)).
*   The **Regulator** can turn on and off the flow of messages passing through it depending on how many messages are in a configured set of Buffers.

The configuration below is a version of the tic-tac-toe configuration above, but modified to run 100,000 games:

```yaml file=./examples/tic-tac-toe/many_games.pa.yaml
connection:
  random_selection: "l:random_player | regulator:regulate_flow | j:turn"
  player_o_branch: "j:turn | l:player_o_filter | l:player_o | j:draw"
  player_x_branch: "j:turn | l:player_x_filter | l:player_x | j:draw"
  last_draw: "j:draw | l:referee | j:loop | l:only_finishes | l:draw | d:output"
  looper: "j:loop | l:turn_swapper | buffer:reprocess | j:turn"
drain:
  output: { destination: '-' }
regulator:
  reg:
    buffered: [10, 100]
    monitored_buffers: [ "reprocess" ]
launch:
  random_player:
    cmd: "bash"
    arg:
      - '-c'
      - 'for i in {1..100000}; do echo $((RANDOM % 2))::::::::: | sed "s/1/X/" | sed "s/0/O/"; done '
  player_o_filter: { cmd: "grep", arg: [ "--line-buffered", "^O" ] }
  player_o:
    cmd: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=O', '-f', 'examples/tic-tac-toe/player.awk' ]
  player_x_filter: { cmd: "grep", arg: [ "--line-buffered", "^X" ] }
  player_x:
    cmd: "gawk"
    arg: [ '-F', ':', '-v', 'PLAYER=X', '-f', 'examples/tic-tac-toe/player.awk' ]
  referee:
    cmd: "gawk"
    arg: ['-F', ':', '-v', 'DESIRED_GAME_COUNT=100000', '-f', './examples/tic-tac-toe/referee.awk', 'NF=10', 'OFS=:']
  draw:
    cmd: "gawk"
    arg: [ '-F', ':', '-f', 'examples/tic-tac-toe/draw.awk' ]
  only_finishes:
    cmd: "grep"
    arg:
      - "--line-buffered"
      - "^[DW]"
  turn_swapper:
    cmd: "sed"
    arg:
      - "--unbuffered"
      - |
        s/^O/9/
        s/^X/O/
        s/^9/X/
```

Which could be visualized as:

<svg width="715pt" height="365pt" viewBox="0.00 0.00 714.97 365.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 361)"><title>g_get_graph</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-361 710.97,-361 710.97,4 -4,4"/><g id="clust2" class="cluster"><title>cluster_nodes_looper</title><polygon fill="none" stroke="black" points="29.97,-155 29.97,-230 317.97,-230 317.97,-155 29.97,-155"/><text text-anchor="middle" x="62.97" y="-214.8" font-family="Times,serif" font-size="14.00">looper:</text></g><g id="clust3" class="cluster"><title>cluster_nodes_player_o_branch</title><polygon fill="none" stroke="black" points="158.97,-8 158.97,-83 388.97,-83 388.97,-8 158.97,-8"/><text text-anchor="middle" x="227.47" y="-67.8" font-family="Times,serif" font-size="14.00">player_o_branch:</text></g><g id="clust4" class="cluster"><title>cluster_nodes_player_x_branch</title><polygon fill="none" stroke="black" points="468.97,-8 468.97,-83 698.97,-83 698.97,-8 468.97,-8"/><text text-anchor="middle" x="537.47" y="-67.8" font-family="Times,serif" font-size="14.00">player_x_branch:</text></g><g id="clust5" class="cluster"><title>cluster_nodes_random_selection</title><polygon fill="none" stroke="black" points="325.97,-155 325.97,-230 664.97,-230 664.97,-155 325.97,-155"/><text text-anchor="middle" x="398.97" y="-214.8" font-family="Times,serif" font-size="14.00">random_selection:</text></g><g id="clust6" class="cluster"><title>cluster_nodes_last_draw</title><polygon fill="none" stroke="black" points="29.97,-238 29.97,-313 527.97,-313 527.97,-238 29.97,-238"/><text text-anchor="middle" x="74.97" y="-297.8" font-family="Times,serif" font-size="14.00">last_draw:</text></g><!-- j_draw --><g id="node1" class="node"><title>j_draw</title><ellipse fill="papayawhip" stroke="black" cx="71.97" cy="-339" rx="34.39" ry="18"/><text text-anchor="middle" x="71.97" y="-335.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- l_referee --><g id="node12" class="node"><title>l_referee</title><polygon fill="lightblue" stroke="black" points="105.97,-282 37.97,-282 37.97,-246 105.97,-246 105.97,-282"/><text text-anchor="middle" x="71.97" y="-260.3" font-family="Times,serif" font-size="14.00">referee</text></g><!-- j_draw&#45;&gt;l_referee --><g id="edge1" class="edge"><title>j_draw->l_referee</title><path fill="none" stroke="black" d="M71.97,-320.7C71.97,-312.25 71.97,-301.87 71.97,-292.37"/><polygon fill="black" stroke="black" points="75.47,-292.18 71.97,-282.18 68.47,-292.18 75.47,-292.18"/></g><!-- j_turn --><g id="node2" class="node"><title>j_turn</title><ellipse fill="papayawhip" stroke="black" cx="345.97" cy="-109" rx="30.59" ry="18"/><text text-anchor="middle" x="345.97" y="-105.3" font-family="Times,serif" font-size="14.00">turn</text></g><!-- l_player_o_filter --><g id="node5" class="node"><title>l_player_o_filter</title><polygon fill="lightblue" stroke="black" points="380.97,-52 262.97,-52 262.97,-16 380.97,-16 380.97,-52"/><text text-anchor="middle" x="321.97" y="-30.3" font-family="Times,serif" font-size="14.00">player_o_filter</text></g><!-- j_turn&#45;&gt;l_player_o_filter --><g id="edge9" class="edge"><title>j_turn->l_player_o_filter</title><path fill="none" stroke="black" d="M340.4,-91.07C337.55,-82.38 334,-71.59 330.78,-61.8"/><polygon fill="black" stroke="black" points="334.04,-60.5 327.59,-52.09 327.39,-62.69 334.04,-60.5"/></g><!-- l_player_x_filter --><g id="node7" class="node"><title>l_player_x_filter</title><polygon fill="lightblue" stroke="black" points="594.97,-52 476.97,-52 476.97,-16 594.97,-16 594.97,-52"/><text text-anchor="middle" x="535.97" y="-30.3" font-family="Times,serif" font-size="14.00">player_x_filter</text></g><!-- j_turn&#45;&gt;l_player_x_filter --><g id="edge12" class="edge"><title>j_turn->l_player_x_filter</title><path fill="none" stroke="black" d="M370.81,-98.46C398.88,-87.67 445.42,-69.79 481.95,-55.76"/><polygon fill="black" stroke="black" points="483.41,-58.95 491.48,-52.09 480.89,-52.41 483.41,-58.95"/></g><!-- l_turn_swapper --><g id="node3" class="node"><title>l_turn_swapper</title><polygon fill="lightblue" stroke="black" points="153.47,-199 38.47,-199 38.47,-163 153.47,-163 153.47,-199"/><text text-anchor="middle" x="95.97" y="-177.3" font-family="Times,serif" font-size="14.00">turn_swapper</text></g><!-- b_reprocess --><g id="node4" class="node"><title>b_reprocess</title><polygon fill="lightgray" stroke="black" points="171.79,-175.44 240.97,-163 310.15,-175.44 310.09,-195.56 171.86,-195.56 171.79,-175.44"/><text text-anchor="middle" x="240.97" y="-177.3" font-family="Times,serif" font-size="14.00">reprocess</text></g><!-- l_turn_swapper&#45;&gt;b_reprocess --><g id="edge7" class="edge"><title>l_turn_swapper->b_reprocess</title><path fill="none" stroke="black" d="M153.74,-181C156.38,-181 159.01,-181 161.65,-181"/><polygon fill="black" stroke="black" points="161.8,-184.5 171.8,-181 161.8,-177.5 161.8,-184.5"/></g><!-- b_reprocess&#45;&gt;j_turn --><g id="edge8" class="edge"><title>b_reprocess->j_turn</title><path fill="none" stroke="black" d="M261.22,-166.5C277.4,-155.71 300.26,-140.47 318.16,-128.54"/><polygon fill="black" stroke="black" points="320.12,-131.44 326.5,-122.98 316.24,-125.62 320.12,-131.44"/></g><!-- l_player_o --><g id="node6" class="node"><title>l_player_o</title><polygon fill="lightblue" stroke="black" points="244.47,-52 167.47,-52 167.47,-16 244.47,-16 244.47,-52"/><text text-anchor="middle" x="205.97" y="-30.3" font-family="Times,serif" font-size="14.00">player_o</text></g><!-- l_player_o_filter&#45;&gt;l_player_o --><g id="edge10" class="edge"><title>l_player_o_filter->l_player_o</title><path fill="none" stroke="black" d="M262.95,-34C260.16,-34 257.37,-34 254.58,-34"/><polygon fill="black" stroke="black" points="254.49,-30.5 244.49,-34 254.49,-37.5 254.49,-30.5"/></g><!-- l_player_o&#45;&gt;j_draw --><g id="edge11" class="edge"><title>l_player_o->j_draw</title><path fill="none" stroke="black" d="M167.34,-48.07C123.96,-65.07 55.74,-99.37 25.97,-155 -7.16,-216.91 -9.92,-252.64 25.97,-313 28.22,-316.78 31.26,-320.02 34.71,-322.8"/><polygon fill="black" stroke="black" points="33.06,-325.9 43.34,-328.46 36.9,-320.05 33.06,-325.9"/></g><!-- l_player_x --><g id="node8" class="node"><title>l_player_x</title><polygon fill="lightblue" stroke="black" points="690.47,-52 613.47,-52 613.47,-16 690.47,-16 690.47,-52"/><text text-anchor="middle" x="651.97" y="-30.3" font-family="Times,serif" font-size="14.00">player_x</text></g><!-- l_player_x_filter&#45;&gt;l_player_x --><g id="edge13" class="edge"><title>l_player_x_filter->l_player_x</title><path fill="none" stroke="black" d="M595.33,-34C597.82,-34 600.31,-34 602.8,-34"/><polygon fill="black" stroke="black" points="603.03,-37.5 613.03,-34 603.03,-30.5 603.03,-37.5"/></g><!-- l_player_x&#45;&gt;j_draw --><g id="edge14" class="edge"><title>l_player_x->j_draw</title><path fill="none" stroke="black" d="M659.5,-52C674.05,-87.64 701.83,-171.67 668.97,-230 634.03,-292.03 599.99,-292 531.97,-313 455.4,-336.65 216.81,-338.59 116.79,-338.32"/><polygon fill="black" stroke="black" points="116.72,-334.82 106.71,-338.28 116.7,-341.82 116.72,-334.82"/></g><!-- l_random_player --><g id="node9" class="node"><title>l_random_player</title><polygon fill="lightblue" stroke="black" points="457.47,-199 334.47,-199 334.47,-163 457.47,-163 457.47,-199"/><text text-anchor="middle" x="395.97" y="-177.3" font-family="Times,serif" font-size="14.00">random_player</text></g><!-- r_regulate_flow --><g id="node10" class="node"><title>r_regulate_flow</title><polygon fill="lightgray" stroke="black" points="656.53,-186.56 565.97,-199 475.41,-186.56 475.49,-166.44 656.45,-166.44 656.53,-186.56"/><text text-anchor="middle" x="565.97" y="-177.3" font-family="Times,serif" font-size="14.00">regulate_flow</text></g><!-- l_random_player&#45;&gt;r_regulate_flow --><g id="edge15" class="edge"><title>l_random_player->r_regulate_flow</title><path fill="none" stroke="black" d="M457.73,-181C460.17,-181 462.6,-181 465.04,-181"/><polygon fill="black" stroke="black" points="465.06,-184.5 475.06,-181 465.06,-177.5 465.06,-184.5"/></g><!-- r_regulate_flow&#45;&gt;j_turn --><g id="edge16" class="edge"><title>r_regulate_flow->j_turn</title><path fill="none" stroke="black" d="M523.29,-166.42C482.23,-153.35 420.82,-133.82 382.11,-121.5"/><polygon fill="black" stroke="black" points="382.9,-118.08 372.31,-118.38 380.77,-124.75 382.9,-118.08"/></g><!-- j_loop --><g id="node11" class="node"><title>j_loop</title><ellipse fill="papayawhip" stroke="black" cx="154.97" cy="-264" rx="30.59" ry="18"/><text text-anchor="middle" x="154.97" y="-260.3" font-family="Times,serif" font-size="14.00">loop</text></g><!-- j_loop&#45;&gt;l_turn_swapper --><g id="edge6" class="edge"><title>j_loop->l_turn_swapper</title><path fill="none" stroke="black" d="M143.31,-247C135.05,-235.65 123.79,-220.2 114.33,-207.2"/><polygon fill="black" stroke="black" points="117.14,-205.11 108.42,-199.09 111.48,-209.23 117.14,-205.11"/></g><!-- l_only_finishes --><g id="node13" class="node"><title>l_only_finishes</title><polygon fill="lightblue" stroke="black" points="311.97,-282 203.97,-282 203.97,-246 311.97,-246 311.97,-282"/><text text-anchor="middle" x="257.97" y="-260.3" font-family="Times,serif" font-size="14.00">only_finishes</text></g><!-- j_loop&#45;&gt;l_only_finishes --><g id="edge3" class="edge"><title>j_loop->l_only_finishes</title><path fill="none" stroke="black" d="M185.55,-264C188.33,-264 191.1,-264 193.88,-264"/><polygon fill="black" stroke="black" points="193.94,-267.5 203.94,-264 193.94,-260.5 193.94,-267.5"/></g><!-- l_referee&#45;&gt;j_loop --><g id="edge2" class="edge"><title>l_referee->j_loop</title><path fill="none" stroke="black" d="M106.01,-264C108.79,-264 111.56,-264 114.33,-264"/><polygon fill="black" stroke="black" points="114.37,-267.5 124.37,-264 114.37,-260.5 114.37,-267.5"/></g><!-- l_draw --><g id="node14" class="node"><title>l_draw</title><polygon fill="lightblue" stroke="black" points="383.97,-282 329.97,-282 329.97,-246 383.97,-246 383.97,-282"/><text text-anchor="middle" x="356.97" y="-260.3" font-family="Times,serif" font-size="14.00">draw</text></g><!-- l_only_finishes&#45;&gt;l_draw --><g id="edge4" class="edge"><title>l_only_finishes->l_draw</title><path fill="none" stroke="black" d="M312.11,-264C314.58,-264 317.04,-264 319.5,-264"/><polygon fill="black" stroke="black" points="319.64,-267.5 329.64,-264 319.64,-260.5 319.64,-267.5"/></g><!-- d_output --><g id="node15" class="node"><title>d_output</title><polygon fill="lightpink" stroke="black" points="426.72,-246 495.23,-246 519.62,-282 402.32,-282 426.72,-246"/><text text-anchor="middle" x="460.97" y="-260.3" font-family="Times,serif" font-size="14.00">output</text></g><!-- l_draw&#45;&gt;d_output --><g id="edge5" class="edge"><title>l_draw->d_output</title><path fill="none" stroke="black" d="M384.19,-264C390.82,-264 397.44,-264 404.07,-264"/><polygon fill="black" stroke="black" points="404.48,-267.5 414.48,-264 404.48,-260.5 404.48,-267.5"/></g></g></svg>

In this situation, the `l:random_player` is creating 100,000 messages instead of just 1, but every one of these will loop back around until the game is complete, so we will have a significant amount of messages. In this configuration, when there are more than 100 messages in `buffer:reprocess`, the `regulator:regulate_flow` will stop accepting messages, but when the amount of messages in the buffer drops below 10, it will resume. Using these two components can solve the problem of issue described above.

## Pipe Variations, Output Types and Input Priorities.

### Pipe Variations

When thinking about passing data between programs, when the recieving program dies or closes it's input, we eventually we end up with data coming out from the sending program, but with nowhere for it to go. In this situation I think we have three options:

1.  Terminate (T): Terminate Pipeawesome.
2.  Finish (F): Close the pipe - letting the sending program deal with the problem itself (this will likely cause a cascade effect).
3.  Consume (C): Pipeawesome will keep the pipe open by consuming the data itself (bug discarding it).

You can specify the the pipe variation using:

*   `l:sender |T| l:reciever` - Terminate.
*   `l:sender |F| l:reciever` - Finish.
*   `l:sender |C| l:reciever` - Consume.

The normal, single pipe symbol (`l:sender | l:reciever`) is merely a quick way of writing "`|T|`".

### Output Types

A running program may not output all it's output to STDOUT, it can also send data to STDERR.

Pipeawesome allows you to capture what programs output to STDOUT and STDERR but also the EXIT code when the program finishes. This is done by using `[O]`, `[E]` and `[X]` just after the component in the connection set, for example:

```yaml file=./examples/ls/pa.yaml
connection:
  ls_stdout: "l:ls[O] | l:awk_stdout | j:out"
  ls_stderr: "l:ls[E] | l:awk_stderr | j:out"
  ls_exit: "l:ls[X] | l:awk_exit | j:out"
  out: "j:out | d:out"
drain:
  out: { destination: '-' }
launch:
  awk_stdout:
    cmd: awk
    arg: ['{ print "STDOUT: " $0 }']
  awk_stderr:
    cmd: awk
    arg: ['{ print "STDERR: " $0 }']
  awk_exit:
    cmd: awk
    arg: ['{ print "EXIT: " $0 }']
  ls:
    cmd: ls
    arg:
    - "."
    - "i_should_not_exist"
```

NOTE: Infact on UNIX programs can also read & write to `/dev/fdN` where N can be 0 (STDIN), 1 (STDOUT), 2 (STDERR) or other values of N. These other values of N are not currently directly supported.

### Input Priorities

All components except Junction only have one input, but a Junction can have multiple. To control which input to read we can add priorities, these are specified like the following:

```yaml
connection:
  high_priority: "launch:one_thing | [5]junction:many_to_many"
  low_priority: "launch:something_else | [1]junction:many_to_many"
```

In this example 5 and 1 are priorities, when priorities are not specified, they will be 0. Priorities can also be negative.

## Appendix

### Component Diagram Legend

This is the legend for diagrams shown in the [Component Types](#component-types) section.

<svg width="279pt" height="210pt" viewBox="0.00 0.00 279.00 210.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 206)"><title>G</title><polygon fill="white" stroke="transparent" points="-4,4 -4,-206 275,-206 275,4 -4,4"/><g id="clust1" class="cluster"><title>cluster_legend</title><polygon fill="none" stroke="black" points="8,-8 8,-194 263,-194 263,-8 8,-8"/><text text-anchor="middle" x="135.5" y="-178.8" font-family="Times,serif" font-size="14.00">Legend</text></g><!-- a --><g id="node1" class="node"><title>a</title><ellipse fill="none" stroke="black" cx="27" cy="-151" rx="11" ry="11"/></g><!-- b --><g id="node2" class="node"><title>b</title><ellipse fill="none" stroke="black" cx="244" cy="-151" rx="11" ry="11"/></g><!-- a&#45;&gt;b --><g id="edge1" class="edge"><title>a->b</title><path fill="none" stroke="black" d="M38.08,-151C71.3,-151 177.73,-151 222.67,-151"/><polygon fill="black" stroke="black" points="222.69,-154.5 232.69,-151 222.69,-147.5 222.69,-154.5"/><text text-anchor="middle" x="135.5" y="-154.8" font-family="Times,serif" font-size="14.00">motion</text></g><!-- c --><g id="node3" class="node"><title>c</title><ellipse fill="none" stroke="black" cx="27" cy="-111" rx="11" ry="11"/></g><!-- d --><g id="node4" class="node"><title>d</title><ellipse fill="none" stroke="black" cx="244" cy="-111" rx="11" ry="11"/></g><!-- c&#45;&gt;d --><g id="edge2" class="edge"><title>c->d</title><path fill="none" stroke="black" stroke-dasharray="1,5" d="M38.08,-111C74.11,-111 196.3,-111 232.69,-111"/><text text-anchor="middle" x="135.5" y="-114.8" font-family="Times,serif" font-size="14.00">io</text></g><!-- e --><g id="node5" class="node"><title>e</title><ellipse fill="none" stroke="black" cx="27" cy="-71" rx="11" ry="11"/></g><!-- f --><g id="node6" class="node"><title>f</title><ellipse fill="none" stroke="black" cx="244" cy="-71" rx="11" ry="11"/></g><!-- e&#45;&gt;f --><g id="edge3" class="edge"><title>e->f</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M38.08,-71C71.3,-71 177.73,-71 222.67,-71"/><polygon fill="black" stroke="black" points="222.69,-74.5 232.69,-71 222.69,-67.5 222.69,-74.5"/><text text-anchor="middle" x="135.5" y="-74.8" font-family="Times,serif" font-size="14.00">channel synchronous</text></g><!-- g --><g id="node7" class="node"><title>g</title><ellipse fill="none" stroke="black" cx="27" cy="-31" rx="11" ry="11"/></g><!-- h --><g id="node8" class="node"><title>h</title><ellipse fill="none" stroke="black" cx="244" cy="-31" rx="11" ry="11"/></g><!-- g&#45;&gt;h --><g id="edge4" class="edge"><title>g->h</title><path fill="none" stroke="black" stroke-dasharray="5,2" d="M38.08,-31C71.3,-31 177.73,-31 222.67,-31"/><polygon fill="none" stroke="black" points="222.69,-34.5 232.69,-31 222.69,-27.5 222.69,-34.5"/><text text-anchor="middle" x="135.5" y="-34.8" font-family="Times,serif" font-size="14.00">channel asynchronous</text></g></g></svg>

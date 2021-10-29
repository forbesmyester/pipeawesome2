
echo '{}' | \
./target/debug/pipeawesome2 config --config=- --config-out=- faucet     --id tap      source      -- res/test/simple_input.txt  | \
./target/debug/pipeawesome2 config --config=- --config-out=- faucet     --id tap      watermark   -- 5 10 | \
./target/debug/pipeawesome2 config --config=- --config-out=- drain      --id hole     destination -- tmp.output.txt | \
./target/debug/pipeawesome2 config --config=- --config-out=- connection --id only_one join        -- 'f:tap | l:cmd | d:hole' | \
./target/debug/pipeawesome2 config --config=- --config-out=- launch     --id cmd      command     -- grep | \
./target/debug/pipeawesome2 config --config=- --config-out=- launch     --id cmd      args        -- '--line-buffered' '-n' hello  | \
./target/debug/pipeawesome2 config --config=- --config-out=- launch     --id cmd      env         -- 'DEBUG=TRUE' 'OUT=ZZZ' | \
./target/debug/pipeawesome2 config --config=- --config-out=- launch     --id cmd      path        -- '/tmp'




echo '{}' | \
./target/debug/pipeawesome2 config --config=- --config-out=- faucet     --id tap      source      -- res/test/simple_input.txt  | \
./target/debug/pipeawesome2 config --config=- --config-out=- connection --id only_one join        -- 'f:f | l:l | d:hole' | \
./target/debug/pipeawesome2 config --config=- lint

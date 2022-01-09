#!/usr/bin/env bats

@test "pipeawesome simple" {

    EXPECTED="$( cat ./bats-tests/straight_through_pipe/expected.txt )"
    RESULT="$( ./target/debug/pipeawesome2 process --config ./bats-tests/straight_through_pipe/pa.yaml )"

    echo "RESULT = $RESULT"
    echo "EXPECTED = $EXPECTED"
    [ "$RESULT" = "$EXPECTED" ]
}


@test "pipeawesome lint" {

    run ./target/debug/pipeawesome2 config --config ./bats-tests/lint-errors/pa.json lint 2>&1 1>/dev/null
    [ "$status" -eq 1 ]
    RESULT="$( echo "$output" | awk -F ': ' '$3 ~ "^00" { X[$3]++ }; END { for (k in X) { print k ":" X[k] } }' )"
    EXPECTED=$( echo "0001:2;0002:1" | sed 's/;/\n/g' )

    echo "RESULT = $RESULT"
    echo "EXPECTED = $EXPECTED"
    [ "$RESULT" = "$EXPECTED" ]
}


@test "pipeawesome branches" {

    EXPECTED="$( cat ./bats-tests/branches/expected.txt )"
    RESULT="$( cat ./bats-tests/branches/input.txt | ./target/debug/pipeawesome2 process --config ./bats-tests/branches/pa.yaml )"

    echo "RESULT = $RESULT"
    echo "EXPECTED = $EXPECTED"
    [ "$RESULT" = "$EXPECTED" ]
}


@test "pipeawesome loops" {

    EXPECTED="$( cat ./bats-tests/loops/expected.txt )"
    RESULT="$( cat ./bats-tests/loops/input.txt | ./target/debug/pipeawesome2 process --config ./bats-tests/loops/pa.yaml )"

    echo "RESULT = $RESULT"
    echo "EXPECTED = $EXPECTED"
    [ "$RESULT" = "$EXPECTED" ]
}


@test "pipeawesome process-fail" {

    run ./target/debug/pipeawesome2 process --config ./bats-tests/lint-errors/pa.json
    [ "$status" -eq 1 ]
    RESULT="$( echo "$output" | awk -F ': ' '$3 ~ "^00" { X[$3]++ }; END { for (k in X) { print k ":" X[k] } }' )"
    EXPECTED=$( echo "0002:1" | sed 's/;/\n/g' )

    echo "RESULT = $RESULT"
    echo "EXPECTED = $EXPECTED"
    [ "$RESULT" = "$EXPECTED" ]
}




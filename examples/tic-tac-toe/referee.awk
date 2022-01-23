function won_diagonal() {
    if ((length($4) == 1) && ($4 == $6) && ($4 == $8)) { return $4 }
    if ((length($2) == 1) && ($2 == $6) && ($2 == $10)) { return $2 }
    return 0
}
function won_horizontal() {
    if ((length($2) == 1) && ($2 == $3) && ($2 == $4)) { return $2 }
    if ((length($5) == 1) && ($5 == $6) && ($5 == $7)) { return $5 }
    if ((length($8) == 1) && ($8 == $9) && ($8 == $10)) { return $8 }
    return 0
}
function won_vertical() {
    if ((length($2) == 1) && ($2 == $5) && ($2 == $8)) { return $2 }
    if ((length($3) == 1) && ($3 == $6) && ($3 == $9)) { return $3 }
    if ((length($4) == 1) && ($4 == $7) && ($4 == $10)) { return $4 }
    return 0
}
function full() {
    for (i=2; i<=NF; i++) {
        if (($i != "O") && ($i != "X")) { return 0 }
    }
    return 1
}
function mark_won(s) {
    $1=s;
    return $0
}
function close_game()
{
    FINISHED_GAME_COUNT = FINISHED_GAME_COUNT + 1
    if (FINISHED_GAME_COUNT >= DESIRED_COMPLETE_COUNT) {
        exit 0
    }
    fflush()
    next
}
BEGIN {
    FINISHED_GAME_COUNT = 0

    # These lines are here as DESIRED_GAME_COUNT is the passed in variable but
    # want the default number of games to be 0
    DESIRED_COMPLETE_COUNT = 1
    if (DESIRED_GAME_COUNT > 2) {
        DESIRED_COMPLETE_COUNT = DESIRED_GAME_COUNT
    }
}
{
    if (full()) { print mark_won("D"); close_game() }
    if (player = won_diagonal()) { print mark_won(sprintf("W%s", player)); close_game() } # Assignment in if condition is intended
    if (player = won_horizontal()) { print mark_won(sprintf("W%s", player)); close_game() } # Assignment in if condition is intended
    if (player = won_vertical()) { print mark_won(sprintf("W%s", player)); close_game() } # Assignment in if condition is intended
    print $0
    fflush()
}

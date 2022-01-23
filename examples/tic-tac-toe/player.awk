BEGIN { srand(); OFS=":"; NF=10 }
function find_positions(ar) {
    for (i=2;i<=NF;i++) {
        if ($i == "") {
            ar[length(ar)] = i
        }
    }
}
function initialize_array(ar) {
    ar[0] = 1;
    for (i = length(ar) - 1; i >= 0; i--) {
        delete ar[i]
    }
}
{
    initialize_array(positions)
    find_positions(positions)
    position = positions[int(rand() * length(positions))]
    $position = PLAYER
    print $0
    fflush()
}


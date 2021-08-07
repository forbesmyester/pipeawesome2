BEGIN {
    if (mod == "") {
        mod = 2
    }
    if (rem == "") {
        rem = 1
    }
}
{
    if ((NR % mod) == rem) {
        print $0
    }
}

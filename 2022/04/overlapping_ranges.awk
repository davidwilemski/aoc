BEGIN {
    fullOverlap = 0
    anyOverlap = 0
}

match($0, /([[:digit:]]+)-([[:digit:]]+),([[:digit:]]+)-([[:digit:]]+)/, m) {
    if ((int(m[1]) <= int(m[3]) && int(m[2]) >= int(m[4])) ||
         (int(m[3]) <= int(m[1]) && int(m[4]) >= int(m[2]))) {
             # print "fully overlapping range: " $0
             fullOverlap += 1
    }

    if ((int(m[1]) <= int(m[3]) && int(m[3]) <= int(m[2])) ||
         (int(m[1]) >= int(m[3]) && int(m[1]) <= int(m[4]))) {
            anyOverlap += 1
    }
    # print "range 1: " m[1] "-" m[2] ", range 2: " m[3] "-" m[4]
}

END {
    print "fully overlapping range count: " fullOverlap

    print "ranges with any overlap: " anyOverlap
}

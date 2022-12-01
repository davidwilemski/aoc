BEGIN {
i = 0
elves[i] = 0
}

{
    if (length($0) == 0) {
        i += 1
        elves[i] = 0
    } else {
        elves[i] += $1
    }
}

# /[[:digit:]].+/ {
#     elves[i] += $1
# }

END {
    max_calories = 0
    max_calories_idx = -1
    for(j = 0; j < length(elves); j++) {
        if (elves[j] > max_calories) {
            max_calories = elves[j]
            max_calories_idx = j
        }
    }

    # part 1
    print "part 1"
    print(max_calories_idx, ": ", max_calories)
    print "\n\n"

    print "part 2"
    # we can just sort values without worrying about original indexes since we
    # don't care what the original elf was - we just want the sum of the max 3
    sorted_elves_count = asort(elves, sorted_elves, "@val_num_desc")
    # print "sorted_elves_count: " sorted_elves_count
    # print "i: " i
    # for (j = sorted_elves_count ; j >= (0); j--) {
    #     elf = sorted_elves_idxs[j]
    #     print j, elf, elves[elf]
    # }
    max_three_sum = 0
    for (j = 1 ; j < 4; j++) {
        elf = sorted_elves[j]
        print j, elf #, elves[elf]
        max_three_sum += elf
    }

    print max_three_sum
}

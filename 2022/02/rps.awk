# A Y
# B X
# C Z

# A Rock
# B Paper
# C Scissors
#
# X Rock 1
# Y Paper 2
# Z Scissors 3

# Win 6
# Draw 3
# Loss 0

BEGIN {
    score = 0
}

# Wins
/C X/ { score += (1 + 6) }
/A Y/ { score += (2 + 6) }
/B Z/ { score += (3 + 6) }

# Draws
/A X/ { score += ( 1 + 3) }
/B Y/ { score += ( 2 + 3) }
/C Z/ { score += ( 3 + 3) }

# Losses
/A Z/ { score += 3 }
/B X/ { score += 1 }
/C Y/ { score += 2 }

END {
    print "score: " score
}

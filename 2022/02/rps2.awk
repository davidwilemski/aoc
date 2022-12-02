# second char is outcome
# A Y
# B X
# C Z

# X Lose
# Y Draw
# Z Win

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

# # Wins
# /C X/ { score += (1 + 6) }
# /A Y/ { score += (2 + 6) }
# /B Z/ { score += (3 + 6) }

# # Draws
# /A X/ { score += ( 1 + 3) }
# /B Y/ { score += ( 2 + 3) }
# /C Z/ { score += ( 3 + 3) }

# # Losses
# /A Z/ { score += 3 }
# /B X/ { score += 1 }
# /C Y/ { score += 2 }

# Losses
/A X/ { score += (3 + 0) } # play scissors
/B X/ { score += (1 + 0) } # play rock
/C X/ { score += (2 + 0) } # play paper

# Draws
/A Y/ { score += (1 + 3) } # play rock
/B Y/ { score += (2 + 3) } # play paper
/C Y/ { score += (3 + 3) } # play scissors

# Wins
/A Z/ { score += (2 + 6) } # play paper
/B Z/ { score += (3 + 6) } # play scissors
/C Z/ { score += (1 + 6) } # play rock

END {
    print "score: " score
}

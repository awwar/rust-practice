#MAIN
call #LOTTERY () $LOTTERY_RESULT

#LOTTERY() void
var (-32) $SOME_VALUE

call #FLIP_COIN (0.5) $FLIP_COIN_RESULT

if ($FLIP_COIN_RESULT) (#MORE, #LESS)

print ("Try Next?")

return (void(0))

#MORE
print ("you're WIN!")

#LESS
var (sum("you're lose ", $SOME_VALUE, " points!")) $OUT_RESULT

print ($OUT_RESULT)

#FLIP_COIN(float($FLIP_COIN_ARG0)) bool

return (bool(rand() > $FLIP_COIN_ARG0))
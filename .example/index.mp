#MAIN() void
call #LOTTERY () $LOTTERY_RESULT

#LOTTERY() void
var (-32) $SOME_VALUE

call #FLIP_COIN (0.5) $FLIP_COIN_RESULT

if ($FLIP_COIN_RESULT) (#MORE, #LESS)

call print ("Try Next?") $_

return (void(0))

#MORE() void
call print ("you're WIN!") $_

#LESS() bool
var ("you're lose " + string($SOME_VALUE) + " points!") $OUT_RESULT

call print ($OUT_RESULT) $_

#FLIP_COIN(VAR(float(), $FLIP_COIN_ARG0)) bool

return (bool(rand() > $FLIP_COIN_ARG0))
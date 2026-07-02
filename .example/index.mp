#MAIN() void
CALL #LOTTERY () $LOTTERY_RESULT

#LOTTERY() void
VAR $SOME_VALUE (-32)

CALL #FLIP_COIN (0.5) $FLIP_COIN_RESULT

IF ($FLIP_COIN_RESULT) (#MORE, #LESS)

CALL print ("Try Next?") $_

RETURN (void(0))

#MORE() void
CALL print ("you're WIN!") $_

#LESS() bool
VAR $OUT_RESULT ("you're lose " + string($SOME_VALUE) + " points!")

CALL print ($OUT_RESULT) $_

#FLIP_COIN() bool
DEF struct flip_coin_args {TRESHOLD: float}

return (bool(rand() > flip_coin_args.TRESHOLD))
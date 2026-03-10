#MAIN() void
var (array(int(0))) $INITIAL_VALUE

call fill_random ($INITIAL_VALUE, 10, -100, 100) $FILLED_VALUE

call print (AT($FILLED_VALUE, 3)) $_
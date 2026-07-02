#MAIN() void
VAR $INITIAL_VALUE (array(int(0))) 

CALL fill_random ($INITIAL_VALUE, 10, -100, 100) $FILLED_VALUE

CALL print (AT($FILLED_VALUE, 3)) $_
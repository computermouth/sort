
BIN := c.sort

$(BIN): main.c
	gcc -Wall main.c -o $(BIN)

memtest: $(BIN)
	echo "ok\nfarts\nstuff\nzebra\njunk\naaa\nzzz" | valgrind --track-origins=yes --leak-check=yes ./$(BIN)


BIN := go.sort

$(BIN): main.go
	go build

memtest: $(BIN)
	echo "ok\nfarts\nstuff\nzebra\njunk\naaa\nzzz" | ./$(BIN)

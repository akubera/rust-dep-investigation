
.PHONY: A B C D1 D2

all: A

run:
	cd A && cargo run

A: B C
	cd A && cargo build --verbose

B: D1
	cd B && cargo build

C: D2
	cd C && cargo build

D1:
	cd D && cargo build

D2:
	cd D2 && cargo build


clean:
	cd A && cargo clean
	cd B && cargo clean
	cd C && cargo clean
	cd D && cargo clean
	cd D2 && cargo clean
	cd animal && cargo clean

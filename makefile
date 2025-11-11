.PHONY: all run release release-run clean

all:
	./build.py

run:
	./build.py --run

release:
	./build.py --release

release-run:
	./build.py --release --run

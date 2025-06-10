.SUFFIXES: # disable default makefile rules
.PHONY: run, build, open_doc, doc, clean

all: run

run: build
	build/exec/a.out

build: a.out

a.out:
	idris2 advent_4.idr -o a.out

open_doc: doc


doc:
	idris2 --mkdoc advent_4.ipkg
# @ prefix hides outputting command text itself
	@echo Docs built. Located at `pwd`/build/docs/index.html

clean:
	rm -rf build/
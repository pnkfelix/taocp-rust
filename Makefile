default: run-ll3
# dancing_links.bin run-dancing_links 
# exact_cover.bin run-exact_cover
# run-latin-squares
# run-bool
# run-play
# run-ll2
# run-ll
# run-ex

RUSTC=rustc -g

run-sets: sets.bin
run-graphs: graphs.bin
run-gs: gs.bin

run-bool: bool_formulas.bin
	./$<

run-langford: langford.bin
	./$<

run-ll3: llist3.bin
	./$<

run-ll2: llist2.bin
	./$<

run-%: %.bin
	./$<

run-ll: llist.bin
	./$<

run-sl: slist.bin
	./$<

run-ex: ex-sec02-01
	./$<

run-ex-old: ex-sec01-03-02

ex-%: ex-%.rs
	$(RUSTC) -o $@ $<

graphs.bin: graphs.rs libsets.dylib
gs.bin: gs.rs libsets.dylib

LIBSETS=ls -t libsets-*.dylib | head -1

libsets.dylib: sets.rs
	$(RUSTC) --lib $<
	ln -f -s `$(LIBSETS)`g $@

%.bin: %.rs
	$(RUSTC) -L. -o $@ $<

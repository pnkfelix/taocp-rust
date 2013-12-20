default: run-bool # run-play # run-ll2 # run-ll # run-ex

run-sets: sets.bin
run-graphs: graphs.bin
run-gs: gs.bin

run-bool: bool_formulas.bin
	./$<

run-langford: langford.bin
	./$<

run-ll2: llist2.bin

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
	rustc -o $@ $<

graphs.bin: graphs.rs libsets.dylib
gs.bin: gs.rs libsets.dylib

LIBSETS=ls -t libsets-*.dylib | head -1

libsets.dylib: sets.rs
	rustc --lib $<
	ln -f -s `$(LIBSETS)`g $@

%.bin: %.rs
	rustc -L. -o $@ $<

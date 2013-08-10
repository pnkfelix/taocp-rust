default: run-bool # run-ll2 # run-ex

run-bool: bool_formulas.bin
	./$<

run-langford: langford.bin
	./$<

run-ll2: llist2.bin
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

%.bin: %.rs
	rustc -o $@ $<

default: run-ll # run-ex

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

default: run-ex

run-ex: ex-sec02-01
	./$<

run-ex-old: ex-sec01-03-02

ex-%: ex-%.rs
	rustc -o $@ $<

Knuth speaks of "generating" all of the combinatorial objects that we need,
and "visiting" each object in turn.

For this go-round, lets take a shot at trying to encode this via the
Rust iterator API.

Ah, yes, I forgot the most obvious problem with this plan:
you really want the iterator to return a temporary reference
into the iterator's internal state, with a guarantee that
the iteration body will be done before the next call to
the iterator's `next` method.

Okay, weill, I am not terribly interesting in trying to
adjust the `for` loop API for this. Let us instead focus
on alternative protocols that do not leverage `for`.

```rust
pub mod n_tuples;
```

Eventually I will have unit tests, either here or in the submodules.

For now this can be a placeholder to remind me of that.

```
#[test]
fn works2() { }
```

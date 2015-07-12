SIMD
===
Working on a compiler plugin to add [ISPC](http://ispc.github.io/) like code generation capabilities. The
goal is that you'll be able to write a function and tag it with a `#[simd]` annotation to have
a vectorized version of it generated. The vectorized version will still be able to handle branching,
looping and so on but conditional control flow will be handled by masking off vector lanes. You
would then call the function from a SIMD context (eg. a vectorized for loop).

For example, you should eventually be able to write something like this:

```rust
#[simd]
fn foo(x: f32) -> f32 {
	if x > 0 {
		x * 5
	} else {
		x * 2
	}
}

// Now call from a vectorized context
#[simd]
for x in 0..vec.len() {
	vec[i] = foo(vec[i]);
}
```

Providing some kind of vectorized iterator would be even better, but we'll see how things go.

It's probably clear but worth mentioning, this is very much in development.
	


## TAOCP main

What happens to cargo if your `bin` program has no `fn main` definition?

```
   Compiling taocp-rust v0.0.1 (file:///Users/fklock/Dev/Rust/taocp-rust)
error: main function not found
```

So, lets not do that.

```rust
pub fn main() {
    println!("Hello from TAOCP");
}
```

I like drawing pictures too:

<svg xmlns="http://www.w3.org/2000/svg">
  <circle r="25%" cx="30%" cy="50%"
  fill="lightgreen"
  stroke="#e60" stroke-width="25" />
  <circle r="25%" cx="50%" cy="50%"
  fill="none"
  stroke="#e60" stroke-width="25" />
</svg>

<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" >
  <title>Collage involving &lt;rect&gt; , &lt;circle&gt; and &lt;ellipse&gt; </title>
  <circle cx="50%" cy="50%" r="25%" fill="none" stroke="#e60" stroke-width="25"/>
</svg>

<svg xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink" >
  <title>Collage involving &lt;rect&gt; , &lt;circle&gt; and &lt;ellipse&gt; </title>
  <circle cx="50%" cy="50%" r="25%" fill="none"
          stroke="#e60" stroke-width="25"/>
  <rect x="10%" width="80%" y="50%" height="10%" fill="#8ff"
    stroke="black" stroke-width="6" />
  <ellipse cx="50%" cy="50%" rx="10%" ry="40%" fill="yellow" fill-opacity=".45"
    stroke="purple" stroke-width="15" />
</svg>

Excerpts From: David Dailey, Jon Frost, and Domenico Strazzullo. “Building Web Applications with SVG (Felix Klock's Library).” iBooks.

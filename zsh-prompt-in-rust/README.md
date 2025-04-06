This is my `zsh` `$PS1`, in Rust.

# Are you crazy?

Probably.

# Why tho?

Obviously, you *can* do this in shell. (In fact, this prompt was originally in
shell.)

To me, it was an opportunity to just be an artisan in one small facet of my
day-to-day compute. Rust is *far* more efficient than the corresponding shell,
in two areas:

* the number of times we have to exec and entire child process for some small
  piece of data, or some small transformation
* the number of memory allocations required

Rust can do many of the transformations directly (`basename`, `sed`) and there
was one spot where we had to run `python3` to get the names of the various
signals. (We cache that result at startup, but with Rust it's just compiled
into the binary, and available.)

Much of the prompt's "pieces" don't require allocations, and are
just stack-allocated; the entire prompt string is printed to `zsh` in one go
near the end of execution.

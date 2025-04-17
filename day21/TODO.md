# Brute Force, But Less Memory
Yes, it takes time to generate the full sequence of key presses for the human.
But one of the things that is contributing to poor performance is the massive
memory use.  On the last pass for each code, the input string is on the order
of 40GB, and the output string is on the order of 70GB.  This is causing huge
amounts of paging.  Luckily, we go through both sequentially, so paging or
compression can be relatively efficient.

## Nested Iterators
Make the presses_for_directional_code() function take an iterator (of char?)
as input, and produce an iterator (of the same type) as output.  Then just
nest them 25 deep.  Do a .count() on the final iterator to get the length
of that string.  Using generics and manually listing all 25 passes
sequentially in the source (i.e. no loop), it will specialize the code
for each nesting level.

### Dynamic Types for Iterators
Use a loop that calls presses_for_directional_code() 25 times, much like
it does with String inputs and outputs, reusing the same variable.  This
requires that the input and output type be the same, and that they not
be specialized based on the nesting level.  I think Rust uses something
like `impl Iterator<Item=char>` to express this.

## Chain of Filters
Here, my mental model is Unix/Posix style "filter" programs that take a
stream of text as input, and produce a stream of text as output.  Build
up a pipeline of 25 of these (each in its own thread), and count the
characters in the final output.

It will need some sort of buffer type that is capable of holding at least
a few characters (enough for the worst case of movements to type one key).
It needs to allow at least one reader and one writer, with appropriate
concurrency protection.  It needs to be able to block a reader or writer
if data (or free space) is not available yet.

Can I just create pipes, and read/write them?

# Caching
I could cache a transformation of some sequence of arrows followed by "A"
into the appropriate output.  This would eliminate having to run the tiny
loops that figure out the movements, but still producing the full output
sequence.

We need to know the length of the output, not the actual characters.  Can
we cache that somehow?

In order for caching to make a big difference, it needs to have lots of
cache hits.  It needs to see the same inputs many times.

We could use brute force for the first 10 robots in the sequence, and then
cache the (length of the) result of the next 15 robots.  The first 10 will
produce a string on the order of 100KB, which should make for lots of
cache hits.

That 10/15 split is arbitrary.  If we pass the number of robots in the chain
as a parameter, and use a cache to memoize recursive calls, it would
automatically discover repeated patterns.

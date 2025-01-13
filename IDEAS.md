## Day 3

* Use one of the built-in number parsers (eg., `complete::u32`) to parse
    the arguments of the `mul` instruction.  (There are no cases where a
    `mul` instruction is invalid because it has too many digits.)
* Use the `many_till` and `anychar` parsers to ignore the characters between
    instructions.
* Write an `ignore_till` parser, based on `many_till`, that skips over input
    until the given parser matches.
* Write a state machine to do the parsing and state to compute results.

## Day 4
### Part 1
* Use `ndarray` instead of `Vec<Vec<char>>` to access the input.
* Figure out how to find all of the (non-overlapping) diagonals, and window
  through them in the same way as for horizontal and vertical.  I think you
  start on the left edge, going up, then top edge, going right.
* (Advanced) Since we're looking for a 4-byte pattern, we can represent 4
  consecutive letters as a single u32.  We can pre-compute 4-character
  sequences by doing numeric array operations on suitably shifted variants
  of the input.

  For example, a horizontal pattern could be something like:
    array << 24 + array.offset_by(1) << 16 + array.offset_by(2) << 8 + array.offset_by(3)
  
  Then just search for the equivalent of 'XMAS' or 'SAMX', and count them.

  Those array operations are potentially vectorizable (although alignment could
  be a problem).
  
## Day 5
* Parts 1 and 2 could be computed in a single pass, doing almost the same work
  as part 2 alone.

## Day 6
### Part 2
Part 2 is really slow (about 1.34 optimized, or almost 20 seconds debug!).
We need to not test the guard's entire path from original starting position
for every potential obstacle location.

Changing the Row and Col types to i32 and i16 helped a little.  This may have
sped up the hash computation.  It would have also reduced the overall size of
the HashMap's memory usage.

* Look into a cheaper hashing library?
* Change the structure/size of the hash keys?

* Can we reset the guard's position to just before hitting the new obstacle,
  and then check for loops?

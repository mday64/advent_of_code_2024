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

* Look into a cheaper hashing library? (Using FxHash helped.)
* Change the structure/size of the hash keys?

* Can we reset the guard's position to just before hitting the new obstacle,
  and then check for loops?  (Yes, did that, too.)

* We're spending a lot of time inserting new positions into a HashSet.
* We're spending some time doing bounds checks.

* Perhaps we should surround the input grid with some kind of "out of
  bounds" character, to simplify the checking (when moving into a
  space we'd find an obstacle or out-of-bounds, then decide what to do).
  (Done)

* It might help to keep going forward until reaching the next obstacle
  (to avoid repeated checks of the current direction).  Then we only need
  to keep track of the obstacles we encounter (and from which direction?).
  (Done)

* It might help to precompute or cache the next obstacle (or out of bounds) when
  hitting an obstacle from each side.  When testing potential new obstacle
  locations, don't actually add them to the grid; instead, check whether
  the most recent go_forward() line segment would have passed through the
  potential obstacle location.  It might work better if go_forward() didn't
  update the Guard's location, but instead returned the number of spaces
  that can be moved in the direction the Guard is facing.

* Part 1 could potentially make use of the "go forward as far as possible
  in a single direction" method, especially if it merely returns the number
  of steps.  That would help pre-populate the cache for part 2.  But it would
  require extra processing to build the list of visited locations and
  directions (used for potential obstacles in part 2).

## Day 9
### Part 2
* Try keeping binary heaps for free chunks (sorted by position).  Have a
  heap for each size (1-9).  When looking to move a file, peek at all of
  the heaps of that size or larger, and select the smallest position.
  Obviously, the free space needs to be removed from its heap; if the
  chunk was not fully used, insert the remainder in the appropriate size
  heap.

  NOTE: std::collections::binary_heap::BinaryHeap lacks a heapify() method,
  although it can extend() from an iterator.  It can also do from() an
  array.

  NOTE: BinaryHeap is a MAX heap.  Use std::cmp::Reverse to wrap elements
  so that they sort in reverse order (i.e. a min heap).

## Day 10
Solve both parts with a single traversal.  Use a depth-first search to find
all paths from a given trailhead.  Count (using a hashmap) how many times
each trail end ('9') is reached.  The answer for part 1 will be the number
of keys in the hashmap. The answer for part 2 will be the sum of the values
in the hashmap.

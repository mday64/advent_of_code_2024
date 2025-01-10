## Day 3

* Use one of the built-in number parsers (eg., `complete::u32`) to parse
    the arguments of the `mul` instruction.  (There are no cases where a
    `mul` instruction is invalid because it has too many digits.)
* Use the `many_till` and `anychar` parsers to ignore the characters between
    instructions.
* Write an `ignore_till` parser, based on `many_till`, that skips over input
    until the given parser matches.
* Write a state machine to do the parsing and state to compute results.

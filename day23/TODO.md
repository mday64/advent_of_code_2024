Part 2 is slow.  How to make it faster?

As currently written, we end up generating every subset of the solution.
Since the answer has length 13, that means we are generating 8192 subsets
(not to mention the subsets for all smaller strongly connected components).

We can produce larger connected components more quickly by combining smaller
connected components (if all of the nodes are strongly connected to each other).
For example, if "a,b,c,d,f,g,h" is a strongly connected component, and we know
"a,b" and "c,d" are each strongly connected, and "a" and "b" are each connected
to both "c" and "d", then we can merge "a,b" and "c,d" into "a,b,c,d".
Similarly, we could merge "a,b,c,d" with "f,g,h".  The only hard part is that
another node might be fully connected to a smaller subset of the solution,
but not to the rest.  For example, "e" might be connected to each of "a,b,c,d",
but not to "f,g,h".  We need to keep "a,b,c,d" around to try to merge with
"f,g,h", and also keep "a,b,c,d,e" in case it merges with later nodes.

I was thinking that if we keep nodes and connected components in sorted order,
there might be a greedy algorithm that only requires one pass through all of
the nodes.  The idea is to keep track of all known connected components (of
all sizes), and see if we can add the next node.  The current node always gets
added as a size=1 connected component in case it is the first of a larger one.

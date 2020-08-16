# rstsort: Topological sort in rust

The `rstsort` program can be used to construct a
[directed graph](https://en.wikipedia.org/wiki/Directed_graph)
and perform a
[topological sort](https://en.wikipedia.org/wiki/Topological_sorting)
on that graph. It is roughly analogous to the
[POSIX tsort](https://en.wikipedia.org/wiki/Tsort)
program available on UNIX-like operating systems.

The input is a series of edges or adjacency lists where the leftmost string is
the name of the node and the rest of the strings on the line, separated by
spaces are the nodes to which the original node has outgoing edges.

Output is printed with one node-name per line.

## Example

Here is an example of using `rstsort` taking its input from stdin:

```sh
$ rstsort
a b
b c
a d
^D
a
d
b
c
```

## Why?

I mostly wrote this program as a way to learn of a good way to implement data
structures that contain a lot of "pointers" and that would normally be
difficult to implement in Rust because of the issues of ownership of those
pointers. The digraph data structure used is implemented as an
[adjacency list](https://en.wikipedia.org/wiki/Adjacency_list).
Normally, this would cause issues because of the pointers, so rather than deal
with that at all, this program uses
[arena allocation](https://en.wikipedia.org/wiki/Region-based_memory_management)
which is similar to
[slab allocation](https://en.wikipedia.org/wiki/Slab_allocation).
Rather than returning a pointer to an allocated node, a "handle" is returned,
which contains an index into an array of nodes with an "arena" (an array) of
contiguous memory where all of the nodes live. The handle can be used to
retrieve a node from a graph without requiring holding on to pointers. This
allows the entire arena to be moved in memory without invalidating any pointers
since the handle indices never change.

This approach was heavily inspired by
[Catherine West's RustConf Talk](https://www.youtube.com/watch?v=aKLntZcp27M)
and
[this blog post](https://floooh.github.io/2018/06/17/handles-vs-pointers.html).

## License

This project is licensed under the MIT license.

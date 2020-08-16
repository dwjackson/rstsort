# rstsort: Topological sort in rust

The `rstsort` program can be used to construct a
[directed graph](https://en.wikipedia.org/wiki/Directed_graph)
and perform a
[topological sort](https://en.wikipedia.org/wiki/Topological_sorting)
on that graph.

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

## License

This project is licensed under the MIT license.

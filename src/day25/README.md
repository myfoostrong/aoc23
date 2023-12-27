# Notes
The final challenge defines an undirected graph, and I believe is asking you to solve the [minimum cut problem](https://en.wikipedia.org/wiki/Minimum_cut).

There's an optimized random function with high probability called [Karger's algorithm](https://en.wikipedia.org/wiki/Karger%27s_algorithm).

Or, I could try solving the [minimum k-cut problem](https://en.wikipedia.org/wiki/Minimum_k-cut] which is less efficient time-wise, but would be more flexible wrt solving the second half of the puzzle...

Welp, someone happened to provide some [Rusty version of Karger's](https://gvelim.github.io/CSX0003RUST/graph_min_cut.html)...

Someone also wrote a [library of petgraph algos](https://docs.rs/graphalgs/latest/graphalgs/connect/fn.articulation_points.html)..

Finally, [this gist](https://gist.github.com/kolloldas/3bc916097d8997a002619fe7066cf717) made more sense to me so ported that to Rust.

# Challenge

You have nowhere near that many stars - you need to find a way to disconnect at least half of the equipment here, but it's already Christmas! You only have time to disconnect three wires.

Fortunately, someone left a wiring diagram [your puzzle input] that shows how the components are connected. For example:

jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
Each line shows the name of a component, a colon, and then a list of other components to which that component is connected. Connections aren't directional; abc: xyz and xyz: abc both represent the same configuration. Each connection between two components is represented only once, so some components might only ever appear on the left or right side of a colon.

In this example, if you disconnect the wire between hfx/pzl, the wire between bvb/cmg, and the wire between nvd/jqt, you will divide the components into two separate, disconnected groups:

9 components: cmg, frs, lhk, lsr, nvd, pzl, qnr, rsh, and rzs.
6 components: bvb, hfx, jqt, ntq, rhn, and xhk.
Multiplying the sizes of these groups together produces 54.

Find the three wires you need to disconnect in order to divide the components into two separate groups. What do you get if you multiply the sizes of these two groups together?


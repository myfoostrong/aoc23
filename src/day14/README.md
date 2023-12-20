# Notes
The first solution was quick and easy, solved inline while parsing the input.

Second solve shows more memory was needed, to track where all the objects were. I went with building out the full grid, but I don't think we were actually supposed to loop through 1000000000 times. Analyzing the answers up to 200 cycles showed a pattern, but one I didn't clearly see the sequence of. So totaled the number of times certain values showed up in the sequence, and just guessesed. 
```
95251: 7
95252: 7
95253: 8
95254: 7
95255: 7
95262: 8
95264: 8
95265: 8
95267: 15
95269: 7
95270: 7
95273: 7
95274: 7
```

```
0: 87
1: 69
2: 69
3: 69
4: 65
5: 64
6: 65
7: 63
8: 68
9: 69
10: 69
11: 65
12: 64
13: 65
14: 63
15: 68
16: 69
17: 69
18: 65
19: 64
20: 65
21: 63
22: 68
23: 69
24: 69
25: 65
26: 64
27: 65
28: 63
29: 68
30: 69
31: 69
32: 65
33: 64
34: 65
35: 63
36: 68
37: 69
38: 69
39: 65
40: 64
41: 65
42: 63
43: 68
44: 69
45: 69
46: 65
47: 64
48: 65
49: 63
50: 68
51: 69
52: 69
53: 65
54: 64
55: 65
56: 63
57: 68
58: 69
59: 69
60: 65
61: 64
62: 65
63: 63
64: 68
65: 69
66: 69
67: 65
68: 64
69: 65
70: 63
71: 68
72: 69
73: 69
74: 65
75: 64
76: 65
77: 63
78: 68
79: 69
```

```
0: 99713
1: 99719
2: 99808
3: 99857
4: 99797
5: 99762
6: 99826
7: 99846
8: 99890
9: 99943
10: 99984
11: 100046
12: 100093
13: 100089
14: 100069
15: 100096
16: 100114
17: 100116
18: 100096
19: 100121
20: 100106
21: 100116
22: 100107
23: 100108
24: 100113
25: 100112
26: 100094
27: 100054
28: 99995
29: 99921
30: 99828
31: 99752
32: 99651
33: 99528
34: 99379
35: 99251
36: 99129
37: 99041
38: 98950
39: 98886
40: 98811
41: 98751
42: 98670
43: 98590
44: 98486
45: 98428
46: 98390
47: 98367
48: 98354
49: 98341
50: 98301
51: 98275
52: 98228
53: 98177
54: 98101
55: 98068
56: 98024
57: 97986
58: 97936
59: 97878
60: 97810
61: 97739
62: 97658
63: 97577
64: 97487
65: 97418
66: 97331
67: 97256
68: 97169
69: 97087
70: 96993
71: 96904
72: 96820
73: 96753
74: 96672
75: 96586
76: 96489
77: 96417
78: 96330
79: 96253
80: 96176
81: 96103
82: 96028
83: 95948
84: 95860
85: 95789
86: 95727
87: 95667
88: 95619
89: 95570
90: 95513
91: 95463
92: 95411
93: 95403
94: 95361
95: 95326
96: 95310
97: 95267
98: 95253
99: 95264
```


--- Day 14: Parabolic Reflector Dish ---
You reach the place where all of the mirrors were pointing: a massive parabolic reflector dish attached to the side of another large mountain.

The dish is made up of many small mirrors, but while the mirrors themselves are roughly in the shape of a parabolic reflector dish, each individual mirror seems to be pointing in slightly the wrong direction. If the dish is meant to focus light, all it's doing right now is sending it in a vague direction.

This system must be what provides the energy for the lava! If you focus the reflector dish, maybe you can go where it's pointing and use the light to fix the lava production.

Upon closer inspection, the individual mirrors each appear to be connected via an elaborate system of ropes and pulleys to a large metal platform below the dish. The platform is covered in large rocks of various shapes. Depending on their position, the weight of the rocks deforms the platform, and the shape of the platform controls which ropes move and ultimately the focus of the dish.

In short: if you move the rocks, you can focus the dish. The platform even has a control panel on the side that lets you tilt it in one of four directions! The rounded rocks (O) will roll when the platform is tilted, while the cube-shaped rocks (#) will stay in place. You note the positions of all of the empty spaces (.) and rocks (your puzzle input). For example:

O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
Start by tilting the lever so all of the rocks will slide north as far as they will go:

OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
You notice that the support beams along the north side of the platform are damaged; to ensure the platform doesn't collapse, you should calculate the total load on the north support beams.

The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the south edge of the platform, including the row the rock is on. (Cube-shaped rocks (#) don't contribute to load.) So, the amount of load caused by each rock in each row is as follows:

OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1
The total load is the sum of the load caused by all of the rounded rocks. In this example, the total load is 136.

Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load on the north support beams?

Your puzzle answer was 108857.

--- Part Two ---
The parabolic reflector dish deforms, but not in a way that focuses the beam. To do that, you'll need to move the rocks to the edges of the platform. Fortunately, a button on the side of the control panel labeled "spin cycle" attempts to do just that!

Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east. After each tilt, the rounded rocks roll as far as they can before the platform tilts in the next direction. After one cycle, the platform will have finished rolling the rounded rocks in those four directions in that order.

Here's what happens in the example above after each of the first few cycles:

After 1 cycle:
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....

After 2 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O

After 3 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
This process should work if you leave it running long enough, but you're still worried about the north support beams. To make sure they'll survive for a while, you need to calculate the total load on the north support beams after 1000000000 cycles.

In the above example, after 1000000000 cycles, the total load on the north support beams is 64.

Run the spin cycle for 1000000000 cycles. Afterward, what is the total load on the north support beams?

Your puzzle answer was 95273.

Both parts of this puzzle are complete! They provide two gold stars: **


# maze
maze generation, formatting and visualization

There are 3 programs:
 - `maze` - create a maze (note only inner walls are created, not the outside walls)
 - `maze_fmt_convert` - changes the wall statements from TILE centric to LINE centric
 - `maze_visualizer` - displays a simple ascii version of the maze

To create a 20 x 10 maze with a single path between any two points in TILE centric format:
```
maze --width 20 --height 10 > m1.maze
```

To create a 20 x 10 maze with approximately 15% of the inner walls removed in LINE centric format with a 3x3 room randomly placed inside the maze:
```
maze --width 20 --height 10 --remove-percentage 15.0 --room-size 3 | maze_fmt_convert > m2.maze
```

To display m2.maze in a terminal:
```
maze_visualizer --width 20 --height 10 --line < m2.maze
```

To create a maze for the Harp Lab's maze system (this style of maze is 11x11 with many small rooms to make the passages wider)
```
maze --harp | maze_fmt_convert > m3.maze
```

# Game of Live

A Conway's Game of Live kata in Rust.

![Game of Life](./docs/media/game-of-life.gif)

The universe of the Game of Life is an infinite, two-dimensional orthogonal grid of square cells, each of which is in one of two possible states, live or dead (or populated and unpopulated, respectively). Every cell interacts with its eight neighbours, which are the cells that are horizontally, vertically, or diagonally adjacent.

## Rules

From: <https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life>

 At each step in time, the following transitions occur:

- Any live cell with fewer than two live neighbours dies, as if by underpopulation.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by overpopulation.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

These rules, which compare the behaviour of the automaton to real life, can be condensed into the following:

- Any live cell with two or three live neighbours survives.
- Any dead cell with three live neighbours becomes a live cell.
- All other live cells die in the next generation. Similarly, all other dead cells stay dead.

The initial pattern constitutes the seed of the system. The first generation is created by applying the above rules simultaneously to every cell in the seed, live or dead; births and deaths occur simultaneously, and the discrete moment at which this happens is sometimes called a tick. The simultaneity means that when each cell counts the number of live neighbors around it, it uses its neighbors' old states before the update, not their new states after the update. If the cells are instead updated in reading order, so that each cell uses the old states of the cells to its right and below it but the new states of the cells to its left and above it, different cellular automaton results, which is known as NaiveLife because it is a common beginners' mistake among people attempting to program Conway's Game of Life.

Each generation is a pure function of the preceding one. The rules continue to be applied repeatedly to create further generations.

## Commands

Run:

```s
cargo run
```

The app will show you the available options, for example:

```s
cargo run ./patterns/glider.txt 30 60 1000 1
```

That command will run the "glider" pattern in a 30x60 background grid.

Run tests:

```s
cargo test
```

Run Clipply:

```s
cargo clippy --all-targets -- -D clippy::pedantic
```

## Ideas

- A new grid function `grid_expand` could be used to expand a pattern grid. It could be implemented internally with `grip_overlap`. It expands from the grid center.
- Create a toroidal array and use it like the container in the grid.

## Todo

- Workflow to run tests with coverage report.
- Workflow to publish crate.
- Cargo alias for code coverage.

See `todo` labels in the code.

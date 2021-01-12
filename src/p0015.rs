// Lattice paths
//
// Starting in the top left corner of a 2×2 grid, and only being able to move to the
// right and down, there are exactly 6 routes to the bottom right corner.
//
//      >   >   V           >   V   .           >   V   .
//      .   .   V           .   >   V           .   V   .
//      .   .   V           .   .   V           .   >   >
//
//
//      V   .   .           V   .   .           V   .   .
//      >   >   V           >   V   .           V   .   .
//      .   .   V           .   >   >           >   >   >
//
// How many such routes are there through a 20×20 grid?

fn compute(size: usize) -> u64 {
    let mut grid = vec![vec![1; size + 1]; size + 1];

    for x in 1..=size {
        for y in 1..=size {
            grid[x][y] = grid[x - 1][y] + grid[x][y - 1];
        }
    }

    grid[size][size]
}

#[test]
fn example() {
    assert_eq!(6, compute(2));
}

#[test]
fn problem() {
    assert_eq!(137846528820, compute(20));
}

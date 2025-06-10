fn main() {
    const SEARCH_FOR: [char; 4] = ['X', 'M', 'A', 'S'];
    let input = include_str!("input.txt");

    // Get the grid of characters
    // [
    // ['A', 'B', 'C'],
    // ['D', 'E', 'F'],
    // ['G', 'H', 'I']
    // ]
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1), // up-left
        (0, -1),  // up
        (1, -1),  // up-right
        (-1, 0),  // left
        (1, 0),   // right
        (-1, 1),  // down-left
        (0, 1),   // down
        (1, 1),   // down-right
    ];

    let mut sequence_count = 0;

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] != 'X' {
                continue;
            }

            'dir_loop: for (dy, dx) in DIRECTIONS {
                // Check if we can fit the whole sequence in this direction
                let mut valid = true;

                for i in 0..SEARCH_FOR.len() {
                    let ny = y as i32 + dy * i as i32;
                    let nx = x as i32 + dx * i as i32;

                    if ny < 0 || ny >= rows as i32 || nx < 0 || nx >= cols as i32 {
                        continue 'dir_loop;
                    }

                    let current_char = grid[ny as usize][nx as usize];
                    if current_char != SEARCH_FOR[i] {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    sequence_count += 1;
                }
            }
        }
    }

    println!("Found {} XMAS sequences", sequence_count);
}

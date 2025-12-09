#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Parse red tiles
    let objects: Vec<(usize, usize)> = input
        .lines()
        .filter_map(|line| {
            line.split_once(',').map(|(x, y)| {
                (
                    x.parse::<usize>().unwrap(),
                    y.parse::<usize>().unwrap(),
                )
            })
        })
        .collect();

    // Coordinate compression
    let (compressed_objects, x_map, y_map) =
        compress_coordinates(&objects);

    let mut grid = init_grid(&compressed_objects);
    draw_lines(&mut grid, &compressed_objects);
    fill_interior(&mut grid);
    let answer = largest_red_rectangle(
        &grid,
        &compressed_objects,
        &x_map,
        &y_map,
    );

    Ok(answer.to_string())
}

fn compress_coordinates(
    objects: &[(usize, usize)],
) -> (
    Vec<(usize, usize)>,
    Vec<usize>,
    Vec<usize>,
) {
    use std::collections::BTreeSet;

    let x_coords: BTreeSet<usize> =
        objects.iter().map(|(x, _)| *x).collect();
    let y_coords: BTreeSet<usize> =
        objects.iter().map(|(_, y)| *y).collect();

    let x_map: Vec<usize> =
        x_coords.iter().copied().collect();
    let y_map: Vec<usize> =
        y_coords.iter().copied().collect();

    let x_to_compressed: std::collections::HashMap<
        usize,
        usize,
    > = x_map
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect();
    let y_to_compressed: std::collections::HashMap<
        usize,
        usize,
    > = y_map
        .iter()
        .enumerate()
        .map(|(i, &y)| (y, i))
        .collect();

    let compressed_objects: Vec<(usize, usize)> = objects
        .iter()
        .map(|(x, y)| {
            (x_to_compressed[x], y_to_compressed[y])
        })
        .collect();

    (compressed_objects, x_map, y_map)
}

fn init_grid(objects: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let max_x =
        objects.iter().map(|(x, _)| *x).max().unwrap();
    let max_y =
        objects.iter().map(|(_, y)| *y).max().unwrap();
    let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
    for &(x, y) in objects {
        grid[y][x] = true;
    }
    grid
}

fn fill_interior(grid: &mut [Vec<bool>]) {
    let height = grid.len();
    let width = grid[0].len();

    let mut exterior = vec![vec![false; width]; height];
    let mut queue = std::collections::VecDeque::new();

    for x in 0..width {
        if !grid[0][x] {
            queue.push_back((x, 0));
            exterior[0][x] = true;
        }
        if !grid[height - 1][x] {
            queue.push_back((x, height - 1));
            exterior[height - 1][x] = true;
        }
    }
    for y in 0..height {
        if !grid[y][0] {
            queue.push_back((0, y));
            exterior[y][0] = true;
        }
        if !grid[y][width - 1] {
            queue.push_back((width - 1, y));
            exterior[y][width - 1] = true;
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && nx < width as i32
                && ny >= 0
                && ny < height as i32
            {
                let nx = nx as usize;
                let ny = ny as usize;
                if !grid[ny][nx] && !exterior[ny][nx] {
                    exterior[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if !exterior[y][x] {
                grid[y][x] = true;
            }
        }
    }
}

fn draw_lines(
    grid: &mut [Vec<bool>],
    objects: &[(usize, usize)],
) {
    let n = objects.len();
    for i in 0..n {
        let (x1, y1) = objects[i];
        let (x2, y2) = objects[(i + 1) % n];
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[y][x1] = true;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                grid[y1][x] = true;
            }
        } else {
            panic!(
                "Only horizontal or vertical lines allowed"
            );
        }
    }
}

fn largest_red_rectangle(
    grid: &[Vec<bool>],
    red_tiles: &[(usize, usize)],
    x_map: &[usize],
    y_map: &[usize],
) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    // Build 2D prefix sum array for O(1) rectangle sum queries
    let mut prefix =
        vec![vec![0i64; width + 1]; height + 1];
    for y in 0..height {
        for x in 0..width {
            prefix[y + 1][x + 1] = prefix[y][x + 1]
                + prefix[y + 1][x]
                - prefix[y][x]
                + if grid[y][x] { 1 } else { 0 };
        }
    }

    let n = red_tiles.len();

    // Create all valid pairs with their potential areas (in original coordinates)
    let mut pairs = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            if x1 == x2 || y1 == y2 {
                continue;
            }

            // Calculate potential area using original coordinates
            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);

            let orig_xdiff = x_map[xmax] - x_map[xmin] + 1;
            let orig_ydiff = y_map[ymax] - y_map[ymin] + 1;
            let potential_area = orig_xdiff * orig_ydiff;

            pairs.push((potential_area, i, j));
        }
    }

    // Sort by potential area descending for better early termination
    pairs.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut max_area = 0;

    for (potential_area, i, j) in pairs {
        // Early termination: if potential area can't beat max, stop
        if potential_area <= max_area {
            break;
        }

        let (x1, y1) = red_tiles[i];
        let (x2, y2) = red_tiles[j];

        let xmin = x1.min(x2);
        let xmax = x1.max(x2);
        let ymin = y1.min(y2);
        let ymax = y1.max(y2);

        // Check if rectangle is valid in compressed space
        let compressed_area =
            (xmax - xmin + 1) * (ymax - ymin + 1);

        let sum = prefix[ymax + 1][xmax + 1]
            - prefix[ymin][xmax + 1]
            - prefix[ymax + 1][xmin]
            + prefix[ymin][xmin];

        if sum == compressed_area as i64 {
            // Calculate actual area using original coordinates
            let actual_area = (x_map[xmax] - x_map[xmin]
                + 1)
                * (y_map[ymax] - y_map[ymin] + 1);
            max_area = max_area.max(actual_area);
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("24", process(input)?);
        Ok(())
    }
}

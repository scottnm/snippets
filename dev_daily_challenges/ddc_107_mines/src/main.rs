#[macro_export]
macro_rules! BoolVec2D {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Vec<bool>> = Vec::new();
            $(
                temp_vec.push($x.to_vec());
            )*
            temp_vec
        }
    };
}

type ConstStr = &'static str;
type StringVec = Vec<ConstStr>;
type CaveMap = Vec<Vec<bool>>;

#[derive(Copy, Clone, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn direction_string(d: Direction) -> ConstStr {
    match d {
        Direction::Left => "Left",
        Direction::Right => "Right",
        Direction::Down => "Down",
        Direction::Up => "Up",
    }
}

fn solve_dfs(cave_map: &CaveMap, miner: Pos, exit: Pos) -> StringVec {
    fn solve_dfs_helper(path: &mut StringVec, cave_map: &mut CaveMap, miner: Pos, exit: Pos) -> bool {
        const DIRECTIONS: [Direction; 4] = [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ];

        if miner == exit {
            return true
        }

        cave_map[miner.x][miner.y] = false;

        let width = cave_map.len();
        let height = cave_map[0].len();

        for direction in &DIRECTIONS {
            let next_pos = match direction {
                Direction::Left if miner.x > 0 => Pos{x: miner.x - 1, y: miner.y },
                Direction::Right if miner.x < width - 1 => Pos{x: miner.x + 1, y: miner.y },
                Direction::Down if miner.y > 0 => Pos{x: miner.x, y: miner.y - 1 },
                Direction::Up if miner.y < height - 1 => Pos{x: miner.x, y: miner.y + 1 },
                _ => miner,
            };

            if next_pos != miner && cave_map[next_pos.x][next_pos.y] {
                path.push(direction_string(*direction));
                let solved = solve_dfs_helper(path, cave_map, next_pos, exit);
                if solved {
                    return true;
                } else {
                    path.pop();
                }
            }
        }

        // Default case: no paths from this miner position
        false
    }

    let mut path = vec![];
    let _ = solve_dfs_helper(&mut path, &mut cave_map.clone(), miner, exit);
    path
}

fn main() {
    let _ = solve_dfs(&BoolVec2D![[true, false], [false, false]], Pos{x: 0, y: 0}, Pos{x: 1, y: 1});
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_path() {
        assert_eq!(
            solve_dfs(&BoolVec2D![[true, false], [false, false]], Pos{x: 0, y: 0}, Pos{x: 1, y: 1}),
            vec!["";0]);
    }

    #[test]
    fn test_straight_path() {
        assert_eq!(
            solve_dfs(&BoolVec2D![[true, false], [true, false]], Pos{x: 0, y: 0}, Pos{x: 1, y: 0}),
            vec!["Right"]);
    }
}

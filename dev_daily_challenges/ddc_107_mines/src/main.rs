#[macro_export]
macro_rules! BoolVec2D {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
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
        Direction::Up => "Up",
        Direction::Right => "Right",
        Direction::Down => "Down",
    }
}

fn solve_dfs(cave_map: &CaveMap, miner: Pos, exit: Pos) -> StringVec {
    fn solve_dfs_helper(
        path: &mut StringVec,
        cave_map: &mut CaveMap,
        miner: Pos,
        exit: Pos,
    ) -> bool {
        const DIRECTIONS: [Direction; 4] = [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ];

        if miner == exit {
            return true;
        }

        cave_map[miner.x][miner.y] = false;

        let width = cave_map.len();
        let height = cave_map[0].len();

        for direction in &DIRECTIONS {
            let mut next_pos = miner;
            match direction {
                Direction::Left if miner.x > 0 => next_pos.x -= 1,
                Direction::Right if miner.x < width - 1 => next_pos.x += 1,
                Direction::Up if miner.y > 0 => next_pos.y -= 1,
                Direction::Down if miner.y < height - 1 => next_pos.y += 1,
                _ => continue,
            }

            if cave_map[next_pos.x][next_pos.y] {
                path.push(direction_string(*direction));
                if solve_dfs_helper(path, cave_map, next_pos, exit) {
                    return true;
                }
                path.pop();
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
    let _ = solve_dfs(
        &BoolVec2D![[true, false], [false, false]],
        Pos { x: 0, y: 0 },
        Pos { x: 1, y: 1 },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_path() {
        assert_eq!(
            solve_dfs(
                &BoolVec2D![[true, false], [false, false]],
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 1 }
            ),
            vec![""; 0]
        );
    }

    #[test]
    fn test_straight_path() {
        assert_eq!(
            solve_dfs(
                &BoolVec2D![[false, false], [true, true]],
                Pos { x: 1, y: 0 },
                Pos { x: 1, y: 1 }
            ),
            vec!["Down"]
        );
    }

    #[test]
    fn test_2x2_simple() {
        assert_eq!(
            solve_dfs(
                &BoolVec2D![[true, false], [true, true]],
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 1 }
            ),
            vec!["Right", "Down"]
        );
    }

    #[test]
    fn test_1x4_line() {
        assert_eq!(
            solve_dfs(
                &BoolVec2D![[true], [true], [true], [true]],
                Pos { x: 0, y: 0 },
                Pos { x: 3, y: 0 }
            ),
            vec!["Right", "Right", "Right"]
        );
    }

    #[test]
    fn test_3x3_walk_around() {
        assert_eq!(
            solve_dfs(
                &BoolVec2D![[true, true, true], [false, false, true], [true, true, true]],
                Pos { x: 0, y: 0 },
                Pos { x: 2, y: 0 }
            ),
            vec!["Down", "Down", "Right", "Right", "Up", "Up"]
        );
    }

    #[test]
    fn test_5x5_walk_around() {
        assert_eq!(
            solve_dfs(
                &BoolVec2D![
                    [true, true, false, false, false],
                    [false, true, true, false, false],
                    [false, false, true, true, false],
                    [false, false, false, true, true],
                    [false, false, false, false, true]
                ],
                Pos { x: 0, y: 0 },
                Pos { x: 4, y: 4 }
            ),
            vec!["Down", "Right", "Down", "Right", "Down", "Right", "Down", "Right"]
        );
    }
}

#[derive(Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

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

type CaveMap = Vec<Vec<bool>>;

fn solve_dfs(cave_map: &CaveMap, miner: Pos, exit: Pos) -> Vec<String> {
    /*
    fn solve_dfs_helper(cave_map: &mut CaveMap, miner: Pos, exit: Pos) -> Vec<String> {
    }

    let cave_map_copy: CaveMap<W, H> = cave_map;
    solve_dfs_helper(&mut cave_map_copy, miner, exit);
    */
    vec![]
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
            vec!["right"]);
    }
}

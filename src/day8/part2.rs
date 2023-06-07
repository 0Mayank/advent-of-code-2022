use crate::utils::read_lines;

#[derive(Debug)]
struct TreeMap {
    value: Vec<Vec<usize>>,
}

impl TreeMap {
    fn new() -> Self {
        Self { value: vec![] }
    }

    fn iter_column(&self, col: usize) -> impl Iterator<Item = &usize> + '_ {
        let n_cols = self.value[col].len();
        self.value.iter().flatten().skip(col).step_by(n_cols)
    }

    fn iter_row(&self, row: usize) -> impl Iterator<Item = &usize> + '_ {
        self.value[row].iter()
    }

    fn add_tree(&mut self, tree: usize) {
        if self.value.len() == 0 {
            self.new_row();
        }

        let row = self.value.len() - 1;
        self.value[row].push(tree);
    }

    fn new_row(&mut self) {
        self.value.push(vec![]);
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let given_tree_height = &self.value[row][col];
        let n_cols = self.value[row].len();
        let n_rows = self.value.len();

        let mut score_left = col;
        let mut score_right = n_cols - col - 1;
        let mut score_up = row;
        let mut score_down = n_rows - row - 1;

        for (i, tree_height) in self.iter_row(row).enumerate() {
            if i < col {
                if tree_height >= given_tree_height {
                    score_left = col - i;
                }
            } else if i > col {
                if tree_height >= given_tree_height {
                    score_right = i - col;
                    break;
                }
            }
        }

        for (i, tree_height) in self.iter_column(col).enumerate() {
            if i < row {
                if tree_height >= given_tree_height {
                    score_up = row - i;
                }
            } else if i > row {
                if tree_height >= given_tree_height {
                    score_down = i - row;
                    break;
                }
            }
        }

        score_left * score_right * score_up * score_down
    }

    fn highest_scenic_score(&self) -> usize {
        let mut max = 0;

        for row in 0..self.value.len() {
            for col in 0..self.value[0].len() {
                let scenic_score = self.scenic_score(row, col);
                if scenic_score > max {
                    max = scenic_score;
                }
            }
        }

        max
    }
}

pub fn run() -> String {
    let lines = read_lines("src/day8/puzzle_input.txt").unwrap();
    let mut trees_map = TreeMap::new();

    for line in lines {
        let line = line.unwrap();
        trees_map.new_row();
        for c in line.chars() {
            trees_map.add_tree(c.to_digit(10).unwrap() as usize);
        }
    }

    trees_map.highest_scenic_score().to_string()
}

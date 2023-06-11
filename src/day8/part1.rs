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

    fn is_visible(&self, row: usize, col: usize) -> bool {
        let given_tree_height = &self.value[row][col];
        let mut visible_from_left = true;
        let mut visible_from_right = true;
        let mut visible_from_up = true;
        let mut visible_from_down = true;

        for (i, tree_height) in self.iter_row(row).enumerate() {
            if tree_height >= given_tree_height {
                if i < col {
                    visible_from_left = false
                } else if i > col {
                    visible_from_right = false;
                    break;
                }
            }
        }

        for (i, tree_height) in self.iter_column(col).enumerate() {
            if tree_height >= given_tree_height {
                if i < row {
                    visible_from_up = false
                } else if i > row {
                    visible_from_down = false;
                    break;
                }
            }
        }

        return visible_from_left || visible_from_right || visible_from_up || visible_from_down;
    }

    fn visible_trees(&self) -> usize {
        let mut count = 0;

        for row in 0..self.value.len() {
            for col in 0..self.value[0].len() {
                if self.is_visible(row, col) {
                    count += 1;
                }
            }
        }

        count
    }
}

pub fn run() -> String {
    let lines = read_lines("8").unwrap();
    let mut trees_map = TreeMap::new();

    for line in lines {
        let line = line.unwrap();
        trees_map.new_row();
        for c in line.chars() {
            trees_map.add_tree(c.to_digit(10).unwrap() as usize);
        }
    }

    trees_map.visible_trees().to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(1796.to_string(), run());
    }
}

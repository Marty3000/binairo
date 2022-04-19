/// solves a binairo
use std::cmp::max;

const NEIGHBOR: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub struct Board {
    width: usize,
    field: Vec<Vec<u8>>,
    possible: Vec<Vec<Vec<u8>>>,
}

impl Board {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            field: vec![vec![0; width]; width],
            possible: vec![vec![vec![1, 2]; width]; width],
        }
    }

    pub fn init(&mut self, abc: &str) {
        let mut mabc = abc.chars().rev().collect::<String>();
        for y in 0..self.width {
            for x in 0..self.width {
                let val = max(47, mabc.pop().unwrap() as u8) - 47;
                if 0 < val {
                    self.set_field(x, y, val);
                }
            }
        }
    }

    pub fn solve(&mut self, num: usize) -> Vec<String> {
        let mut sol: Vec<String> = vec![];
        let mut obvi: bool = self.set_first_obvious();
        if !self.is_valid() {
            return sol;
        }
        while obvi {
            obvi = self.set_first_obvious();
            if !self.is_valid() {
                return sol;
            }
        }

        let (next_x, next_y) = self.get_first_guess();
        if next_x == self.width || next_y == self.width {
            if !self.is_valid() {
                return sol;
            }
            sol.push(self.print());
            return sol;
        }
        for next_val in &self.possible[next_y][next_x] {
            let mut nxt_move: Board = Board {
                width: self.width,
                field: self.field.clone(),
                possible: self.possible.clone(),
            };
            nxt_move.set_field(next_x, next_y, *next_val);
            if nxt_move.is_valid() {
                let mut nxt_sol = nxt_move.solve(num - sol.len());
                if !nxt_sol.is_empty() {
                    sol.append(&mut nxt_sol);
                    if num <= sol.len() {
                        return sol;
                    }
                }
            }
        }
        sol
    }


    fn set_field(&mut self, x: usize, y: usize, val: u8) {
        self.field[y][x] = val;
        self.set_pfeld(x, y, val);
    }

    fn set_pfeld(&mut self, x: usize, y: usize, val: u8) {
        self.possible[y][x] = vec![val];
        for (nx, ny) in NEIGHBOR {
            let y_tst: i8 = y as i8 + nx;
            let x_tst: i8 = x as i8 + ny;
            if valid_coord(&self.width, &x_tst, &y_tst) {
                let neigh_val = self.field[y_tst.abs() as usize][x_tst.abs() as usize];
                if neigh_val == val {
                    // println!("  Identical Neighbor found at {}, {}", x_tst, y_tst);
                    let y_two: i8 = y as i8 + 2 * nx;
                    let x_two: i8 = x as i8 + 2 * ny;
                    if valid_coord(&self.width, &x_two, &y_two) {
                        // println!("  Modify NeighborPoss found at {}, {}", x_two, y_two);
                        self.possible[y_two.abs() as usize][x_two.abs() as usize]
                            .retain(|e| e != &val);
                    }
                    let y_two: i8 = y as i8 - nx;
                    let x_two: i8 = x as i8 - ny;
                    if valid_coord(&self.width, &x_two, &y_two) {
                        // println!("  Modify NeighborPoss found at {}, {}", x_two, y_two);
                        self.possible[y_two.abs() as usize][x_two.abs() as usize]
                            .retain(|e| e != &val);
                    }
                } else if neigh_val == 0 {
                    let y_two: i8 = y as i8 + 2 * nx;
                    let x_two: i8 = x as i8 + 2 * ny;
                    if valid_coord(&self.width, &x_two, &y_two)
                        && self.field[y_two.abs() as usize][x_two.abs() as usize] == val
                    {
                        // println!("  Modify NeighborPoss found at {}, {}", x_tst, y_tst);
                        self.possible[y_tst.abs() as usize][x_tst.abs() as usize]
                            .retain(|e| e != &val);
                    }
                }
            }
        }
    }

    fn is_valid(&self) -> bool {
        let mut max_0 = 0;
        let mut max_1 = 0;
        for y in 0..self.width {
            let mut row_0: usize = 0;
            let mut row_1: usize = 0;
            let mut col_0: usize = 0;
            let mut col_1: usize = 0;
            for x in 0..self.width {
                if self.possible[y][x].is_empty() {
                    return false;
                } else {
                    match self.field[y][x] {
                        1 => row_0 += 1,
                        2 => row_1 += 1,
                        _ => (),
                    }
                    match self.field[x][y] {
                        1 => col_0 += 1,
                        2 => col_1 += 1,
                        _ => (),
                    }
                }
            }
            if (self.width + 1) / 2 < row_0
                || (self.width + 1) / 2 < row_1
                || (self.width + 1) / 2 < col_0
                || (self.width + 1) / 2 < col_1
            {
                return false;
            }
            if max_0 < max(row_0, col_0) {
                max_0 = max(row_0, col_0)
            }
            if max_1 < max(row_1, col_1) {
                max_1 = max(row_1, col_1)
            }
            if self.width < max_0 + max_1 {
                return false;
            }
        }
        true
    }

    fn print(&self) -> String {
        let mut abc: String = String::new();
        for y in 0..self.width {
            for x in 0..self.width {
                abc.push(char::from(self.field[y][x] + 47));
            }
        }
        abc
    }

    fn set_first_obvious(&mut self) -> bool {
        for y in 0..self.width {
            for x in 0..self.width {
                if self.field[y][x] == 0 && self.possible[y][x].len() == 1 {
                    self.set_field(x, y, self.possible[y][x][0]);
                    return true;
                }
            }
        }
        false
    }

    fn get_first_guess(&self) -> (usize, usize) {
        for y in 0..self.width {
            for x in 0..self.width {
                if self.possible[y][x].len() == 2 {
                    return (x, y);
                }
            }
        }
        (self.width, self.width)
    }
}

fn valid_coord(width: &usize, x: &i8, y: &i8) -> bool {
    0 <= *x && *x < *width as i8 && 0 <= *y && *y < *width as i8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4x4() {
        let mut test_board = Board {
            width: 4,
            field: vec![vec![0; 4]; 4],
            possible: vec![vec![vec![1, 2]; 4]; 4],
        };
        test_board.init(".1.0..0..0..11.0");
        let solu = test_board.solve(1);
        assert_eq!(vec![String::from("0110100100111100")], solu);
    }

    #[test]
    fn test_number_of_solutions() {
        let sol_num: usize = 3;
        let mut test_board = Board {
            width: 10,
            field: vec![vec![0; 10]; 10],
            possible: vec![vec![vec![1, 2]; 10]; 10],
        };
        test_board.init("......10...0..........1..1.1...010.0...1.0....0.1.....0..11...0...........01....11.0.1....1...1....0");
        let solu = test_board.solve(10);
        assert_eq!(sol_num, solu.len());
        let solu = test_board.solve(3);
        assert_eq!(sol_num, solu.len());
        let solu = test_board.solve(2);
        assert_eq!(2, solu.len());
        let solu = test_board.solve(1);
        assert_eq!(1, solu.len());
    }
}

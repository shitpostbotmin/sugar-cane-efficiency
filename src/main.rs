use core::panic;
use std::fmt::{Display, Formatter, Result};

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Sugar,
    Water,
    Air,
}

#[derive(Debug, Clone)]
struct Farm {
    tiles: Vec<Vec<Tile>>,
    size_x: usize,
    size_y: usize,
}

impl Farm {
    fn new_rect(size_x: usize, size_y: usize) -> Farm {
        let mut rows: Vec<Vec<Tile>> = vec![];

        let mut farm = Farm {
            tiles: rows,
            size_x,
            size_y,
        };

        farm.populate_with_air();

        farm.kill_sugar();

        farm
    }

    fn new_square(size: usize) -> Farm {
        Self::new_rect(size, size)
    }

    fn breed(a: &Farm, b: &Farm) -> Farm {
        let mut rng = rand::thread_rng();

        let mut rows: Vec<Vec<Tile>> = vec![];
        for x in 0..a.size_x {
            let mut row: Vec<Tile> = vec![];
            for y in 0..a.size_y {
                let a_tile = a.get_tile(x, y).unwrap();
                let b_tile = b.get_tile(x, y).unwrap();

                let mut new_tile = match a_tile {
                    Tile::Air => b_tile,
                    a_tile => a_tile,
                };

                row.push(new_tile);
            }

            rows.push(row);
        }

        let mut new_farm = Farm {
            tiles: rows,
            size_x: a.size_x,
            size_y: b.size_y,
        };

        new_farm.kill_sugar();

        new_farm
    }

    fn mutate(&mut self, mutation_factor: f32) {
        let mut rows: Vec<Vec<Tile>> = vec![];
        let mut rng = rand::thread_rng();

        for x in 0..self.size_x {
            let mut row: Vec<Tile> = vec![];
            for y in 0..self.size_y {
                let num: f32 = rng.gen();
                let tile = if num < mutation_factor {
                    let num: i32 = rng.gen_range(0..3);
                    match num {
                        0 => Tile::Sugar,
                        1 => Tile::Water,
                        2 => Tile::Air,
                        _ => panic!("invalid tile index"),
                    }
                } else {
                    self.tiles[x][y]
                };
                row.push(tile);
            }

            rows.push(row);
        }

        self.tiles = rows;
    }

    fn populate_with_air(&mut self) {
        let mut rng = rand::thread_rng();

        let mut rows: Vec<Vec<Tile>> = vec![];
        for _ in 0..self.size_x {
            let mut row: Vec<Tile> = vec![];
            for _ in 0..self.size_y {
                row.push(Tile::Air);
            }
            rows.push(row);
        }

        self.tiles = rows
    }

    fn kill_sugar(&mut self) {
        let mut rows: Vec<Vec<Tile>> = vec![];
        for x in 0..self.size_x {
            let mut row: Vec<Tile> = vec![];
            for y in 0..self.size_y {
                row.push(match self.get_tile(x, y) {
                    Some(Tile::Sugar) => {
                        if self.has_water_in_neighbourhood(x, y) {
                            Tile::Sugar
                        } else {
                            Tile::Air
                        }
                    }
                    tile => tile.unwrap(),
                })
            }
            rows.push(row);
        }

        self.tiles = rows;
    }

    fn has_water_in_neighbourhood(&self, x: usize, y: usize) -> bool {
        for tile in self.get_neighbours(x, y) {
            match tile {
                Tile::Water => return true,
                _ => {}
            }
        }

        false
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<Tile> {
        let top = if y == 0 {
            Tile::Air
        } else {
            self.get_tile(x, y - 1).unwrap_or(Tile::Air)
        };
        let right = self.get_tile(x + 1, y).unwrap_or(Tile::Air);
        let bottom = self.get_tile(x, y + 1).unwrap_or(Tile::Air);
        let left = if x == 0 {
            Tile::Air
        } else {
            self.get_tile(x - 1, y).unwrap_or(Tile::Air)
        };

        vec![top, right, bottom, left]
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.size_x || y >= self.size_y {
            return None;
        };

        Some(self.tiles[y][x])
    }

    fn get_sugar_score(&self) -> usize {
        let mut score: usize = 0;

        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Sugar => score += 100,
                    _ => (),
                }
            }
        }

        score
    }

    fn get_vertical_symmetry_score(&self) -> usize {
        let mut matched_tiles: usize = 0;

        for row in &self.tiles {
            let mut x = 0;
            for tile in row {
                let opposing_tile = row[self.size_x - 1 - x];
                if *tile == opposing_tile {
                    matched_tiles += 1;
                };
                x += 1;
            }
        }

        let total_tiles = self.size_x * self.size_y;
        let symmetry_factor = matched_tiles as f64 / total_tiles as f64;

        (symmetry_factor * 50.0) as usize
    }

    fn get_horizontal_symmetry_score(&self) -> usize {
        let mut matched_tiles: usize = 0;

        let mut y = 0;
        for row in &self.tiles {
            let mut x = 0;
            for tile in row {
                let opposing_tile = &self.tiles[self.size_y - 1 - y][x];
                if *tile == *opposing_tile {
                    matched_tiles += 1;
                };
                x += 1;
            }
            y += 1;
        }

        let total_tiles = self.size_x * self.size_y;
        let symmetry_factor = matched_tiles as f64 / total_tiles as f64;

        (symmetry_factor * 50.0) as usize
    }

    fn score(&self) -> usize {
        let mut score: usize = 0;

        score += self.get_sugar_score();

        score += self.get_vertical_symmetry_score();

        score += self.get_horizontal_symmetry_score();

        score
    }
}

impl Display for Farm {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        for row in &self.tiles {
            for tile in row {
                write!(
                    formatter,
                    "{}",
                    match tile {
                        Tile::Water => "~",
                        Tile::Sugar => "x",
                        Tile::Air => " ",
                    }
                )?;
            }
            writeln!(formatter, "")?;
        }
        Ok(())
    }
}

fn main() {
    let size = 4;
    let popluation = 500;
    let generations = size * 1000;
    let mutation_factor = 0.1;

    let mut farms: Vec<Farm> = vec![];
    for _ in 0..popluation {
        farms.push(Farm::new_square(size));
    }

    print_scores(&farms);

    for generation in 0..generations {
        farms.sort_by(|a, b| a.score().partial_cmp(&b.score()).unwrap());
        while farms.len() > popluation / 3 {
            farms.remove(0);
        }
        while farms.len() < popluation {
            let mut rng = rand::thread_rng();
            let mut new_farm = Farm::breed(
                &farms[rng.gen_range(0..(popluation / 3))],
                &farms[rng.gen_range(0..(popluation / 3))],
            );
            new_farm.mutate(mutation_factor);
            new_farm.kill_sugar();
            farms.push(new_farm);
        }
        if generation % (size * 10) == 0 {
            print!("gen #{}: ", generation + 1);
            print_scores(&farms);
        }
    }

    farms.sort_by(|a, b| a.score().partial_cmp(&b.score()).unwrap());
    print_scores(&farms);
    let winner = &farms[farms.len() - 1];
    println!("{}", winner);
    println!("{} from sugar", winner.get_sugar_score());
    println!("{} from x symm", winner.get_vertical_symmetry_score());
    println!("{} from y symm", winner.get_horizontal_symmetry_score());
}

fn print_scores(farms: &Vec<Farm>) {
    let scores = farms.into_iter().map(|farm| farm.score());
    let max = scores.clone().max().unwrap();
    let min = scores.clone().min().unwrap();
    let total: usize = scores.sum();
    let avg: usize = total / farms.len();

    let sugar_scores = farms.into_iter().map(|farm| farm.get_sugar_score());
    let sugar_total: usize = sugar_scores.sum();
    let sugar_avg: usize = sugar_total / farms.len();

    let symm_scores = farms
        .into_iter()
        .map(|farm| farm.get_horizontal_symmetry_score() + farm.get_vertical_symmetry_score());
    let symm_total: usize = symm_scores.sum();
    let symm_avg: usize = symm_total / farms.len();

    println!(
        "MAX: {}, MIN: {}, AVG: {} (SUGAR AVG: {}, SYMM AVG: {})",
        max, min, avg, sugar_avg, symm_avg
    );
}

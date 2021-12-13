use std::io::{self};

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Fold {
    axis: char,
    pos: usize,
}

#[derive(Debug)]
struct Paper {
    max_x: usize,
    max_y: usize,
    board: Vec<Vec<char>>,
}

impl Paper {
    fn show(&self) {
        for x in 0..self.max_x {
            println!("{}", self.board[x][..self.max_y].iter().collect::<String>())
        }
    }
    fn new(max_x: usize, max_y: usize) -> Paper {
        let board = vec![vec!['.'; max_y]; max_x];
        Paper {
            max_x,
            max_y,
            board,
        }
    }
    fn mark(&mut self, coord: &Coord) {
        let Coord { x, y } = *coord;
        self.board[x][y] = '#';
    }
    fn fold(&mut self, fold: &Fold) {
        let Fold { axis, pos } = *fold;
        match axis {
            'x' => {
                for x in 0..self.max_x {
                    for y in 0..=pos {
                        if pos + y < self.max_y && self.board[x][pos + y] == '#' {
                            self.board[x][pos - y] = self.board[x][pos + y];
                        }
                    }
                }
                self.max_y = pos;
            }
            'y' => {
                for x in 0..=pos {
                    for y in 0..self.max_y {
                        if pos + x < self.max_x && self.board[pos + x][y] == '#' {
                            self.board[pos - x][y] = self.board[pos + x][y];
                        }
                    }
                }
                self.max_x = pos;
            }
            _ => panic!("Wrong axis {}", axis),
        }
    }
    fn count_dots(&self) -> usize {
        let mut output = 0;
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                if self.board[x][y] == '#' {
                    output += 1;
                }
            }
        }
        output
    }
}

fn parse_input(file_content: &Vec<String>) -> (Vec<Coord>, Vec<Fold>) {
    let index = file_content.iter().position(|x| x.is_empty()).unwrap();
    let mut coords = vec![];
    for line in file_content[0..index].iter() {
        let splitted = line.split(",").collect::<Vec<_>>();
        let x = splitted[1].parse::<_>().unwrap();
        let y = splitted[0].parse::<_>().unwrap();
        coords.push(Coord { x, y });
    }
    let mut folds = vec![];
    for line in file_content[index + 1..].iter() {
        let splitted = line.split("=").collect::<Vec<_>>();
        let axis = splitted[0].chars().last().unwrap();
        let pos = splitted[1].parse::<_>().unwrap();
        folds.push(Fold { axis, pos })
    }
    (coords, folds)
}

fn prepare_paper(coords: &Vec<Coord>) -> Paper {
    let max_x = *coords.iter().map(|Coord { x, y: _ }| x).max().unwrap() + 1;
    let max_y = *coords.iter().map(|Coord { x: _, y }| y).max().unwrap() + 1;
    let mut paper = Paper::new(max_x, max_y);
    for coord in coords.iter() {
        paper.mark(coord);
    }
    paper
}

fn part_1(file_content: &Vec<String>) -> usize {
    let (coords, folds) = parse_input(&file_content);
    let mut paper = prepare_paper(&coords);
    paper.fold(&folds[0]);
    paper.count_dots()
}

fn part_2(file_content: &Vec<String>) {
    let (coords, folds) = parse_input(&file_content);
    let mut paper = prepare_paper(&coords);
    for fold in folds.iter() {
        paper.fold(fold);
    }
    paper.show();
}

fn main() -> io::Result<()> {
    // O; ZUJUAFHP
    let files_results = vec![("test.txt", 17), ("input.txt", 737)];
    for (f, result_1) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let res_1 = part_1(&file_content);
        assert_eq!(res_1, result_1);

        part_2(&file_content);
    }
    Ok(())
}

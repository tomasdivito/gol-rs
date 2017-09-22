extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, Button, Grid, WindowType};
use std::io::stdin;
use std::cmp::Ordering;


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq (&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct GameBoard {
    columns: i32,
    rows: i32,
    generation: i32,
    subjects: Vec<Point>,
}

impl GameBoard {
    pub fn new (columns: i32, rows: i32) -> GameBoard {
        GameBoard {
            columns: columns,
            rows: rows,
            generation: 0,
            subjects: Vec::new(),
        }
    }

    fn check_for_newborns (&self) -> Vec<Point> {
        let mut newborns = Vec::new();

        for i in 0..self.columns {
            for j in 0..self.rows {
                match self.subjects
                    .iter()
                    .find(|subj| subj == &&Point {x: i, y: j}) {
                        None => {
                            let mut neighbours = 0;
            
                            // top-left
                            match self.subjects.iter()
                                .find(|s| s.x == i - 1 && s.x == j - 1) {
                                    None => continue,
                                    _ => neighbours += 1,
                                }
                            // top
                            match self.subjects.iter()
                                .find(|s| s.x == i && s.x == j - 1) {
                                    None => continue,
                                    _ => neighbours += 1,
                                }
                            // top-right
                            match self.subjects.iter()
                                .find(|s| s.x == i + 1 && s.x == j - 1) {
                                    None => continue,
                                    _ => neighbours += 1,
                                }
                            // left
                            match self.subjects.iter()
                                .find(|s| s.x == i - 1 && s.x == j) {
                                    None => continue,
                                    _ => neighbours += 1,
                                }
                            // right
                            match self.subjects.iter()
                                .find(|s| s.x == i + 1 && s.x == j) {
                                    None => continue,
                                    _ => neighbours += 1,
                                }
                            // bottom left
                            match self.subjects.iter()
                                .find(|s| s.x == i - 1 && s.x == j + 1) {
                                    None => continue,
                                    _ => neighbours += 1,
                                }
                            // bottom
                            match self.subjects.iter()
                                .find(|s| s.x == i && s.x == j + 1) {
                                    None => continue,
                                    _ => neighbours += 1,
                                }
                            // bottom-right
                            match self.subjects.iter()
                                .find(|s| s.x == i + 1 && s.x == j + 1) {
                                    None => continue,
                                    _ => neighbours += 1,
                                }
                            if neighbours >= 3 {
                                newborns.push(Point {x: i, y: j});
                            }
                        },
                        _ => continue,
                    }
            }
        }

        return newborns;
    }

    fn check_for_deaths (&mut self) -> Vec<i32> {
        let mut dead_subjects_index = Vec::new();

        for subj in &self.subjects {
            // check around the subject to see if it should die
            // TODO(totii) do this in a more _cool_ way ;)
            let mut neighbours = 0;
            
            // top-left
            match self.subjects.iter()
                .find(|s| s.x == subj.x - 1 && s.x == subj.y - 1) {
                    None => continue,
                    _ => neighbours += 1,
                }
            // top
            match self.subjects.iter()
                .find(|s| s.x == subj.x && s.x == subj.y - 1) {
                    None => continue,
                    _ => neighbours += 1,
                }
            // top-right
            match self.subjects.iter()
                .find(|s| s.x == subj.x + 1 && s.x == subj.y - 1) {
                    None => continue,
                    _ => neighbours += 1,
                }
            // left
            match self.subjects.iter()
                .find(|s| s.x == subj.x - 1 && s.x == subj.y) {
                    None => continue,
                    _ => neighbours += 1,
                }
            // right
            match self.subjects.iter()
                .find(|s| s.x == subj.x + 1 && s.x == subj.y) {
                    None => continue,
                    _ => neighbours += 1,
                }
            // bottom left
            match self.subjects.iter()
                .find(|s| s.x == subj.x - 1 && s.x == subj.y + 1) {
                    None => continue,
                    _ => neighbours += 1,
                }
            // bottom
            match self.subjects.iter()
                .find(|s| s.x == subj.x && s.x == subj.y + 1) {
                    None => continue,
                    _ => neighbours += 1,
                }
            // bottom-right
            match self.subjects.iter()
                .find(|s| s.x == subj.x + 1 && s.x == subj.y + 1) {
                    None => continue,
                    _ => neighbours += 1,
                }
            
            if 1 <= neighbours || neighbours >= 4 {
                // Die.
                let index = self.subjects.iter().position(|x| x == subj).unwrap();
                dead_subjects_index.push(index as i32);
            }
        }

        return dead_subjects_index;
    }

    fn draw_board (&mut self) {
        for i in 0..self.columns {
            let mut col = String::new();
            for j in 0..self.rows {
                match self.subjects
                    .iter()
                    .find(|subj| subj == &&Point {x: i, y: j}) {
                        None => col.push_str(" 0 "),
                        _ => col.push_str(" 1 "),
                    }
            }
            println!("{}", col);
        }
        self.generation += 1;
    }
}

fn main () {
    // GTK initialization.
    gtk::init().unwrap();

    // Create the main window.
    let WIDTH = 640;
    let HEIGHT = 480;
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Game Of Life - Tomas Di Vito");
    window.resize(WIDTH, HEIGHT);

    // UI initialization should be here
    let grid = Grid::new();
     for i in 0..50 {
        for j in 0..50 {
            let button = Button::new();
            grid.attach(&button, i, j, 1, 1);
        }
    }

    // End of UI.
    window.add(&grid);
    window.show_all();

    // Handle closing of the window.
    window.connect_delete_event(|_, _| {
        // Stop the main loop.
        gtk::main_quit();
        // Let the default handler destroy the window.
        Inhibit(false)
    });

    // Run the main loop.
    gtk::main();

    let mut board = GameBoard::new(50, 50);

    for i in 18..30 {
        board.subjects.push(Point {
            x: i,
            y: i,
        })
    }

    loop {
        // board.draw_board();
        let new_borns = board.check_for_newborns();
        let dead_subjects = board.check_for_deaths();

        for i in 0..dead_subjects.len() {
            board.subjects.remove(dead_subjects[i] as usize);
        }

        for i in 0..new_borns.len() {
            board.subjects.push(Point {
                x: new_borns[i].x,
                y: new_borns[i].y,
            })
        }

        for i in 0..board.subjects.len() {
            println!("{:?}", board.subjects[i]);
        }
    }
}
use crate::{Tetrimino, TetriminoI};
extern crate rand;

struct Tetris {
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Tetrimino>,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        Tetris {
            game_map: game_map,
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }

    pub fn create_new_tetrimino(&self) -> Tetrimino {
        static mut PREV: u8 = 7;
        let mut rand_nb = rand::random::<u8>() % 7;

        if unsafe { PREV } == rand_nb {
            rand_nb = rand::random::<u8>() % 7;
        }

        unsafe { PREV == rand_nb; }

        match rand_nb {
            0 => TetriminoI::new(),
            1 => TetriminoI::new(),
            2 => TetriminoI::new(),
            3 => TetriminoI::new(),
            4 => TetriminoI::new(),
            5 => TetriminoI::new(),
            6 => TetriminoI::new(),
            _ => TetriminoI::new(),
        }
    }
}
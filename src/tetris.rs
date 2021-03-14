use rand::random;

#[derive(Copy, Clone)]
pub enum SpaceState {
    Empty,
    FallingTetromino,
    SettledTetromino((f32, f32, f32)),
}

pub struct PlaySpace {
    space: [[SpaceState; 22]; 10],
    score: i32,
    lines_cleared: i32,
    pub(crate) color: (f32, f32, f32),
    current_tetromino: Tetromino,
    current_tetromino_rotation: usize,
    falling_position: (usize, usize),
    // top left block
    time_since_movement: i32,
}

impl PlaySpace {
    pub fn initialize() -> PlaySpace {
        let default = PlaySpace {
            space: [[SpaceState::Empty; 22]; 10],
            score: 0,
            lines_cleared: 0,
            color: (1.0, 1.0, 1.0),
            current_tetromino: Tetromino::random(),
            current_tetromino_rotation: 0,
            falling_position: (4, 21),
            time_since_movement: 0,
        };
        return default;
    }

    pub fn tick(&mut self) {
        if self.time_since_movement > 10 {
            if self.can_fall() {
                self.falling_position.1 -= 1;
            } else {
                self.space = self.space_with_falling_as_settled();
            }
            self.time_since_movement = 0;
        } else {
            self.time_since_movement += 1;
        }
    }

    pub fn space_with_falling_as_settled(&self) -> [[SpaceState; 22]; 10] {
        let mut new_space = self.space.clone();
        for i in 0..4 {
            for j in 0..4 {
                if let SpaceState::FallingTetromino =
                self.current_tetromino.map[self.current_tetromino_rotation][i][j]
                {
                    new_space[self.falling_position.0 + j][self.falling_position.1 - i] =
                        SpaceState::SettledTetromino(self.current_tetromino.color);
                }
            }
        }
        return new_space;
    }

    fn can_fall(&self) -> bool {
        let mut lowest_in_col = [usize::MAX; 4];
        for i in 0..4 {
            for j in 0..4 {
                if let SpaceState::FallingTetromino =
                self.current_tetromino.map[self.current_tetromino_rotation][i][j]
                {
                    if i < lowest_in_col[j] {
                        lowest_in_col[j] = i;
                    }
                }
            }
        }
        for i in 0..4 {
            let x_test = self.falling_position.0 + i;
            if x_test > 21 {
                continue;
            }
            if lowest_in_col[i] == usize::MAX { continue; }
            if self.falling_position.1 - lowest_in_col[i] == 0 { return false; }
            let y_test = self.falling_position.1 - lowest_in_col[i] - 1;
            match self.space[x_test][y_test] {
                SpaceState::Empty => {}
                _ => {
                    return false;
                }
            }
        }
        return true;
    }
}

struct Tetromino {
    color: (f32, f32, f32),
    map: [[[SpaceState; 4]; 4]; 4], // [rotation][x][y]
}

impl Tetromino {
    pub fn random() -> Tetromino {
        let mut r: f64 = random();
        r = r * 7.0;
        if r <= 1.0 {
            return Tetromino::i();
        } else if r <= 2.0 {
            return Tetromino::o();
        } else if r <= 3.0 {
            return Tetromino::t();
        } else if r <= 4.0 {
            return Tetromino::s();
        } else if r <= 5.0 {
            return Tetromino::z();
        } else if r <= 6.0 {
            return Tetromino::j();
        } else {
            return Tetromino::l();
        }
    }

    fn i() -> Tetromino {
        let i = Tetromino {
            color: (0.0, 1.0, 1.0),
            map: [
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
            ],
        };
        return i;
    }
    fn o() -> Tetromino {
        let o = Tetromino {
            color: (1.0, 1.0, 0.0),
            map: [
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
            ],
        };
        return o;
    }
    fn t() -> Tetromino {
        let t = Tetromino {
            color: (0.5, 0.0, 0.5),
            map: [
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
            ],
        };
        return t;
    }
    fn s() -> Tetromino {
        let s = Tetromino {
            color: (0.0, 1.0, 0.0),
            map: [
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
            ],
        };
        return s;
    }
    fn z() -> Tetromino {
        let z = Tetromino {
            color: (1.0, 0.0, 0.0),
            map: [
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
            ],
        };
        return z;
    }
    fn j() -> Tetromino {
        let j = Tetromino {
            color: (0.0, 0.0, 1.0),
            map: [
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
            ],
        };
        return j;
    }
    fn l() -> Tetromino {
        let l = Tetromino {
            color: (1.0, 0.5, 0.0),
            map: [
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
                [
                    [
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::FallingTetromino,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                    [
                        SpaceState::Empty,
                        SpaceState::FallingTetromino,
                        SpaceState::Empty,
                        SpaceState::Empty,
                    ],
                ],
            ],
        };
        return l;
    }
}

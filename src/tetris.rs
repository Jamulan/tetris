use rand::random;

#[derive(Copy, Clone)]
pub enum SpaceState {
    Empty,
    FallingTetromino,
    SettledTetromino((f32, f32, f32)),
}

pub enum Action {
    None,
    MoveLeft,
    MoveRight,
    RotateClockwise,
    RotateCounterClockwise,
}

pub struct PlaySpace {
    space: [[SpaceState; 22]; 10],
    score: i32,
    lines_cleared: i32,
    pub(crate) color: (f32, f32, f32),
    current_tetromino: Tetromino,
    current_tetromino_rotation: usize,
    // top left block
    falling_position: (i32, usize),
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

    pub fn tick(&mut self, action: Action) {
        let mut moved = false;
        match action {
            Action::MoveLeft => {
                let mut can_move_left = true;
                if self.falling_position.0 == -1 {
                    can_move_left = false;
                }
                for i in 0..4 {
                    for j in 0..4 {
                        if let SpaceState::FallingTetromino =
                            self.current_tetromino.map[self.current_tetromino_rotation][i][j]
                        {
                            if self.falling_position.0 as usize + j == 0 {
                                can_move_left = false;
                                break;
                            }
                            if let SpaceState::SettledTetromino(_) = self.space
                                [self.falling_position.0 as usize + j - 1]
                                [self.falling_position.1 - i]
                            {
                                can_move_left = false;
                                break;
                            }
                        }
                    }
                    if !can_move_left {
                        break;
                    }
                }
                if can_move_left {
                    self.falling_position.0 = self.falling_position.0 - 1;
                    moved = true;
                }
            }
            Action::MoveRight => {
                let mut can_move_right = true;
                for i in 0..4 {
                    for j in 0..4 {
                        if let SpaceState::FallingTetromino =
                        self.current_tetromino.map[self.current_tetromino_rotation][i][j]
                        {
                            if (self.falling_position.0 as usize + j) as i32 == 9 {
                                can_move_right = false;
                                break;
                            }
                            if let SpaceState::SettledTetromino(_) = self.space
                                [self.falling_position.0 as usize + j + 1]
                                [self.falling_position.1 - i]
                            {
                                can_move_right = false;
                                break;
                            }
                        }
                    }
                    if !can_move_right {
                        break;
                    }
                }
                if can_move_right {
                    self.falling_position.0 = self.falling_position.0 + 1;
                    moved = true;
                }
            }
            Action::RotateClockwise => {
                self.current_tetromino_rotation = (self.current_tetromino_rotation + 1) % 4;
                for i in 0..4 {
                    for j in 0..4 {
                        if let SpaceState::FallingTetromino =
                        self.current_tetromino.map[self.current_tetromino_rotation][i][j]
                        {
                            // TODO
                        }
                    }
                }
            }
            _ => {}
        }
        let can_fall = self.can_fall();
        if !can_fall && moved {
            self.time_since_movement = -1;
        }
        if self.time_since_movement > 10 {
            if can_fall {
                self.falling_position.1 -= 1;
            } else {
                self.space = self.space_with_falling_as_settled();
                self.current_tetromino = Tetromino::random();
                self.current_tetromino_rotation = 0;
                self.falling_position = (4, 21);
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
                    new_space[(self.falling_position.0 + j as i32) as usize]
                        [self.falling_position.1 - i] =
                        SpaceState::SettledTetromino(self.current_tetromino.color);
                }
            }
        }
        return new_space;
    }

    fn can_fall(&self) -> bool {
        let mut lowest_in_col = [0; 4];
        for i in 0..4 {
            for j in 0..4 {
                if let SpaceState::FallingTetromino =
                self.current_tetromino.map[self.current_tetromino_rotation][i][j]
                {
                    if i > lowest_in_col[j] {
                        lowest_in_col[j] = i;
                    }
                }
            }
        }
        for i in 0..4 {
            let x_test = (self.falling_position.0 + i as i32) as usize;
            if x_test > 9 {
                continue;
            }
            if self.falling_position.1 - lowest_in_col[i] == 0 {
                return false;
            }
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

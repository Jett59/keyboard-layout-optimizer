#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyCode {
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyboardLayout {
    pub top_row: [KeyCode; 10],
    pub middle_row: [KeyCode; 10],
    pub bottom_row: [KeyCode; 10],
}

impl Default for KeyboardLayout {
    fn default() -> Self {
        Self::QWERTY
    }
}

impl KeyboardLayout {
    pub const QWERTY: KeyboardLayout = KeyboardLayout {
        top_row: [
            KeyCode::Q,
            KeyCode::W,
            KeyCode::E,
            KeyCode::R,
            KeyCode::T,
            KeyCode::Y,
            KeyCode::U,
            KeyCode::I,
            KeyCode::O,
            KeyCode::P,
        ],
        middle_row: [
            KeyCode::A,
            KeyCode::S,
            KeyCode::D,
            KeyCode::F,
            KeyCode::G,
            KeyCode::H,
            KeyCode::J,
            KeyCode::K,
            KeyCode::L,
            KeyCode::Semicolon,
        ],
        bottom_row: [
            KeyCode::Z,
            KeyCode::X,
            KeyCode::C,
            KeyCode::V,
            KeyCode::B,
            KeyCode::N,
            KeyCode::M,
            KeyCode::Comma,
            KeyCode::Dot,
            KeyCode::Slash,
        ],
    };

    pub fn position_of(&self, key_code: KeyCode) -> Option<(usize, usize)> {
        for (row_index, row) in [self.top_row, self.middle_row, self.bottom_row]
            .iter()
            .enumerate()
        {
            for (column_index, &code) in row.iter().enumerate() {
                if code == key_code {
                    return Some((row_index, column_index));
                }
            }
        }
        None
    }

    pub fn key_at(&self, position: (usize, usize)) -> KeyCode {
        let (row_index, column_index) = position;
        match row_index {
            0 => self.top_row[column_index],
            1 => self.middle_row[column_index],
            2 => self.bottom_row[column_index],
            _ => panic!("Invalid row index"),
        }
    }

    pub fn set_key_at(&mut self, row_index: usize, column_index: usize, key_code: KeyCode) {
        match row_index {
            0 => self.top_row[column_index] = key_code,
            1 => self.middle_row[column_index] = key_code,
            2 => self.bottom_row[column_index] = key_code,
            _ => panic!("Invalid row index"),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = KeyCode> + '_ {
        self.top_row
            .iter()
            .copied()
            .chain(self.middle_row.iter().copied())
            .chain(self.bottom_row.iter().copied())
    }
}

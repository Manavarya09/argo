use alacritty_terminal::Term;
use alacritty_terminal::event::VoidListener;
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Column, Line, Point};
use alacritty_terminal::term::Config;
use alacritty_terminal::vte::ansi::Processor;

#[derive(Debug, Clone, Copy)]
struct Size {
    cols: usize,
    rows: usize,
    history: usize,
}

impl Dimensions for Size {
    fn columns(&self) -> usize {
        self.cols
    }
    fn screen_lines(&self) -> usize {
        self.rows
    }
    fn total_lines(&self) -> usize {
        self.rows + self.history
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: (0xe8, 0xe6, 0xe3),
            bg: (0x0a, 0x0a, 0x0c),
        }
    }
}

pub struct TerminalState {
    term: Term<VoidListener>,
    parser: Processor,
    cols: u16,
    rows: u16,
}

impl TerminalState {
    pub fn new(cols: u16, rows: u16) -> Self {
        let size = Size {
            cols: cols as usize,
            rows: rows as usize,
            history: 10_000,
        };
        let term = Term::new(Config::default(), &size, VoidListener);
        Self {
            term,
            parser: Processor::new(),
            cols,
            rows,
        }
    }

    pub fn feed(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.parser.advance(&mut self.term, *byte);
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        let size = Size {
            cols: cols as usize,
            rows: rows as usize,
            history: 10_000,
        };
        self.term.resize(size);
        self.cols = cols;
        self.rows = rows;
    }

    pub fn visible_cells(&self) -> Vec<Vec<Cell>> {
        let grid = self.term.grid();
        let mut out = Vec::with_capacity(self.rows as usize);
        for line in 0..self.rows as i32 {
            let mut row = Vec::with_capacity(self.cols as usize);
            for col in 0..self.cols as usize {
                let p = Point::new(Line(line), Column(col));
                let cell = &grid[p];
                let ch = cell.c;
                row.push(Cell { ch, ..Cell::default() });
            }
            out.push(row);
        }
        out
    }

    pub fn cursor_position(&self) -> (u16, u16) {
        let p = self.term.grid().cursor.point;
        (p.column.0 as u16, p.line.0 as u16)
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.cols, self.rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state_has_empty_grid() {
        let state = TerminalState::new(80, 24);
        let cells = state.visible_cells();
        // 24 rows, 80 cols
        assert_eq!(cells.len(), 24);
        assert_eq!(cells[0].len(), 80);
        // every cell is whitespace
        for row in cells.iter() {
            for cell in row.iter() {
                assert_eq!(cell.ch, ' ');
            }
        }
    }

    #[test]
    fn feeding_text_updates_cells() {
        let mut state = TerminalState::new(80, 24);
        state.feed(b"hello");
        let cells = state.visible_cells();
        assert_eq!(cells[0][0].ch, 'h');
        assert_eq!(cells[0][1].ch, 'e');
        assert_eq!(cells[0][2].ch, 'l');
        assert_eq!(cells[0][3].ch, 'l');
        assert_eq!(cells[0][4].ch, 'o');
    }

    #[test]
    fn newline_advances_cursor() {
        let mut state = TerminalState::new(80, 24);
        state.feed(b"a\r\nb");
        let cells = state.visible_cells();
        assert_eq!(cells[0][0].ch, 'a');
        assert_eq!(cells[1][0].ch, 'b');
    }

    #[test]
    fn resize_changes_grid_dims() {
        let mut state = TerminalState::new(80, 24);
        state.resize(120, 40);
        let cells = state.visible_cells();
        assert_eq!(cells.len(), 40);
        assert_eq!(cells[0].len(), 120);
    }
}

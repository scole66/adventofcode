use ahash::AHashMap;
use anyhow::anyhow;
use std::io;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
struct Position {
    row: u32,
    column: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Default, Copy, Clone)]
struct Identifier(u32);

#[derive(Debug, Default)]
struct SquareState {
    marked: bool,
    id: Identifier,
}

#[derive(Debug, Default)]
struct Board {
    winning_call: Option<Identifier>,
    squares: [SquareState; 25],
    by_id: AHashMap<Identifier, Position>,
}

impl Index<&Position> for Board {
    type Output = SquareState;
    fn index(&self, index: &Position) -> &Self::Output {
        let Position { row, column } = index;
        &self.squares[(row * 5 + column) as usize]
    }
}

impl IndexMut<&Position> for Board {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        let Position { row, column } = index;
        &mut self.squares[(row * 5 + column) as usize]
    }
}

impl Board {
    pub fn new(identifiers: &[u32]) -> Self {
        assert!(identifiers.len() >= 25);
        let mut b = Board::default();
        let mut ids = identifiers.iter();
        for row in 0..5 {
            for column in 0..5 {
                let id = Identifier(*ids.next().unwrap());
                let loc = Position { row, column };
                b[&loc].id = id;
                b.by_id.insert(id, loc);
            }
        }

        b
    }

    pub fn mark(&mut self, id: Identifier) -> bool {
        // Don't mark boards which have already won
        if self.winning_call.is_some() {
            return false;
        }

        let maybe_location = self.by_id.get(&id).cloned();
        if let Some(loc) = maybe_location {
            // Mark this location
            self[&loc].marked = true;

            // See if this board is now a winner
            if (0..5).all(|r| {
                let scan = Position {
                    row: r,
                    column: loc.column,
                };
                self[&scan].marked
            }) || (0..5).all(|c| {
                let scan = Position {
                    row: loc.row,
                    column: c,
                };
                self[&scan].marked
            }) {
                self.winning_call = Some(id);
                return true;
            }
        }
        false
    }

    pub fn tally(&self) -> u32 {
        match &self.winning_call {
            None => 0,
            Some(final_call) => {
                let unmarked_square_sum: u32 = self.squares.iter().filter(|s| !s.marked).map(|s| s.id.0).sum();
                unmarked_square_sum * final_call.0
            }
        }
    }
}

fn process_input(lines: &[String]) -> Result<(Vec<Identifier>, Vec<Board>), anyhow::Error> {
    let mut input = lines.iter();
    let guess_line = input.next().ok_or_else(|| anyhow!("truncated input"))?;
    let guesses: Vec<Identifier> = guess_line
        .split(',')
        .filter_map(|n| n.parse::<u32>().ok())
        .map(Identifier)
        .collect();

    let mut boards: Vec<Board> = vec![];
    loop {
        let blank_line = input.next();
        if blank_line.is_none() {
            break;
        }
        let maybe_rows: Vec<Option<Vec<u32>>> = (0..5)
            .map(|_| {
                input
                    .next()
                    .map(|s| s.split_whitespace().filter_map(|n| n.parse::<u32>().ok()).collect())
            })
            .collect();
        if maybe_rows.iter().any(|row| row.is_none()) {
            break;
        }
        let ident_sequence: Vec<u32> = maybe_rows
            .into_iter()
            .map(|maybe_row| maybe_row.unwrap())
            .collect::<Vec<_>>()
            .concat();
        boards.push(Board::new(&ident_sequence));
    }

    Ok((guesses, boards))
}

fn play_games(boards: &mut [Board], guesses: Vec<Identifier>) -> Option<(&Board, &Board)> {
    let mut winners: Vec<usize> = Vec::with_capacity(boards.len());
    for guess in guesses {
        for (idx, board) in boards.iter_mut().enumerate() {
            let won = board.mark(guess);
            if won {
                winners.push(idx);
            }
        }
        if winners.len() == boards.len() {
            return Some((&boards[winners[0]], &boards[winners[winners.len() - 1]]));
        }
    }
    None
}

fn main() -> Result<(), anyhow::Error> {
    let mut lines = Vec::<String>::new();

    loop {
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let s = buffer.trim();
        lines.push(s.to_string());
    }

    let (guesses, mut boards) = process_input(&lines)?;

    let (winner, loser) =
        play_games(&mut boards, guesses).ok_or_else(|| anyhow!("No winners at the end of the round!"))?;

    println!("Part1: Score: {}", winner.tally());
    println!("Part2: Score: {}", loser.tally());

    Ok(())
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use test_case::test_case;
//}

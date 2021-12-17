use std::fs;
type Row<T> = Vec<T>;
type Board<T> = Vec<Row<T>>;

fn main() {
    let contents = fs::read_to_string("./input").expect("Something went wrong with this file");
    //let contents = fs::read_to_string("./example").expect("Something went wrong with this file");

    let input: Vec<&str> = contents
        .split_terminator("\n")
        .filter(|x| !x.is_empty())
        .collect();
    let (n, b) = input.split_at(1);
    let numbers: Vec<u32> = n[0]
        .split_terminator(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards: Vec<(Board<u32>, bool)> = b
        .chunks(5)
        .map(|y| {
            (
                y.iter()
                    .map(|x| {
                        x.split_whitespace()
                            .map(|z| z.parse::<u32>().unwrap())
                            .collect::<Row<u32>>()
                    })
                    .collect::<Board<u32>>(),
                false,
            )
        })
        .collect();

    'outer: for i in 0..numbers.len() {
        let drawn = &numbers[0..i].to_owned();
        for (board, solved) in boards.iter_mut() {
            *solved = bingo(&board, drawn);
            if (!*solved) {
                //println!("{:?}", board);
                //println!();
            }
            //            if bingo(&board, drawn) {
            //                let unmarked_sum: u32 = board
            //                    .iter()
            //                    .flat_map(|y| y.iter().filter(|&x| !drawn.contains(x)))
            //                    .sum();
            //                println!("{}", unmarked_sum * drawn.last().unwrap());
            //                break 'outer;
            //            }
        }
        let c: Vec<&(Board<u32>, bool)> = boards.iter().filter(|x| !x.1).collect();
        if c.iter().count() == 1 {
            let unmarked_sum: u32 = *c[0]
                .iter()
                .flat_map(|y| y.iter().filter(|&x| !drawn.contains(x)))
                .sum();
            println!("{:?}", c);
            break;
        }
    }
}

fn bingo<T>(board: &Board<T>, numbers: &Vec<T>) -> bool
where
    T: PartialEq,
    T: Copy,
{
    board.iter().any(|row| check_row(row, numbers))
        || columns(board).iter().any(|row| check_row(row, numbers))
}
fn check_row<T>(row: &Row<T>, numbers: &Vec<T>) -> bool
where
    T: PartialEq,
{
    row.iter().all(|x| numbers.contains(x))
}

fn columns<T>(board: &Board<T>) -> Board<T>
where
    T: Copy,
{
    let mut columns = vec![Vec::with_capacity(board.len()); board[0].len()];
    for row in board {
        for i in 0..row.len() {
            columns[i].push(row[i]);
        }
    }
    columns
}

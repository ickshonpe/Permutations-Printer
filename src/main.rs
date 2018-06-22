#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn swap_direction(self) -> Self {
        use Direction;
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right
        }
    }
}

fn factorial(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        n => (1..n + 1).product(),
    }
}

fn find_largest_mobile(p: &[usize], mobile: usize) -> usize {
    for (i, &v) in p.iter().enumerate() {
        if v == mobile {
            return i;
        }
    }
    0
}

fn get_mobile(p: &[usize], dirs: &[Direction]) -> usize {
    use Direction::*;
    let mut mobile = 0;
    let mut previous_mobile = 0;
    for i in 0..p.len() {
        if dirs[p[i]] == Left && i != 0
            && p[i - 1] < p[i] && previous_mobile < p[i] {
            mobile = p[i];
            previous_mobile = mobile;
        }

        if dirs[p[i]] == Right && i != p.len() - 1
            && p[i + 1] < p[i] && previous_mobile < p[i] {
            mobile = p[i];
            previous_mobile = mobile;
        }
    }
    if mobile == 0 && previous_mobile == 0 { 0 } else { mobile }
}

fn find_next_permutation(p: &mut [usize], dirs: &mut [Direction]) {
    use Direction::*;
    let mobile = get_mobile(p, dirs);
    let pos = find_largest_mobile(p, mobile);
    match dirs[p[pos]] {
        Left => p.swap(pos, pos - 1),
        Right => p.swap(pos, pos + 1),
    }
    for &v in p.iter() {
        if mobile < v {
            dirs[v] = dirs[v].swap_direction();
        }
    }

}

fn print_all_permutations(n: usize) {
    let mut p = (0..n).collect::<Vec<_>>();
    let mut dirs = vec![Direction::Left; n];
    println!("{:?}", p);
    for _ in 1..factorial(n) {
        find_next_permutation(&mut p, &mut dirs);
        println!("{:?}", p);
    }
}

static USAGE_TEXT: &'static str = "Prints all permutations of the sequence of numbers from 0 to n - 1.\n Usage: permutations n";

fn main() {
    use std::str::FromStr;
     let n = match std::env::args().skip(1).next() {
         None => {
             eprintln!("{}", USAGE_TEXT);
             std::process::exit(2);
         },
         Some(arg) => match usize::from_str(&arg) {
             Ok(n) => n,
             Err(_) => {
                 eprintln!("Invalid argument {}", arg);
                 eprintln!("{}", USAGE_TEXT);
                 std::process::exit(2);
             },
         }
     };
    print_all_permutations(n);

}

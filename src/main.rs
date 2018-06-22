#[derive(PartialEq, Copy, Clone, Debug)]
enum Facing {
    LeftToRight,
    RightToLeft,
}

impl Facing {
    fn swap_direction(self) -> Self {
        use Facing;
        match self {
            Facing::LeftToRight => Facing::RightToLeft,
            Facing::RightToLeft => Facing::LeftToRight
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
            return i + 1;
        }
    }
    0
}

fn get_mobile(p: &[usize], dirs: &[Facing]) -> usize {
    use Facing::*;
    let mut mobile = 0;
    let mut previous_mobile = 0;
    for i in 0..p.len() {
        if dirs[p[i] - 1] == RightToLeft && i != 0             
            && p[i - 1] < p[i] && previous_mobile < p[i] {
            mobile = p[i];
            previous_mobile = mobile;
        }

        if dirs[p[i] - 1] == LeftToRight && i != p.len() - 1             
            && p[i + 1] < p[i] && previous_mobile < p[i] {
            mobile = p[i];
            previous_mobile = mobile;
        }
    }
    if mobile == 0 && previous_mobile == 0 { 0 } else { mobile }
}

fn print_permutation(p: &mut [usize], dirs: &mut [Facing]) {
    use Facing::*;
    let mobile = get_mobile(p, dirs);
    let pos = find_largest_mobile(p, mobile);
    match dirs[p[pos - 1] - 1] {
        RightToLeft => p.swap(pos - 1, pos - 2),
        LeftToRight => p.swap(pos - 1, pos),
    }
    for &v in p.iter() {
        if mobile < v {
            dirs[v - 1] = dirs[v - 1].swap_direction();
        }
    }
    println!("{:?}", p);
}

fn print_all_permutations(n: usize) {
    let mut p = (0..n).collect::<Vec<_>>();
    let mut dirs = vec![Facing::RightToLeft; n];
    println!("{:?}", p);
    for _ in 1..factorial(n) {
        print_permutation(&mut p, &mut dirs);
    }
}

static USAGE_TEXT: &'static str = "Prints all permutations of the numbers 1 to n.\n Usage: permutations n";

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

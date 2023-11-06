use super::*;

#[test]
fn main() {
    #[derive(PartialEq, Clone, Copy, Debug)]
    enum Type {
        Rock,
        Paper,
        Scissor,
    }
    use Type::*;

    fn get_outcome(you: &Type, them: &Type) -> bool {
        match (you, them) {
            (Paper, Rock) => true,
            (Rock, Scissor) => true,
            (Scissor, Paper) => true,
            (_, _) => you == them,
        }
    }

    fn get_score(you: Type, them: Type) -> i32 {
        let you_win = get_outcome(&you, &them);
        let they_win = get_outcome(&them, &you);
        if you_win && they_win {
            return 3;
        }
        if you_win {
            return 6;
        }
        return 0;
    }

    fn your_move(expected: &Type, them: &Type) -> Type {
        match (them, expected) {
            (_, Paper) => *them,
            (Rock, Rock) => Scissor,
            (Rock, Scissor) => Paper,
            (Paper, Rock) => Rock,
            (Paper, Scissor) => Scissor,
            (Scissor, Rock) => Paper,
            (Scissor, Scissor) => Rock,
        }
    }

    fn uno_reverse(expected: Type, them: Type) -> i32 {
        let you = your_move(&expected, &them);
        println!("{:?}", you);
        get_bonus(&you) + get_score(you, them)
    }

    fn get_type(slice: char) -> Type {
        match slice {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissor,
            _ => panic!("{}", slice),
        }
    }

    fn get_bonus(t: &Type) -> i32 {
        match t {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
    }

    let contents = read_input("d2.txt");

    let mut total = 0;

    for line in contents.lines() {
        let them = get_type(line.chars().nth(0).unwrap());
        let you = get_type(line.chars().nth(2).unwrap());
        total += uno_reverse(you, them);
    }

    println!("Total {}", total);
}

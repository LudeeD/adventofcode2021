
pub fn run() {
    println!("From day one file");

    let input = include_str!("../inputs/1.txt");
    println!("{input}");

    parse(input);
    println!("{}", parse1(input));
}

fn parse1(input: &str) -> usize {
    input
        .lines()
        .map(|n| n.parse().unwrap())
            .collect::<Vec<u16>>()
            .array_windows()
            .filter(|[a, b]| a < b)
            .count()
}


fn parse(input: &str) -> u32 {
    let lines = input.lines().map(|line| line.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    println!("{:?}", lines.len());

    let mut count : u32 = 0;
    let mut previous = lines[0];

    let mut first = true;

    for value in lines {
        if first {
            first = false;
            continue;
        }

        if value > previous {
            count += 1;
            //println!("Current {} Previous {} (increased)", value, previous);
        }else {
            //println!("Current {} Previous {} (decreased)", value, previous)
        }

        previous = value;
    }

    println!("Increased {} times", count);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "199
200
208
210
200
207
240
269
260
263";
    #[test]
    fn mini() {
        assert_eq!(parse(INPUTS), 7);
    }
}
fn main() {
    const MAX_IT: u32 = 14;
    const MAX_IT_J: u32 = 11;
    let (mut a1, mut a2, mut d1) = (1.0, 0.0, 3.2);

    println!(" i    d");
    for i in 2..MAX_IT {
        let mut a = a1 + (a1 - a2) / d1;
        for _ in 1..MAX_IT_J {
            let (mut x, mut y) = (0.0, 0.0);
            for _ in 1..(1 << i) + 1 {
                y = 1.0 - 2.0 * y * x;
                x = a - x * x;
            }
            a -= x / y;
        }
        let d = (a1 - a2) / (a - a1);
        println!("{i:>2}    {d}");
        d1 = d;
        a2 = a1;
        a1 = a;
    }
}

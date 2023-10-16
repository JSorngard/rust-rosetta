fn feigenbaum() -> f64 {
    let (mut a1, mut a2, mut d1) = (1.0, 0.0, 3.2);
    println!(" i    d");

    for i in 2..14 {
        let mut a = a1 + (a1 - a2) / d1;
        for _ in 1..11 {
            let (mut x, mut y) = (0.0, 0.0);
            for _ in 1..(1 << i) + 1 {
                y = 1.0 - 2.0 * y * x;
                x = a - x * x;
            }
            a -= x / y;
        }
        d1 = (a1 - a2) / (a - a1);
        a2 = a1;
        a1 = a;

        println!("{i:>2}    {d1}");
    }

    d1
}

fn main() {
    feigenbaum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_feigenbaum_value() {
        let d1 = feigenbaum();
        assert!(d1 > 4.669_2 && d1 < 4.669_206);
    }
}

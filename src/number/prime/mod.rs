pub fn prime_table(n: usize) -> Vec<bool> {
    let mut tab = vec![true; n + 1];
    tab[0] = false;
    if n >= 1 {
        tab[1] = false;
    }
    for x in 2..=n {
        if tab[x] {
            for y in (2 * x..=n).step_by(x) {
                tab[y] = false;
            }
        }
    }
    tab
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_table_test() {
        let tab = prime_table(100);
        assert!(tab[2]);
        assert!(tab[13]);
        assert!(!tab[6]);
        assert!(!tab[9]);
        assert!(!tab[0]);
        assert!(!tab[1]);
    }
}

use std::mem::swap;

pub fn suffix_array<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return Vec::new()
    }

    let mut sa: Vec<_> = (0..n).collect();
    sa.sort_by(|i, j| s[*i].cmp(&s[*j]).then(j.cmp(i)));

    let mut ord = vec![0; n];
    for (i, j) in sa.windows(2).map(|w| (w[0], w[1])) {
        ord[j] = ord[i] + (s[i] != s[j]) as usize;
    }

    let mut backet = vec![0; n];
    let mut tmp = vec![0; n];

    for k in (0..).map(|i| 1 << i).take_while(|k| *k << 1 < n) {
        let backet_last = ord[*sa.last().unwrap()];
        backet[..=backet_last].iter_mut().for_each(|x| *x = 0);
        for &o in &ord {
            backet[o] += 1;
        }
        for i in 1..=backet_last {
            backet[i] += backet[i - 1];
        }
        tmp.copy_from_slice(&sa);
        for &i in tmp.iter().rev() {
            let (i, of) = i.overflowing_sub(k);
            if !of {
                backet[ord[i]] -= 1;
                sa[backet[ord[i]]] = i;
            } 
        }

        tmp[sa[0]] = 0;
        for (i, j) in sa.windows(2).map(|w| (w[0], w[1])) {
            tmp[j] = tmp[i];
            if ord[i] != ord[j] || i + k >= n || j + k >= n || ord[i + k] != ord[j + k] {
                tmp[j] += 1;
            }
        }
        swap(&mut ord, &mut tmp);
    }

    sa
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suffix_array_test() {
        assert_eq!(suffix_array(b"mississippi"), &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
        assert_eq!(suffix_array(b"axxxxx"), &[0, 5, 4, 3, 2, 1]);
    }
}
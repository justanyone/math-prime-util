//! Vector and integer set operations from MPU 0.75.
use std::collections::{HashMap, HashSet};

/// vecsum for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecsum`.
/// Results use `u128`, including sums that exceed `u64`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecsum(&[u64::MAX,1]), 1_u128<<64);
/// ```
pub fn vecsum(a: &[u64]) -> u128 {
    a.iter().map(|&x| u128::from(x)).sum()
}
/// vecprefixsum for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecprefixsum`.
/// Results use `u128`, including sums that exceed `u64`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecprefixsum(&[1,2]), [1,3]);
/// ```
pub fn vecprefixsum(a: &[u64]) -> Vec<u128> {
    let mut s = 0;
    a.iter()
        .map(|&x| {
            s += u128::from(x);
            s
        })
        .collect()
}
/// vecprod for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecprod`.
///
/// # Panics
/// Panics on numeric overflow.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecprod(&[2,3]), 6);
/// ```
pub fn vecprod(a: &[u64]) -> u128 {
    if a.contains(&0) {
        return 0;
    }
    a.iter().fold(1_u128, |p, &x| {
        p.checked_mul(u128::from(x)).expect("product exceeds u128")
    })
}
/// vecmin for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecmin`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecmin(&[1,2]), Some(1));
/// ```
pub fn vecmin<T: Ord + Copy>(a: &[T]) -> Option<T> {
    a.iter().copied().min()
}
/// vecmax for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecmax`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecmax(&[1,2]), Some(2));
/// ```
pub fn vecmax<T: Ord + Copy>(a: &[T]) -> Option<T> {
    a.iter().copied().max()
}
/// vecreduce for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecreduce`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecreduce(&[1,2], |a,b| a+b), Some(3));
/// ```
pub fn vecreduce<T: Clone>(a: &[T], f: impl FnMut(T, T) -> T) -> Option<T> {
    a.iter().cloned().reduce(f)
}
/// vecany for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecany`.
///
/// ```
/// use math_prime_util::*;
/// assert!(vecany(&[1,2], |&x| x==1));
/// ```
pub fn vecany<T>(a: &[T], f: impl FnMut(&T) -> bool) -> bool {
    a.iter().any(f)
}
/// vecall for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecall`.
///
/// ```
/// use math_prime_util::*;
/// assert!(vecall(&[1,2], |&x| x>0));
/// ```
pub fn vecall<T>(a: &[T], f: impl FnMut(&T) -> bool) -> bool {
    a.iter().all(f)
}
/// vecnone for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecnone`.
///
/// ```
/// use math_prime_util::*;
/// assert!(vecnone(&[1,2], |&x| x==0));
/// ```
pub fn vecnone<T>(a: &[T], f: impl FnMut(&T) -> bool) -> bool {
    !a.iter().any(f)
}
/// vecnotall for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecnotall`.
///
/// ```
/// use math_prime_util::*;
/// assert!(vecnotall(&[1,2], |&x| x==1));
/// ```
pub fn vecnotall<T>(a: &[T], f: impl FnMut(&T) -> bool) -> bool {
    !a.iter().all(f)
}
/// vecfirst for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecfirst`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecfirst(&[1,2], |&x| x>1), Some(&2));
/// ```
pub fn vecfirst<T>(a: &[T], mut f: impl FnMut(&T) -> bool) -> Option<&T> {
    a.iter().find(|x| f(x))
}
/// vecfirstidx for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecfirstidx`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecfirstidx(&[1,2], |&x| x>1), Some(1));
/// ```
pub fn vecfirstidx<T>(a: &[T], f: impl FnMut(&T) -> bool) -> Option<usize> {
    a.iter().position(f)
}
/// A bit mask or an index list for extraction.
pub enum VecSelection<'a> {
    /// Select set bit positions.
    Mask(u64),
    /// Select indices. Negative indices count from the end.
    Indices(&'a [isize]),
}
/// vecextract for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecextract`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecextract(&[10,20], VecSelection::Mask(2)), [Some(20)]);
/// ```
pub fn vecextract<T: Clone>(a: &[T], selection: VecSelection<'_>) -> Vec<Option<T>> {
    match selection {
        VecSelection::Mask(m) => (0..64)
            .filter(|i| m & (1_u64 << i) != 0)
            .map(|i| a.get(i).cloned())
            .collect(),
        VecSelection::Indices(ix) => ix
            .iter()
            .map(|&i| {
                let index = if i < 0 {
                    a.len().checked_sub(i.unsigned_abs())
                } else {
                    usize::try_from(i).ok()
                };
                index.and_then(|i| a.get(i)).cloned()
            })
            .collect(),
    }
}
/// vecuniq for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecuniq`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecuniq(&[3,1,3]), [3,1]);
/// ```
pub fn vecuniq(a: &[i64]) -> Vec<i64> {
    let mut seen = HashSet::new();
    a.iter().copied().filter(|x| seen.insert(*x)).collect()
}
/// vecfreq for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecfreq`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecfreq(&[3,1,3]), [(1,1),(3,2)]);
/// ```
pub fn vecfreq(a: &[i64]) -> Vec<(i64, usize)> {
    let mut out: Vec<(i64, usize)> = Vec::new();
    for x in vecsort(a) {
        if let Some(last) = out.last_mut() {
            if last.0 == x {
                last.1 += 1;
                continue;
            }
        }
        out.push((x, 1));
    }
    out
}
/// vecsingleton for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecsingleton`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecsingleton(&[3,1,3]), [1]);
/// ```
pub fn vecsingleton(a: &[i64]) -> Vec<i64> {
    let mut count = HashMap::new();
    for &x in a {
        *count.entry(x).or_insert(0_usize) += 1;
    }
    a.iter().copied().filter(|x| count[x] == 1).collect()
}
/// vecsort for integer vectors or sets.
///
/// Port of MPU 0.75 `sort.c: sort_iv_array, _qs_iv, _radixsort`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecsort(&[2,1]), [1,2]);
/// ```
pub fn vecsort(a: &[i64]) -> Vec<i64> {
    let mut r = a.to_vec();
    vecsorti(&mut r);
    r
}
/// vecrsort for integer vectors or sets.
///
/// Port of MPU 0.75 `sort.c: sort_iv_array, _qs_iv, _radixsort`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecrsort(&[1,2]), [2,1]);
/// ```
pub fn vecrsort(a: &[i64]) -> Vec<i64> {
    let mut r = vecsort(a);
    r.reverse();
    r
}
/// vecsorti for integer vectors or sets.
///
/// Port of MPU 0.75 `sort.c: sort_iv_array, _qs_iv, _radixsort`.
///
/// ```
/// use math_prime_util::*;
/// let mut a=[2,1]; vecsorti(&mut a); assert_eq!(a,[1,2]);
/// ```
pub fn vecsorti(a: &mut [i64]) {
    sort_signed(a);
}
/// vecrsorti for integer vectors or sets.
///
/// Port of MPU 0.75 `sort.c: sort_iv_array, _qs_iv, _radixsort`.
///
/// ```
/// use math_prime_util::*;
/// let mut a=[1,2]; vecrsorti(&mut a); assert_eq!(a,[2,1]);
/// ```
pub fn vecrsorti(a: &mut [i64]) {
    sort_signed(a);
    a.reverse();
}
/// vecequal for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecequal`.
///
/// ```
/// use math_prime_util::*;
/// assert!(vecequal(&[1,2], &[1,2]));
/// ```
pub fn vecequal<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a == b
}
/// vecmex for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecmex`.
///
/// # Panics
/// Panics if an input is negative.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecmex(&[0,1,3]),2);
/// ```
pub fn vecmex(a: &[i64]) -> usize {
    assert!(a.iter().all(|&x| x >= 0), "values must be nonnegative");
    mex(a, 0)
}
/// vecpmex for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecpmex`.
///
/// # Panics
/// Panics if an input is zero or negative.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecpmex(&[1,3]),2);
/// ```
pub fn vecpmex(a: &[i64]) -> usize {
    assert!(a.iter().all(|&x| x > 0), "values must be positive");
    mex(a, 1)
}
/// vecslide for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecslide`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecslide(&[1,2,4], |a,b| b-a), [1,2]);
/// ```
pub fn vecslide<T, R>(a: &[T], mut f: impl FnMut(&T, &T) -> R) -> Vec<R> {
    a.windows(2).map(|w| f(&w[0], &w[1])).collect()
}
/// vecpairwise for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecpairwise`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecpairwise(&[1,2], &[3], |a,b| [a+b]), [4]);
/// ```
pub fn vecpairwise<T, U, R, I: IntoIterator<Item = R>>(
    a: &[T],
    b: &[U],
    mut f: impl FnMut(&T, &U) -> I,
) -> Vec<R> {
    a.iter().zip(b).flat_map(|(a, b)| f(a, b)).collect()
}
/// vecwindow for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: vecwindow`.
///
/// # Panics
/// Panics if step or size is zero.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(vecwindow(&[1,2,3],1,2, |w| [w[0]+w[1]]), [3,5]);
/// ```
pub fn vecwindow<T, R, I: IntoIterator<Item = R>>(
    a: &[T],
    step: usize,
    size: usize,
    f: impl FnMut(&[T]) -> I,
) -> Vec<R> {
    assert!(step > 0 && size > 0, "step and size must be positive");
    a.windows(size).step_by(step).flat_map(f).collect()
}
/// toset for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_op`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(toset(&[3,1,3]),[1,3]);
/// ```
pub fn toset(a: &[i64]) -> Vec<i64> {
    let mut r = vecsort(a);
    r.dedup();
    r
}
/// setunion for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_op`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(setunion(&[1,2], &[2,3]), [1,2,3]);
/// ```
pub fn setunion(a: &[i64], b: &[i64]) -> Vec<i64> {
    merge(&toset(a), &toset(b), 0)
}
/// setintersect for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_op`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(setintersect(&[1,2], &[2,3]), [2]);
/// ```
pub fn setintersect(a: &[i64], b: &[i64]) -> Vec<i64> {
    merge(&toset(a), &toset(b), 1)
}
/// setminus for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_op`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(setminus(&[1,2], &[2,3]), [1]);
/// ```
pub fn setminus(a: &[i64], b: &[i64]) -> Vec<i64> {
    merge(&toset(a), &toset(b), 2)
}
/// setdelta for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_op`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(setdelta(&[1,2], &[2,3]), [1,3]);
/// ```
pub fn setdelta(a: &[i64], b: &[i64]) -> Vec<i64> {
    merge(&toset(a), &toset(b), 3)
}
/// setinsert for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: setinsert`.
/// Return the count added, count removed, or signed size change, respectively.
///
/// ```
/// use math_prime_util::*;
/// let mut a=vec![1,2]; assert_eq!(setinsert(&mut a,&[2,3]),1);
/// ```
pub fn setinsert(a: &mut Vec<i64>, b: &[i64]) -> i128 {
    let old = a.len() as i128;
    *a = setunion(a, b);
    a.len() as i128 - old
}
/// setremove for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: setremove`.
/// Return the count added, count removed, or signed size change, respectively.
///
/// ```
/// use math_prime_util::*;
/// let mut a=vec![1,2]; assert_eq!(setremove(&mut a,&[2,3]),1);
/// ```
pub fn setremove(a: &mut Vec<i64>, b: &[i64]) -> i128 {
    let old = a.len() as i128;
    *a = setminus(a, b);
    old - a.len() as i128
}
/// setinvert for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: setinvert`.
/// Return the count added, count removed, or signed size change, respectively.
///
/// ```
/// use math_prime_util::*;
/// let mut a=vec![1,2]; assert_eq!(setinvert(&mut a,&[2,3]),0);
/// ```
pub fn setinvert(a: &mut Vec<i64>, b: &[i64]) -> i128 {
    let old = a.len() as i128;
    *a = setdelta(a, b);
    a.len() as i128 - old
}
/// setcontains for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: setcontains`.
///
/// ```
/// use math_prime_util::*;
/// assert!(setcontains(&[1,2], &[2]));
/// ```
pub fn setcontains(a: &[i64], b: &[i64]) -> bool {
    let a = toset(a);
    b.iter().all(|x| a.binary_search(x).is_ok())
}
/// setcontainsany for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: setcontainsany`.
///
/// ```
/// use math_prime_util::*;
/// assert!(setcontainsany(&[1,2], &[2,3]));
/// ```
pub fn setcontainsany(a: &[i64], b: &[i64]) -> bool {
    let a = toset(a);
    b.iter().any(|x| a.binary_search(x).is_ok())
}
/// setbinop for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: setbinop`.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(setbinop(&[1,2], &[3], |a,b| a+b), [4,5]);
/// ```
pub fn setbinop(a: &[i64], b: &[i64], mut f: impl FnMut(i64, i64) -> i64) -> Vec<i64> {
    let mut seen = HashSet::new();
    for &x in a {
        for &y in b {
            seen.insert(f(x, y));
        }
    }
    toset(&seen.into_iter().collect::<Vec<_>>())
}
/// sumset for integer vectors or sets.
///
/// Port of MPU 0.75 `XS.xs: sumset`.
///
/// # Panics
/// Panics on numeric overflow.
///
/// ```
/// use math_prime_util::*;
/// assert_eq!(sumset(&[1,2], &[1,2]), [2,3,4]);
/// ```
pub fn sumset(a: &[i64], b: &[i64]) -> Vec<i64> {
    setbinop(a, b, |x, y| x.checked_add(y).expect("sum exceeds i64"))
}
/// is sidon set for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_is_sidon_set`.
///
/// ```
/// use math_prime_util::*;
/// assert!(is_sidon_set(&[1,2,4]));
/// ```
pub fn is_sidon_set(a: &[i64]) -> bool {
    let a = toset(a);
    if a.first().is_some_and(|&x| x < 0) {
        return false;
    }
    let mut seen = HashSet::new();
    for (i, &x) in a.iter().enumerate() {
        for &y in &a[i..] {
            if !seen.insert(i128::from(x) + i128::from(y)) {
                return false;
            }
        }
    }
    true
}
/// is sumfree set for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_is_sumfree_set`.
///
/// ```
/// use math_prime_util::*;
/// assert!(is_sumfree_set(&[1,3,5]));
/// ```
pub fn is_sumfree_set(a: &[i64]) -> bool {
    let a = toset(a);
    for (i, &x) in a.iter().enumerate() {
        for &y in &a[i..] {
            if let Some(sum) = x.checked_add(y) {
                if a.binary_search(&sum).is_ok() {
                    return false;
                }
            }
        }
    }
    true
}
/// set is disjoint for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_relation`.
///
/// ```
/// use math_prime_util::*;
/// assert!(set_is_disjoint(&[1], &[2]));
/// ```
pub fn set_is_disjoint(a: &[i64], b: &[i64]) -> bool {
    setintersect(a, b).is_empty()
}
/// set is equal for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_relation`.
///
/// ```
/// use math_prime_util::*;
/// assert!(set_is_equal(&[2,1], &[1,2]));
/// ```
pub fn set_is_equal(a: &[i64], b: &[i64]) -> bool {
    toset(a) == toset(b)
}
/// set is subset for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_relation`.
/// MPU tests whether the second set is a subset or superset of the first.
///
/// ```
/// use math_prime_util::*;
/// assert!(set_is_subset(&[1,2], &[1]));
/// ```
pub fn set_is_subset(a: &[i64], b: &[i64]) -> bool {
    setcontains(a, b)
}
/// set is proper subset for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_relation`.
/// MPU tests whether the second set is a subset or superset of the first.
///
/// ```
/// use math_prime_util::*;
/// assert!(set_is_proper_subset(&[1,2], &[1]));
/// ```
pub fn set_is_proper_subset(a: &[i64], b: &[i64]) -> bool {
    setcontains(a, b) && toset(a).len() > toset(b).len()
}
/// set is superset for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_relation`.
/// MPU tests whether the second set is a subset or superset of the first.
///
/// ```
/// use math_prime_util::*;
/// assert!(set_is_superset(&[1], &[1,2]));
/// ```
pub fn set_is_superset(a: &[i64], b: &[i64]) -> bool {
    setcontains(b, a)
}
/// set is proper superset for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_relation`.
/// MPU tests whether the second set is a subset or superset of the first.
///
/// ```
/// use math_prime_util::*;
/// assert!(set_is_proper_superset(&[1], &[1,2]));
/// ```
pub fn set_is_proper_superset(a: &[i64], b: &[i64]) -> bool {
    setcontains(b, a) && toset(b).len() > toset(a).len()
}
/// set is proper intersection for integer vectors or sets.
///
/// Port of MPU 0.75 `xs_set.c: xs_set_relation`.
///
/// ```
/// use math_prime_util::*;
/// assert!(set_is_proper_intersection(&[1,2], &[2,3]));
/// ```
pub fn set_is_proper_intersection(a: &[i64], b: &[i64]) -> bool {
    let r = setintersect(a, b).len();
    r > 0 && r < toset(a).len() && r < toset(b).len()
}
fn mex(a: &[i64], start: usize) -> usize {
    let mut seen = vec![false; a.len()];
    for &x in a {
        if let Ok(x) = usize::try_from(x) {
            if let Some(i) = x.checked_sub(start) {
                if let Some(v) = seen.get_mut(i) {
                    *v = true;
                }
            }
        }
    }
    start + seen.iter().position(|&x| !x).unwrap_or(seen.len())
}
fn merge(a: &[i64], b: &[i64], mode: u8) -> Vec<i64> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::new();
    while i < a.len() || j < b.len() {
        if j == b.len() || (i < a.len() && a[i] < b[j]) {
            if mode != 1 {
                out.push(a[i]);
            }
            i += 1;
        } else if i == a.len() || b[j] < a[i] {
            if mode == 0 || mode == 3 {
                out.push(b[j]);
            }
            j += 1;
        } else {
            if mode <= 1 {
                out.push(a[i]);
            }
            i += 1;
            j += 1;
        }
    }
    out
}
// sort.c: _mid3_iv_val, _mid2_of_4_iv, _partition_iv, _qs_iv.
fn median(a: i64, b: i64, c: i64) -> i64 {
    let s = [a, b, c];
    let x = a > b;
    let y = a > c;
    let z = b > c;
    s[usize::from(x == y) + usize::from(y ^ z)]
}
fn middle_four(a: &mut [i64]) {
    if a[0] > a[1] {
        a.swap(0, 1);
    }
    if a[2] > a[3] {
        a.swap(2, 3);
    }
    a[2] = a[0].max(a[2]);
    a[1] = a[1].min(a[3]);
}
fn quicksort(a: &mut [i64], mut bad: u32) {
    let n = a.len();
    if n <= 16 {
        for i in 1..n {
            let t = a[i];
            let mut j = i;
            while j > 0 && a[j - 1] > t {
                a[j] = a[j - 1];
                j -= 1;
            }
            a[j] = t;
        }
        return;
    }
    let pivot = if n <= 40 {
        median(a[0], a[(n - 1) / 2], a[n - 1])
    } else {
        let mut s = [0; 9];
        for i in 0..9 {
            s[i] = a[i * ((n - 1) / 8)];
        }
        middle_four(&mut s);
        middle_four(&mut s[4..]);
        s[0] = s[5];
        s[3] = s[8];
        middle_four(&mut s);
        median(s[6], s[1], s[2])
    };
    let (mut i, mut j) = (0, n - 1);
    loop {
        while a[i] < pivot {
            i += 1;
        }
        while a[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break;
        }
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
    let split = j + 1;
    if split < n / 8 || n - split < n / 8 {
        bad = bad.saturating_sub(1);
        if bad == 0 {
            heapsort(a);
            return;
        }
    }
    let (left, right) = a.split_at_mut(split);
    quicksort(left, bad);
    quicksort(right, bad);
}
// sort.c: _heapsort.
fn heapsort(a: &mut [i64]) {
    let mut start = a.len() / 2;
    if start == 0 {
        return;
    }
    let mut end = a.len() - 1;
    loop {
        let r = if start > 0 {
            start -= 1;
            a[start]
        } else if end > 0 {
            let r = a[end];
            a[end] = a[0];
            end -= 1;
            r
        } else {
            return;
        };
        let mut b = start;
        let mut c = 2 * b + 1;
        while c < end {
            if a[c + 1] >= a[c] {
                c += 1;
            }
            if r >= a[c] {
                break;
            }
            a[b] = a[c];
            b = c;
            c = 2 * b + 1;
        }
        if c == end && r < a[c] {
            a[b] = a[c];
            b = c;
        }
        a[b] = r;
    }
}
// sort.c: sort_iv_array and _radixsort.
fn sort_signed(a: &mut [i64]) {
    if a.len() < 400 {
        if a.len() > 1 {
            quicksort(a, a.len().ilog2());
        }
        return;
    }
    let first = a[0];
    let mut vary = 0_u64;
    let mut width = 0_u64;
    let mut asc = true;
    let mut desc = true;
    let mut prev = first;
    for &x in a.iter() {
        let bits = u64::from_ne_bytes(x.to_ne_bytes());
        vary |= bits ^ u64::from_ne_bytes(first.to_ne_bytes());
        width |= if x < 0 { !bits } else { bits };
        asc &= prev <= x;
        desc &= prev >= x;
        prev = x;
    }
    if asc {
        return;
    }
    if desc {
        a.reverse();
        return;
    }
    let mut signshift = 0;
    while signshift < 56 && width >> (signshift + 7) != 0 {
        signshift += 8;
    }
    let mut buffer = Vec::new();
    if buffer.try_reserve_exact(a.len()).is_err() {
        quicksort(a, a.len().ilog2());
        return;
    }
    buffer.resize(a.len(), 0);
    for shift in (0..=signshift).step_by(8) {
        if vary >> shift & 255 == 0 {
            continue;
        }
        let sign = if shift == signshift { 128 } else { 0 };
        let key = |x: i64| -> usize { usize::from(x.to_le_bytes()[shift / 8]) ^ sign };
        let mut counts = [0_usize; 256];
        for &x in a.iter() {
            counts[key(x)] += 1;
        }
        let mut total = 0;
        for c in &mut counts {
            let n = *c;
            *c = total;
            total += n;
        }
        for &x in a.iter() {
            let k = key(x);
            buffer[counts[k]] = x;
            counts[k] += 1;
        }
        a.copy_from_slice(&buffer);
    }
}

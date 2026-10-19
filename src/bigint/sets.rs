//! Arbitrary precision set and vector operations.
//!
//! Inputs may be unsorted. Set results are sorted and unique. Generic
//! element operations accept both `BigUint` and `BigInt`. State is local.
use num_bigint::BigUint;
use std::collections::BTreeSet;
use std::ops::Add;

/// Return an extreme value, or `None` for empty input.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::vecmin`.

///
/// ```
/// use math_prime_util::bigint::sets::vecmin;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert!(vecmin(&a).is_some());
/// ```
pub fn vecmin<T: Ord + Clone>(a: &[T]) -> Option<T> {
    a.iter().min().cloned()
}

/// Return an extreme value, or `None` for empty input.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::vecmax`.

///
/// ```
/// use math_prime_util::bigint::sets::vecmax;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert!(vecmax(&a).is_some());
/// ```
pub fn vecmax<T: Ord + Clone>(a: &[T]) -> Option<T> {
    a.iter().max().cloned()
}

/// Sort values in place from low to high.
///
/// Port of MPU 0.75 `reference/mpu/sort.c::sort_iv_array`.

///
/// ```
/// use math_prime_util::bigint::sets::vecsorti;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// let mut b = a.clone(); vecsorti(&mut b); assert_eq!(b.len(), 2);
/// ```
pub fn vecsorti<T: Ord>(a: &mut [T]) {
    a.sort_unstable();
}

/// Sort values in place from high to low.
///
/// Port of MPU 0.75 `reference/mpu/sort.c::sort_iv_array`.

///
/// ```
/// use math_prime_util::bigint::sets::vecrsorti;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// let mut b = a.clone(); vecrsorti(&mut b); assert_eq!(b.len(), 2);
/// ```
pub fn vecrsorti<T: Ord>(a: &mut [T]) {
    a.sort_unstable_by(|x, y| y.cmp(x));
}

/// Return a sorted copy from low to high.
///
/// Port of MPU 0.75 `reference/mpu/sort.c::sort_iv_array`.

///
/// ```
/// use math_prime_util::bigint::sets::vecsort;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(vecsort(&a).len(), 2);
/// ```
pub fn vecsort<T: Ord + Clone>(a: &[T]) -> Vec<T> {
    let mut r = a.to_vec();
    vecsorti(&mut r);
    r
}

/// Return a sorted copy from high to low.
///
/// Port of MPU 0.75 `reference/mpu/sort.c::sort_iv_array`.

///
/// ```
/// use math_prime_util::bigint::sets::vecrsort;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(vecrsort(&a).len(), 2);
/// ```
pub fn vecrsort<T: Ord + Clone>(a: &[T]) -> Vec<T> {
    let mut r = a.to_vec();
    vecrsorti(&mut r);
    r
}

/// Return sorted unique values.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_op`.

///
/// ```
/// use math_prime_util::bigint::sets::toset;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(toset(&a), a);
/// ```
pub fn toset<T: Ord + Clone>(a: &[T]) -> Vec<T> {
    let mut r = vecsort(a);
    r.dedup();
    r
}

/// Return unique values in first occurrence order.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::vecuniq`.

///
/// ```
/// use math_prime_util::bigint::sets::vecuniq;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(vecuniq(&a), a);
/// ```
pub fn vecuniq<T: Ord + Clone>(a: &[T]) -> Vec<T> {
    let mut seen = BTreeSet::new();
    a.iter().filter(|x| seen.insert(*x)).cloned().collect()
}

/// Return sorted value and count pairs.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::vecfreq`.

///
/// ```
/// use math_prime_util::bigint::sets::vecfreq;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(vecfreq(&a).len(), 2);
/// ```
pub fn vecfreq<T: Ord + Clone>(a: &[T]) -> Vec<(T, usize)> {
    let mut out: Vec<(T, usize)> = Vec::new();
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

/// Return values that occur once in input order.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::vecsingleton`.

///
/// ```
/// use math_prime_util::bigint::sets::vecsingleton;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(vecsingleton(&a), a);
/// ```
pub fn vecsingleton<T: Ord + Clone>(a: &[T]) -> Vec<T> {
    let single: BTreeSet<T> = vecfreq(a)
        .into_iter()
        .filter(|x| x.1 == 1)
        .map(|x| x.0)
        .collect();
    a.iter().filter(|x| single.contains(*x)).cloned().collect()
}

/// Return the first missing nonnegative integer.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::vecmex`.

///
/// ```
/// use math_prime_util::bigint::sets::vecmex;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(vecmex(&a), BigUint::from(0_u8));
/// ```
pub fn vecmex(a: &[BigUint]) -> BigUint {
    let seen: BTreeSet<&BigUint> = a.iter().collect();
    let mut value = BigUint::from(0_u8);
    while seen.contains(&value) {
        value += 1_u8;
    }
    value
}

/// Return the first missing positive integer.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::vecpmex`.

///
/// # Panics
/// Panics if an input is zero.
///
/// ```
/// use math_prime_util::bigint::sets::vecpmex;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(vecpmex(&a), BigUint::from(3_u8));
/// ```
pub fn vecpmex(a: &[BigUint]) -> BigUint {
    assert!(
        a.iter().all(|x| *x != BigUint::from(0_u8)),
        "values must be positive"
    );
    let seen: BTreeSet<&BigUint> = a.iter().collect();
    let mut value = BigUint::from(1_u8);
    while seen.contains(&value) {
        value += 1_u8;
    }
    value
}

fn merge<T: Ord + Clone>(a: &[T], b: &[T], mode: u8) -> Vec<T> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::new();
    while i < a.len() || j < b.len() {
        if j == b.len() || (i < a.len() && a[i] < b[j]) {
            if mode != 1 {
                out.push(a[i].clone());
            }
            i += 1;
        } else if i == a.len() || b[j] < a[i] {
            if mode == 0 || mode == 3 {
                out.push(b[j].clone());
            }
            j += 1;
        } else {
            if mode <= 1 {
                out.push(a[i].clone());
            }
            i += 1;
            j += 1;
        }
    }
    out
}

/// Return the union of two sets.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_op`.

///
/// ```
/// use math_prime_util::bigint::sets::setunion;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(setunion(&a, &a).len(), 2);
/// ```
pub fn setunion<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    merge(&toset(a), &toset(b), 0)
}

/// Return the intersection of two sets.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_op`.

///
/// ```
/// use math_prime_util::bigint::sets::setintersect;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(setintersect(&a, &a).len(), 2);
/// ```
pub fn setintersect<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    merge(&toset(a), &toset(b), 1)
}

/// Return the difference of two sets.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_op`.

///
/// ```
/// use math_prime_util::bigint::sets::setminus;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(setminus(&a, &a).len(), 0);
/// ```
pub fn setminus<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    merge(&toset(a), &toset(b), 2)
}

/// Return the symmetric difference of two sets.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_op`.

///
/// ```
/// use math_prime_util::bigint::sets::setdelta;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(setdelta(&a, &a).len(), 0);
/// ```
pub fn setdelta<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    merge(&toset(a), &toset(b), 3)
}

/// Return the number of inserted values.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::setinsert`.

///
/// ```
/// use math_prime_util::bigint::sets::setinsert;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// let mut b = a.clone(); setinsert(&mut b, &a);
/// ```
pub fn setinsert<T: Ord + Clone>(a: &mut Vec<T>, b: &[T]) -> i128 {
    let old = a.len() as i128;
    *a = setunion(a, b);
    let new = a.len() as i128;
    new - old
}

/// Return the number of removed values.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::setremove`.

///
/// ```
/// use math_prime_util::bigint::sets::setremove;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// let mut b = a.clone(); setremove(&mut b, &a);
/// ```
pub fn setremove<T: Ord + Clone>(a: &mut Vec<T>, b: &[T]) -> i128 {
    let old = a.len() as i128;
    *a = setminus(a, b);
    let new = a.len() as i128;
    old - new
}

/// Toggle values and return the signed length change.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::setinvert`.

///
/// ```
/// use math_prime_util::bigint::sets::setinvert;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// let mut b = a.clone(); setinvert(&mut b, &a);
/// ```
pub fn setinvert<T: Ord + Clone>(a: &mut Vec<T>, b: &[T]) -> i128 {
    let old = a.len() as i128;
    *a = setdelta(a, b);
    let new = a.len() as i128;
    new - old
}

/// Test whether all query values occur in the set.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::setcontains`.

///
/// ```
/// use math_prime_util::bigint::sets::setcontains;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert!(setcontains(&a, &a));
/// ```
pub fn setcontains<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    let a = toset(a);
    b.iter().all(|x| a.binary_search(x).is_ok())
}

/// Test whether any query values occur in the set.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::setcontainsany`.

///
/// ```
/// use math_prime_util::bigint::sets::setcontainsany;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert!(setcontainsany(&a, &a));
/// ```
pub fn setcontainsany<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    let a = toset(a);
    b.iter().any(|x| a.binary_search(x).is_ok())
}

/// Test for no common values.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_relation`.

///
/// ```
/// use math_prime_util::bigint::sets::set_is_disjoint;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(set_is_disjoint(&a, &a), false);
/// ```
pub fn set_is_disjoint<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    setintersect(a, b).is_empty()
}

/// Test for equal sets.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_relation`.

///
/// ```
/// use math_prime_util::bigint::sets::set_is_equal;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(set_is_equal(&a, &a), true);
/// ```
pub fn set_is_equal<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    toset(a) == toset(b)
}

/// Test if the second set is contained in the first.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_relation`.

///
/// ```
/// use math_prime_util::bigint::sets::set_is_subset;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(set_is_subset(&a, &a), true);
/// ```
pub fn set_is_subset<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    setcontains(a, b)
}

/// Test if the second set contains the first.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_relation`.

///
/// ```
/// use math_prime_util::bigint::sets::set_is_superset;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(set_is_superset(&a, &a), true);
/// ```
pub fn set_is_superset<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    setcontains(b, a)
}

/// Test if the second set is a strict subset of the first.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_relation`.

///
/// ```
/// use math_prime_util::bigint::sets::set_is_proper_subset;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(set_is_proper_subset(&a, &a), false);
/// ```
pub fn set_is_proper_subset<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    setcontains(a, b) && toset(a).len() > toset(b).len()
}

/// Test if the second set is a strict superset of the first.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_relation`.

///
/// ```
/// use math_prime_util::bigint::sets::set_is_proper_superset;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(set_is_proper_superset(&a, &a), false);
/// ```
pub fn set_is_proper_superset<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    setcontains(b, a) && toset(b).len() > toset(a).len()
}

/// Test for an intersection smaller than both sets and not empty.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_set_relation`.

///
/// ```
/// use math_prime_util::bigint::sets::set_is_proper_intersection;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(set_is_proper_intersection(&a, &a), false);
/// ```
pub fn set_is_proper_intersection<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    {
        let n = setintersect(a, b).len();
        n > 0 && n < toset(a).len() && n < toset(b).len()
    }
}

/// Apply a closure to each pair and return sorted unique results.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::setbinop`.

///
/// ```
/// use math_prime_util::bigint::sets::setbinop;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(setbinop(&a, &a, |x, y| x + y).len(), 3);
/// ```
pub fn setbinop<T: Ord + Clone>(a: &[T], b: &[T], mut f: impl FnMut(&T, &T) -> T) -> Vec<T> {
    let mut out = BTreeSet::new();
    for x in a {
        for y in b {
            out.insert(f(x, y));
        }
    }
    out.into_iter().collect()
}

/// Return sorted unique pair sums.
///
/// Port of MPU 0.75 `reference/mpu/XS.xs::sumset`.

///
/// ```
/// use math_prime_util::bigint::sets::sumset;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert_eq!(sumset(&a, &a).len(), 3);
/// ```
pub fn sumset<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    setbinop(a, b, |x, y| x + y)
}

/// Test whether unordered pair sums are distinct.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_is_sidon_set`.

///
/// ```
/// use math_prime_util::bigint::sets::is_sidon_set;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert!(is_sidon_set(&a[..1]));
/// ```
pub fn is_sidon_set<T: Ord + Clone + From<u8>>(a: &[T]) -> bool
where
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    let a = toset(a);
    if a.first().is_some_and(|x| *x < T::from(0_u8)) {
        return false;
    }
    let mut seen = BTreeSet::new();
    for (i, x) in a.iter().enumerate() {
        for y in &a[i..] {
            if !seen.insert(x + y) {
                return false;
            }
        }
    }
    true
}

/// Test whether unordered pair sums are absent from the set.
///
/// Port of MPU 0.75 `reference/mpu/xs_set.c::xs_is_sumfree_set`.

///
/// ```
/// use math_prime_util::bigint::sets::is_sumfree_set;

/// use num_bigint::BigUint;

/// let a = vec![BigUint::from(1_u8), BigUint::from(2_u8)];

/// assert!(is_sumfree_set(&a[..1]));
/// ```
pub fn is_sumfree_set<T: Ord + Clone + From<u8>>(a: &[T]) -> bool
where
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    let a = toset(a);
    for (i, x) in a.iter().enumerate() {
        for y in &a[i..] {
            if a.binary_search(&(x + y)).is_ok() {
                return false;
            }
        }
    }
    true
}

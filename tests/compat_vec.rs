//! Compatibility with the committed MPU 0.75 vector fixtures.
mod common;
use math_prime_util::*;
use std::fmt::Display;
fn list<T: Display>(a: impl IntoIterator<Item = T>) -> String {
    a.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
#[allow(clippy::too_many_lines)] // One dispatch arm per oracle function.
fn oracle(name: &str) {
    let cases = common::load("vec");
    let mut checked = 0;
    for c in cases.iter().filter(|c| {
        c.func == name
            || c.func
                .strip_prefix(name)
                .is_some_and(|s| s.starts_with('_'))
    }) {
        let tokens: Vec<_> = c.args.iter().map(String::as_str).collect();
        let parts: Vec<_> = tokens.split(|s| *s == "|").collect();
        if name == "vecextract" {
            let a = parts[0];
            let result = if c.func == "vecextract_idx" {
                let ix: Vec<isize> = parts[1].iter().map(|s| s.parse().unwrap()).collect();
                vecextract(a, VecSelection::Indices(&ix))
            } else {
                // vecextract masks above u64 require the deferred bigint API.
                let Ok(mask) = parts[1][0].parse::<u64>() else {
                    continue;
                };
                vecextract(a, VecSelection::Mask(mask))
            };
            let result = list(result.into_iter().map(|x| x.unwrap_or("undef")));
            c.check(if result == "undef" {
                None
            } else {
                Some(result)
            });
            checked += 1;
            continue;
        }
        // vecsum, vecprefixsum and vecprod accept u64 only. Signed and bigint inputs are deferred.
        if matches!(name, "vecsum" | "vecprefixsum" | "vecprod") {
            let Ok(a) = parts[0]
                .iter()
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
            else {
                continue;
            };
            let result = match name {
                "vecsum" => vecsum(&a).to_string(),
                "vecprefixsum" => list(vecprefixsum(&a)),
                _ => {
                    // vecprod products above u128 require the deferred bigint API.
                    if !a.contains(&0)
                        && a.iter()
                            .try_fold(1_u128, |p, &x| p.checked_mul(u128::from(x)))
                            .is_none()
                    {
                        continue;
                    }
                    vecprod(&a).to_string()
                }
            };
            c.check(Some(result));
            checked += 1;
            continue;
        }
        // All other native integer APIs use i64. Values outside i64 need bigint support.
        let Ok(a) = parts[0]
            .iter()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
        else {
            continue;
        };
        let b = if parts.len() > 1 {
            let Ok(b) = parts[1]
                .iter()
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()
            else {
                continue;
            };
            b
        } else {
            a.clone()
        };
        // vecmin, vecmax, vecuniq, and vecsingleton preserve Perl scalar spellings.
        // Leading-zero string scalars cannot be represented by the numeric Rust API.
        if matches!(name, "vecmin" | "vecmax" | "vecuniq" | "vecsingleton")
            && parts[0]
                .iter()
                .any(|s| s.parse::<i64>().is_ok_and(|x| x.to_string() != *s))
        {
            continue;
        }
        if c.error && matches!(name, "vecmex" | "vecpmex") {
            assert!(
                std::panic::catch_unwind(|| if name == "vecmex" {
                    vecmex(&a)
                } else {
                    vecpmex(&a)
                })
                .is_err()
            );
            checked += 1;
            continue;
        }
        let result = match name {
            "vecmin" => {
                c.check(vecmin(&a));
                checked += 1;
                continue;
            }
            "vecmax" => {
                c.check(vecmax(&a));
                checked += 1;
                continue;
            }
            "vecuniq" => list(vecuniq(&a)),
            "vecsingleton" => list(vecsingleton(&a)),
            "vecfreq" => list(
                vecfreq(&a)
                    .into_iter()
                    .flat_map(|(x, n)| [x.to_string(), n.to_string()]),
            ),
            "vecsort" => list(vecsort(&a)),
            "vecrsort" => list(vecrsort(&a)),
            "vecsorti" => {
                let mut a = a;
                vecsorti(&mut a);
                list(a)
            }
            "vecrsorti" => {
                let mut a = a;
                vecrsorti(&mut a);
                list(a)
            }
            "vecmex" => vecmex(&a).to_string(),
            "vecpmex" => vecpmex(&a).to_string(),
            "vecequal" => u8::from(vecequal(&a, &b)).to_string(),
            "toset" => list(toset(&a)),
            "sumset" => {
                // sumset outputs outside i64 need the deferred bigint set API.
                if a.iter()
                    .any(|x| b.iter().any(|y| x.checked_add(*y).is_none()))
                {
                    continue;
                }
                list(sumset(&a, &b))
            }
            "setunion" => list(setunion(&a, &b)),
            "setintersect" => list(setintersect(&a, &b)),
            "setminus" => list(setminus(&a, &b)),
            "setdelta" => list(setdelta(&a, &b)),
            "setinsert" | "setremove" | "setinvert" => {
                let mut a = a;
                let n = match name {
                    "setinsert" => setinsert(&mut a, &b),
                    "setremove" => setremove(&mut a, &b),
                    _ => setinvert(&mut a, &b),
                };
                if c.func.ends_with("_n") {
                    n.to_string()
                } else {
                    list(a)
                }
            }
            "setcontains" => u8::from(setcontains(&a, &b)).to_string(),
            "setcontainsany" => u8::from(setcontainsany(&a, &b)).to_string(),
            "is_sidon_set" => u8::from(is_sidon_set(&a)).to_string(),
            "is_sumfree_set" => u8::from(is_sumfree_set(&a)).to_string(),
            "set_is_disjoint" => u8::from(set_is_disjoint(&a, &b)).to_string(),
            "set_is_equal" => u8::from(set_is_equal(&a, &b)).to_string(),
            "set_is_subset" => u8::from(set_is_subset(&a, &b)).to_string(),
            "set_is_proper_subset" => u8::from(set_is_proper_subset(&a, &b)).to_string(),
            "set_is_superset" => u8::from(set_is_superset(&a, &b)).to_string(),
            "set_is_proper_superset" => u8::from(set_is_proper_superset(&a, &b)).to_string(),
            "set_is_proper_intersection" => {
                u8::from(set_is_proper_intersection(&a, &b)).to_string()
            }
            _ => unreachable!(),
        };
        c.check(Some(result));
        checked += 1;
    }
    assert!(checked > 0, "no cases for {name}");
}
mod checks {
    use super::oracle;
    macro_rules! cases {($($n:ident),* $(,)?)=>{$(#[test] fn $n(){oracle(stringify!($n));})*};}
    cases!(
        vecsum,
        vecprefixsum,
        vecprod,
        vecmin,
        vecmax,
        vecextract,
        vecuniq,
        vecfreq,
        vecsingleton,
        vecsort,
        vecrsort,
        vecsorti,
        vecrsorti,
        vecequal,
        vecmex,
        vecpmex,
        toset,
        setinsert,
        setremove,
        setinvert,
        setcontains,
        setcontainsany,
        sumset,
        setunion,
        setintersect,
        setminus,
        setdelta,
        is_sidon_set,
        is_sumfree_set,
        set_is_disjoint,
        set_is_equal,
        set_is_subset,
        set_is_proper_subset,
        set_is_superset,
        set_is_proper_superset,
        set_is_proper_intersection
    );
    #[test]
    fn vecreduce() {
        assert_eq!(
            math_prime_util::vecreduce(&[1, 2, 3], |a, b| a + b),
            Some(6)
        );
    }
    #[test]
    fn vecany() {
        assert!(math_prime_util::vecany(&[1, 2], |&x| x == 1));
    }
    #[test]
    fn vecall() {
        assert!(math_prime_util::vecall(&[1, 2], |&x| x > 0));
    }
    #[test]
    fn vecnone() {
        assert!(math_prime_util::vecnone(&[1, 2], |&x| x == 0));
    }
    #[test]
    fn vecnotall() {
        assert!(math_prime_util::vecnotall(&[1, 2], |&x| x == 1));
    }
    #[test]
    fn vecfirst() {
        assert_eq!(math_prime_util::vecfirst(&[1, 2], |&x| x > 1), Some(&2));
    }
    #[test]
    fn vecfirstidx() {
        assert_eq!(math_prime_util::vecfirstidx(&[1, 2], |&x| x > 1), Some(1));
    }
    #[test]
    fn vecslide() {
        assert_eq!(math_prime_util::vecslide(&[1, 2, 4], |a, b| b - a), [1, 2]);
    }
    #[test]
    fn vecpairwise() {
        assert_eq!(
            math_prime_util::vecpairwise(&[1, 2], &[3], |a, b| [a + b]),
            [4]
        );
    }
    #[test]
    fn vecwindow() {
        assert_eq!(
            math_prime_util::vecwindow(&[1, 2, 3], 1, 2, |w| [w[0] + w[1]]),
            [3, 5]
        );
    }
    #[test]
    fn setbinop() {
        assert_eq!(
            math_prime_util::setbinop(&[1, 2], &[3], |a, b| a + b),
            [4, 5]
        );
    }
}

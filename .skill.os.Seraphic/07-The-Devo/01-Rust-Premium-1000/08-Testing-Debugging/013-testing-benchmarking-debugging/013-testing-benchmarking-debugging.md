# SKILL 013: TESTING, BENCHMARKING & DEBUGGING

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        TESTING & DEBUGGING IN RUST
                     Quality Assurance & Performance
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive mastery of testing in Rust including unit tests, integration tests,
property-based testing, fuzzing, benchmarking, and debugging techniques.

## TABLE OF CONTENTS

1. [Unit Testing](#unit-testing)
2. [Integration Testing](#integration-testing)
3. [Property-Based Testing](#property-based-testing)
4. [Fuzzing](#fuzzing)
5. [Benchmarking](#benchmarking)
6. [Debugging](#debugging)

---

## UNIT TESTING

### 1.1 Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_addition() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn test_with_result() -> Result<(), &'static str> {
        let result = compute()?;
        assert!(result.is_ok());
        Ok(())
    }
    
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This should panic");
    }
    
    #[test]
    #[ignore]
    fn test_expensive() {
        // Expensive test
    }
}
```

---

## PROPERTY-BASED TESTING

### 2.1 QuickCheck

```rust
use quickcheck::{Arbitrary, Gen};

#[derive(Clone, Debug)]
pub struct NonEmptyString(String);

impl Arbitrary for NonEmptyString {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = (1..20).choose(g).unwrap();
        let s: String = (0..len)
            .map(|_| {
                let idx = usize::arbitrary(g) % ALPHABET.len();
                ALPHABET[idx]
            })
            .collect();
        NonEmptyString(s)
    }
}

#[quickcheck]
fn prop_reverse_twice(s: String) -> bool {
    let mut v = s.clone();
    v.reverse();
    v.reverse();
    v == s
}
```

---

## BENCHMARKING

### 3.1 Criterion

```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut data: Vec<i32> = (0..size).collect();
            b.iter(|| {
                data.shuffle(&mut rand::thread_rng());
                data.sort();
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_sort);
criterion_main!(benches);
```

---

## RECAP

1. **Test behavior, not implementation** - Black-box testing
2. **Property-based catches edge cases** - Generate random inputs
3. **Fuzzing finds security bugs** - AFL, libFuzzer
4. **criterion for reliable benchmarks** - Statistical analysis
5. **Profile before optimizing** - Don't guess

---

*Skill ID: 013 | Category: Testing | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*
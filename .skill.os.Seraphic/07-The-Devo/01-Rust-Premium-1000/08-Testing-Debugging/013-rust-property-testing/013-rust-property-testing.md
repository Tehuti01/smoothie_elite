# SKILL 013-B: PROPERTY TESTING & FUZZING

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        PROPERTY TESTING & FUZZING
                     Advanced Testing Techniques
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Property-based testing and fuzzing in Rust with proptest and libFuzzer.

---

## PROPTEST

### 1.1 Custom Arbitrary

```rust
use proptest::{Arbitrary, Strategy, box_std:: Vec};

#[derive(Clone, Debug)]
pub struct NonEmptyString(String);

impl Arbitrary for NonEmptyString {
    type Parameters = ();
    type Strategy = S;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        // Generate strings between 1 and 100 characters
        (1..100)
            .prop_map(|len| {
                let s: String = (0..len)
                    .map(|_| {
                        let idx = usize::arbitrary().any() % CHARS.len();
                        CHARS[idx]
                    })
                    .collect();
                NonEmptyString(s)
            })
    }
}

proptest! {
    #[test]
    fn test_string_reverse_twice(s: String) {
        let mut reversed = s.clone();
        reversed.reverse();
        reversed.reverse();
        prop_assert_eq!(reversed, s);
    }
}
```

---

## FUZZING

### 2.1 libFuzzer Integration

```rust
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Test parsing
        if let Ok(parsed) = parse_input(s) {
            // Test round-trip
            let encoded = encode(&parsed);
            let decoded = decode(&encoded);
            assert_eq!(parsed, decoded);
        }
    }
});
```

---

## RECAP

1. **proptest** - Property-based testing
2. **libFuzzer** - Security testing
3. **Custom Arbitrary** - Domain-specific generation

---

*Skill ID: 013-B | Category: Testing | Complexity: Expert*
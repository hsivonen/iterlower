// Copyright 2019 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Final-sigma-correct lowercasing iterator adapter for iterators
//! over `char`. Turkish/Azeri `'I'` handled optionally.

#![doc(html_root_url = "https://docs.rs/iterlower/1.0.1")]

use std::char::ToLowercase;
use std::collections::VecDeque;
use unic_ucd::CaseIgnorable;
use unic_ucd::Cased;

/// An iterator adapter yielding lower-case `char`s.
#[derive(Debug)]
pub struct Lowercase<I> {
    delegate: I,
    sigma_trailing_case_ignorables: VecDeque<char>,
    sigma_trail: Option<char>,
    lower: ToLowercase,
    prev_cased: bool,
    tr_az: bool,
}

impl<I: Iterator<Item = char>> Iterator for Lowercase<I> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.lower.next() {
            return Some(c);
        }
        if let Some(c) = self.sigma_trailing_case_ignorables.pop_front() {
            return Some(c);
        }
        let c = if let Some(c) = self.sigma_trail {
            self.sigma_trail = None;
            c
        } else if let Some(c) = self.delegate.next() {
            c
        } else {
            return None;
        };
        if self.tr_az && c == 'I' {
            self.prev_cased = true;
            return Some('ı');
        }
        if Cased::of(c).as_bool() {
            if c == 'Σ' && self.prev_cased {
                loop {
                    if let Some(t) = self.delegate.next() {
                        if CaseIgnorable::of(t).as_bool() {
                            self.sigma_trailing_case_ignorables.push_back(t);
                            continue;
                        }
                        self.sigma_trail = Some(t);
                        if Cased::of(t).as_bool() {
                            return Some('σ');
                        }
                    }
                    return Some('ς');
                }
            }
            self.prev_cased = true;
            self.lower = c.to_lowercase();
            return self.lower.next();
        }
        if self.prev_cased && !CaseIgnorable::of(c).as_bool() {
            self.prev_cased = false;
        }
        Some(c)
    }
}

/// Trait that adds a `to_lowercase` method to iterators
/// over `char`.
pub trait IterLowercase<I: Iterator<Item = char>> {
    /// Returns a lower-casing iterator adapter that
    /// handles final sigma correctly.
    ///
    /// `tr_az` set to `true` results in Turkish/Azeri treatment
    /// of `'I'`.
    fn to_lowercase(self, tr_az: bool) -> Lowercase<I>;
}

impl<I: Iterator<Item = char>> IterLowercase<I> for I {
    #[inline]
    fn to_lowercase(self, tr_az: bool) -> Lowercase<I> {
        // Create a consumed `ToLowercase`
        let mut lower = '\0'.to_lowercase();
        lower.next();

        Lowercase {
            delegate: self,
            sigma_trailing_case_ignorables: VecDeque::new(),
            sigma_trail: None,
            lower: lower,
            prev_cased: false,
            tr_az: tr_az,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check(s: &str) {
        assert_eq!(
            s.chars().to_lowercase(false).collect::<String>(),
            s.to_lowercase()
        );
    }

    #[test]
    fn test_sigma() {
        for s in [
            "Σ",
            "Σ\u{0301}",
            "ΣΣ",
            "Σ\u{0301}Σ\u{0301}",
            "ΣΣ ",
            "Σ\u{0301}Σ\u{0301} ",
            " Σ",
            " Σ\u{0301}",
            "ΣΣ-",
            "Σ\u{0301}Σ\u{0301}-",
            "-Σ",
            "-Σ\u{0301}",
            "ΣΣ猪",
            "Σ\u{0301}Σ\u{0301}猪",
            "猪Σ",
            "猪Σ\u{0301}",
            "ΣΣB",
            "Σ\u{0301}Σ\u{0301}B",
            "BΣ",
            "BΣ\u{0301}",
            "ΣΣΔ",
            "Σ\u{0301}Σ\u{0301}Δ",
            "ΔΣ",
            "ΔΣ\u{0301}",
        ]
        .iter()
        {
            check(s);
        }
    }

    #[test]
    fn test_i() {
        assert_eq!(
            "ΣIΣ".chars().to_lowercase(true).collect::<String>(),
            "σıς"
        );
    }

    #[test]
    fn test_uncased() {
        check("猪猪");
    }

}

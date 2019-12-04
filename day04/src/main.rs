// --- Day 4: Secure Container ---
//
// You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.
//
// However, they do remember a few key facts about the password:
//
//     It is a six-digit number.
//     The value is within the range given in your puzzle input.
//     Two adjacent digits are the same (like 22 in 122345).
//     Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
//
// Other than the range rule, the following are true:
//
//     111111 meets these criteria (double 11, never decreases).
//     223450 does not meet these criteria (decreasing pair of digits 50).
//     123789 does not meet these criteria (no double).
//
// How many different passwords within the range given in your puzzle input meet these criteria?
//
// Your puzzle input is 402328-864247.
//
// --- Part Two ---
//
// An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.
//
// Given this additional criterion, but still ignoring the range rule, the following are now true:
//
//     112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
//     123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
//     111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
//
// How many different passwords within the range given in your puzzle input meet all of the criteria?
//

use std::collections::HashMap;

fn main() {
    let mut criteria_met = 0;

    for i in 402328..864247 {
        let mut map = HashMap::new();
        let s = i.to_string();
        s.chars().for_each(|c| {
            let counter = map.entry(c).or_insert(0);
            *counter += 1;
        });
        let c1 = map.values().any(|v| *v == 2);

        let mut compare_to = 0;
        let c2 = !s.chars().clone().any(|c| {
            let n = c.to_digit(10).unwrap();
            let x = n < compare_to;
            compare_to = n;
            x
        });

        if c1 && c2 {
            criteria_met += 1;
        }
    }

    println!("{:?}", criteria_met);
}

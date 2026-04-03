//! Exhaustive and type-varied tests for [`crate::model::app::apply_permutation`].

use std::path::PathBuf;

use crate::model::app::apply_permutation;

/// All permutations of `0..n` (insertion method; `n` ≤ 8 keeps test count reasonable).
fn all_permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    for p in all_permutations(n - 1) {
        for pos in 0..=p.len() {
            let mut q = p.clone();
            q.insert(pos, n - 1);
            out.push(q);
        }
    }
    out
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[test]
fn apply_permutation_identity() {
    let mut v = vec![1, 2, 3, 4];
    apply_permutation(&mut v, &[0, 1, 2, 3]);
    assert_eq!(v, vec![1, 2, 3, 4]);
}

#[test]
fn apply_permutation_reverse() {
    let mut v = vec![10, 20, 30];
    apply_permutation(&mut v, &[2, 1, 0]);
    assert_eq!(v, vec![30, 20, 10]);
}

#[test]
fn apply_permutation_rotate_left() {
    let mut v = vec!['a', 'b', 'c', 'd'];
    apply_permutation(&mut v, &[1, 2, 3, 0]);
    assert_eq!(v, vec!['b', 'c', 'd', 'a']);
}

#[test]
fn apply_permutation_single_element() {
    let mut v = vec![42];
    apply_permutation(&mut v, &[0]);
    assert_eq!(v, vec![42]);
}

#[test]
fn apply_permutation_reorders_option_strings() {
    let mut v: Vec<Option<String>> = vec![
        Some("a".to_string()),
        Some("b".to_string()),
        Some("c".to_string()),
    ];
    apply_permutation(&mut v, &[2, 0, 1]);
    assert_eq!(
        v,
        vec![
            Some("c".to_string()),
            Some("a".to_string()),
            Some("b".to_string())
        ]
    );
}

#[test]
fn apply_permutation_empty_vec_and_indices() {
    let mut v: Vec<i32> = vec![];
    apply_permutation(&mut v, &[]);
    assert!(v.is_empty());
}

#[test]
fn apply_permutation_all_permutations_n1_to_n6_identity_values() {
    for n in 1..=6 {
        let perms = all_permutations(n);
        assert_eq!(perms.len(), factorial(n));
        for perm in perms {
            let mut v: Vec<usize> = (0..n).collect();
            apply_permutation(&mut v, &perm);
            assert_eq!(v, perm, "n={n} perm={perm:?}");
        }
    }
}

#[test]
fn apply_permutation_all_permutations_n7_identity_values() {
    let n = 7;
    let perms = all_permutations(n);
    assert_eq!(perms.len(), factorial(n));
    for perm in perms {
        let mut v: Vec<usize> = (0..n).collect();
        apply_permutation(&mut v, &perm);
        assert_eq!(v, perm);
    }
}

#[test]
fn apply_permutation_all_six_permutations_n3_explicit() {
    let base = vec![100, 200, 300];
    let expected: &[(&[usize], &[i32])] = &[
        (&[0, 1, 2], &[100, 200, 300]),
        (&[0, 2, 1], &[100, 300, 200]),
        (&[1, 0, 2], &[200, 100, 300]),
        (&[1, 2, 0], &[200, 300, 100]),
        (&[2, 0, 1], &[300, 100, 200]),
        (&[2, 1, 0], &[300, 200, 100]),
    ];
    for (perm, exp) in expected {
        let mut v = base.clone();
        apply_permutation(&mut v, perm);
        assert_eq!(v, *exp);
    }
}

#[test]
fn apply_permutation_swap_adjacent_pair() {
    let mut v: Vec<u8> = (0..16).collect();
    apply_permutation(&mut v, &[1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 0);
}

#[test]
fn apply_permutation_pathbufs() {
    let mut v = vec![
        PathBuf::from("/a"),
        PathBuf::from("/b"),
        PathBuf::from("/c"),
    ];
    apply_permutation(&mut v, &[2, 0, 1]);
    assert_eq!(
        v,
        vec![
            PathBuf::from("/c"),
            PathBuf::from("/a"),
            PathBuf::from("/b"),
        ]
    );
}

#[test]
fn apply_permutation_strings() {
    let mut v = vec![
        String::from("x"),
        String::from("y"),
        String::from("z"),
    ];
    apply_permutation(&mut v, &[1, 2, 0]);
    assert_eq!(v, vec!["y", "z", "x"]);
}

#[test]
fn apply_permutation_bool() {
    let mut v = vec![true, false, true];
    apply_permutation(&mut v, &[2, 1, 0]);
    assert_eq!(v, vec![true, false, true]);
}

#[test]
fn apply_permutation_i128() {
    let mut v = vec![1_i128, 2, 3, 4];
    apply_permutation(&mut v, &[3, 2, 1, 0]);
    assert_eq!(v, vec![4, 3, 2, 1]);
}

#[test]
fn apply_permutation_char_unicode() {
    let mut v = vec!['α', 'β', 'γ'];
    apply_permutation(&mut v, &[2, 0, 1]);
    assert_eq!(v, vec!['γ', 'α', 'β']);
}

#[test]
fn apply_permutation_option_none_heavy() {
    let mut v: Vec<Option<i32>> = vec![Some(1), None, Some(3), None];
    apply_permutation(&mut v, &[3, 2, 1, 0]);
    assert_eq!(v, vec![None, Some(3), None, Some(1)]);
}

#[test]
fn apply_permutation_two_element_swap() {
    let mut v = vec!["first", "second"];
    apply_permutation(&mut v, &[1, 0]);
    assert_eq!(v, vec!["second", "first"]);
}

#[test]
fn apply_permutation_n8_sample() {
    let n = 8;
    let mut v: Vec<u32> = (0..n as u32).collect();
    let perm = vec![7, 6, 5, 4, 3, 2, 1, 0];
    apply_permutation(&mut v, &perm);
    assert_eq!(v, vec![7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
fn apply_permutation_n16_full_reverse() {
    let n = 16;
    let perm: Vec<usize> = (0..n).rev().collect();
    let mut v: Vec<usize> = (0..n).collect();
    apply_permutation(&mut v, &perm);
    assert_eq!(v, perm);
}

#[test]
fn apply_permutation_double_roundtrip_is_identity() {
    let n = 5;
    let perm = vec![2, 4, 0, 1, 3];
    let inv: Vec<usize> = (0..n)
        .map(|j| perm.iter().position(|&x| x == j).unwrap())
        .collect();
    let mut v: Vec<i32> = (0..n as i32).collect();
    apply_permutation(&mut v, &perm);
    apply_permutation(&mut v, &inv);
    assert_eq!(v, vec![0, 1, 2, 3, 4]);
}

#[test]
fn apply_permutation_preserves_length() {
    let mut v = vec![1, 2, 3, 4, 5];
    apply_permutation(&mut v, &[4, 3, 2, 1, 0]);
    assert_eq!(v.len(), 5);
}

#[test]
fn apply_permutation_all_permutations_n8_identity_values() {
    let n = 8;
    let perms = all_permutations(n);
    assert_eq!(perms.len(), factorial(n));
    for perm in perms {
        let mut v: Vec<usize> = (0..n).collect();
        apply_permutation(&mut v, &perm);
        assert_eq!(v, perm);
    }
}

#[test]
fn apply_permutation_shift_right_cyclic_three() {
    let mut v = vec![10_u64, 20, 30];
    apply_permutation(&mut v, &[2, 0, 1]);
    assert_eq!(v, vec![30, 10, 20]);
}

#[test]
fn apply_permutation_f32() {
    let mut v = vec![1.5_f32, 2.5, 3.5];
    apply_permutation(&mut v, &[1, 0, 2]);
    assert_eq!(v, vec![2.5, 1.5, 3.5]);
}

#[test]
fn apply_permutation_boxed_integers() {
    let mut v: Vec<Box<i32>> = vec![Box::new(1), Box::new(2), Box::new(3)];
    apply_permutation(&mut v, &[2, 1, 0]);
    assert_eq!(*v[0], 3);
    assert_eq!(*v[1], 2);
    assert_eq!(*v[2], 1);
}

#[test]
fn apply_permutation_n10_sample_permutations() {
    let n = 10;
    let samples: &[&[usize]] = &[
        &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        &[1, 0, 3, 2, 5, 4, 7, 6, 9, 8],
        &[5, 4, 3, 2, 1, 0, 9, 8, 7, 6],
        &[0, 2, 4, 6, 8, 1, 3, 5, 7, 9],
    ];
    for perm in samples {
        let mut v: Vec<usize> = (0..n).collect();
        apply_permutation(&mut v, perm);
        assert_eq!(v, *perm);
    }
}

#[test]
fn apply_permutation_u16_values() {
    let mut v: Vec<u16> = vec![100, 200, 300, 400];
    apply_permutation(&mut v, &[3, 0, 2, 1]);
    assert_eq!(v, vec![400, 100, 300, 200]);
}

#[test]
fn apply_permutation_isize_slice() {
    let mut v: Vec<isize> = vec![-1, -2, -3];
    apply_permutation(&mut v, &[2, 0, 1]);
    assert_eq!(v, vec![-3, -1, -2]);
}

#[test]
fn apply_permutation_unit_type() {
    let mut v = vec![(), (), ()];
    apply_permutation(&mut v, &[2, 1, 0]);
    assert_eq!(v.len(), 3);
}

#[test]
fn apply_permutation_n12_reverse_only() {
    let n = 12;
    let perm: Vec<usize> = (0..n).rev().collect();
    let mut v: Vec<i32> = (0..n as i32).collect();
    apply_permutation(&mut v, &perm);
    assert_eq!(v, (0..n as i32).rev().collect::<Vec<_>>());
}

#[test]
fn apply_permutation_string_slices_reorder() {
    let mut v = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
    ];
    apply_permutation(&mut v, &[1, 2, 0]);
    assert_eq!(
        v,
        vec![
            "beta".to_string(),
            "gamma".to_string(),
            "alpha".to_string()
        ]
    );
}


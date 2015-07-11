//! This module provides a generic backend of vector primitives
//! that the compiler will hopefully optimize down to vectorized code.
//! It's possible that not everything will be vectorized as well as we'd
//! like though.

use std::ops::{Index, IndexMut, Add, Sub, BitAnd, BitOr};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    v: [f32; 8],
}

impl Vector {
    pub fn new() -> Vector {
        Vector { v: [0.0; 8] }
    }
    pub fn broadcast(x: f32) -> Vector {
        Vector { v: [x; 8] }
    }
    pub fn load(v: [f32; 8]) -> Vector {
        Vector { v: v }
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, r: Vector) -> Vector {
        let mut c = Vector::new();
        for i in 0..8 {
            c[i] = self[i] + r[i];
        }
        c
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, r: Vector) -> Vector {
        let mut c = Vector::new();
        for i in 0..8 {
            c[i] = self[i] - r[i];
        }
        c
    }
}

impl Index<usize> for Vector {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        &self.v[i]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.v[i]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Mask {
    v: [bool; 8],
}

impl Mask {
    pub fn new() -> Mask {
        Mask { v: [true; 8] }
    }
    pub fn broadcast(x: bool) -> Mask {
        Mask { v: [x; 8] }
    }
    pub fn load(v: [bool; 8]) -> Mask {
        Mask { v: v }
    }
}

impl BitAnd for Mask {
    type Output = Mask;
    fn bitand(self, r: Mask) -> Mask {
        let mut c = Mask::new();
        for i in 0..8 {
            c[i] = self[i] & r[i];
        }
        c
    }
}

impl BitOr for Mask {
    type Output = Mask;
    fn bitor(self, r: Mask) -> Mask {
        let mut c = Mask::new();
        for i in 0..8 {
            c[i] = self[i] | r[i];
        }
        c
    }
}

impl Index<usize> for Mask {
    type Output = bool;
    fn index(&self, i: usize) -> &bool {
        &self.v[i]
    }
}

impl IndexMut<usize> for Mask {
    fn index_mut(&mut self, i: usize) -> &mut bool {
        &mut self.v[i]
    }
}

/// Blend between a and b based on the values in the mask passed. Returns
/// a new vector where element i is b[i] if mask[i] is true and a[i]
/// if mask[i] is false
pub fn blend(a: &Vector, b: &Vector, mask: &Mask) -> Vector {
    let mut c = Vector::new();
    for i in 0..8 {
        if mask[i] {
            c[i] = b[i];
        } else {
            c[i] = a[i];
        }
    }
    c
}

#[test]
fn test_add() {
    let x = Vector::load([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let y = Vector::load([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
    let z = x + y;
    assert_eq!(z[0], 10.0);
    assert_eq!(z[1], 12.0);
    assert_eq!(z[2], 14.0);
    assert_eq!(z[3], 16.0);
    assert_eq!(z[4], 18.0);
    assert_eq!(z[5], 20.0);
    assert_eq!(z[6], 22.0);
    assert_eq!(z[7], 24.0);
}

#[test]
fn test_mask_and() {
    let x = Mask::load([true, false, true, false, true, false, true, false]);
    let y = Mask::load([false, true, true, false, false, true, true, true]);
    let z = x & y;
    assert_eq!(z[0], false);
    assert_eq!(z[1], false);
    assert_eq!(z[2], true);
    assert_eq!(z[3], false);
    assert_eq!(z[4], false);
    assert_eq!(z[5], false);
    assert_eq!(z[6], true);
    assert_eq!(z[7], false);
}


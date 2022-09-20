use std::{
    fmt::{Debug, Display},
    ops::{AddAssign, SubAssign},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> {
    pub(super) width: u8,
    pub(super) height: u8,
    pub(super) data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Display + AddAssign + SubAssign + Copy,
{
    pub fn new(v_2d: Vec<Vec<T>>) -> Self {
        let width = v_2d[0].len() as u8;
        let height = v_2d.len() as u8;
        let mut data = Vec::with_capacity(width as usize * height as usize);
        v_2d.into_iter().for_each(|mut v_1d| {
            data.append(&mut v_1d);
        });

        Self {
            width,
            height,
            data,
        }
    }

    #[inline]
    fn row_pos(&self, x: u8) -> u64 {
        x as u64 * self.width as u64
    }

    #[inline]
    fn element_pos(&self, x: u8, y: u8) -> usize {
        (self.row_pos(x) + y as u64) as usize
    }

    pub fn at(&self, x: u8, y: u8) -> &T {
        &self.data[self.element_pos(x, y)]
    }

    pub fn at_mut(&mut self, x: u8, y: u8) -> &mut T {
        let cursor = self.element_pos(x, y);
        &mut self.data[cursor]
    }

    fn is_same_size(&self, dst: &Matrix<T>) -> bool {
        if self.width != dst.width {
            return false;
        }
        if self.height != dst.height {
            return false;
        }
        true
    }

    fn same_size_check(&self, dst: &Matrix<T>) -> Result<(), ()> {
        if self.is_same_size(dst) {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn add(&mut self, dst: &Matrix<T>) -> Result<(), ()> {
        self.same_size_check(dst)?;

        for i in 0..self.height {
            for j in 0..self.width {
                *self.at_mut(i, j) += *dst.at(i, j);
            }
        }
        Ok(())
    }

    pub fn diff(&mut self, dst: &Matrix<T>) -> Result<(), ()> {
        self.same_size_check(dst)?;

        for i in 0..self.height {
            for j in 0..self.width {
                *self.at_mut(i, j) -= *dst.at(i, j);
            }
        }
        Ok(())
    }

    pub fn flatten(self) -> Vec<T> {
        self.data
    }

    pub fn print(&self) {
        println!("-----------------");
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{} ", self.at(i, j));
            }
            println!();
        }
        // println!("\r]");
        println!("-----------------");
    }
}

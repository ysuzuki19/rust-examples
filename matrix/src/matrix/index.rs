use std::ops::Index;

use super::Matrix;

impl<T> Index<u8> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: u8) -> &Self::Output {
        let pos = (self.width as usize) * (index as usize);
        &self.data[pos..(pos + self.width as usize)]
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn get_row() {
        let mat = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];
        assert_eq!(mat[0], vec![0, 1, 2]);
        assert_eq!(mat[1], vec![3, 4, 5]);
        assert_eq!(mat[2], vec![6, 7, 8]);
        assert_eq!(mat, matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]]);
    }
}

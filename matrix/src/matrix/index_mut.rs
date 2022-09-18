use std::ops::IndexMut;

use super::Matrix;

impl<T> IndexMut<u8> for Matrix<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        let pos = (self.width as usize) * (index as usize);
        &mut self.data[pos..(pos + self.width as usize)]
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn get_row() {
        let mut mat = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];
        mat[0][0] = 1;
        println!("{:?}", mat);
        assert_eq!(mat[0], vec![1, 1, 2]);
        assert_eq!(mat, matrix![[1, 1, 2], [3, 4, 5], [6, 7, 8]]);
    }
}

#[macro_export]
macro_rules! matrix {
    [$($elements:expr), *] => {
        {
            let mut vec_2d = vec![];
            $(
              vec_2d.push($elements.to_vec());
            )*
            $crate::matrix::Matrix::new(vec_2d)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn can_create_matrix_1x1() {
        let mat = matrix![[1]];
        assert_eq!(mat, Matrix::new(vec![vec![1]]));
    }

    #[test]
    fn can_create_matrix_1x2() {
        let mat = matrix![[1], [2]];
        assert_eq!(mat, Matrix::new(vec![vec![1], vec![2]]));
    }

    #[test]
    fn can_create_matrix_2x2() {
        let mat = matrix![[1, 3], [2, 4]];
        assert_eq!(mat, Matrix::new(vec![vec![1, 3], vec![2, 4]]));
    }

    #[test]
    fn can_create_matrix_3x3() {
        let mat = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];
        assert_eq!(
            mat,
            Matrix::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]])
        );
    }
}

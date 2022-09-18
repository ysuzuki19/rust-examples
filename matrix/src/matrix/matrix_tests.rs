#[cfg(test)]
mod tests {
    mod new {
        use std::{
            fmt::{Debug, Display},
            ops::{AddAssign, SubAssign},
        };

        use crate::matrix::Matrix;

        fn unit<T>(input: Vec<Vec<T>>, width: u8, height: u8, result: Vec<T>)
        where
            T: PartialEq + Debug + Copy + AddAssign + SubAssign + Display,
        {
            let mat = Matrix::<T>::new(input);
            assert_eq!(mat.width, width);
            assert_eq!(mat.height, height);
            let zipped = mat.data.iter().zip(result.iter());
            for (mv, rv) in zipped.into_iter() {
                assert_eq!(mv, rv);
            }
        }

        #[test]
        fn i32_1x1() {
            unit(vec![vec![0]], 1, 1, vec![0]);
        }
        #[test]
        fn i32_1x2() {
            unit(vec![vec![0], vec![1]], 1, 2, vec![0, 1]);
        }
        #[test]
        fn i32_2x1() {
            unit(vec![vec![0, 1]], 2, 1, vec![0, 1]);
        }
        #[test]
        fn i32_2x2() {
            unit(vec![vec![0, 1], vec![2, 3]], 2, 2, vec![0, 1, 2, 3]);
        }
        #[test]
        fn i32_3x3() {
            let input = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            let result = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            unit(input, 3, 3, result);
        }

        #[test]
        fn f32_1x1() {
            unit(vec![vec![0.0_f32]], 1, 1, vec![0.0_f32]);
        }

        #[test]
        fn f64_1x1() {
            unit(vec![vec![0.0_f64]], 1, 1, vec![0.0_f64]);
        }
    }

    #[test]
    fn at() {
        let mat = matrix![[0, 1], [2, 3]];
        assert_eq!(*mat.at(0, 0), 0);
        assert_eq!(*mat.at(0, 1), 1);
        assert_eq!(*mat.at(1, 0), 2);
        assert_eq!(*mat.at(1, 1), 3);
    }

    #[test]
    fn at_mut() {
        let mut mat = matrix![[0, 1], [2, 3]];
        *mat.at_mut(0, 0) = 1;
        assert_eq!(*mat.at(0, 0), 1);
    }

    #[test]
    fn flatten() {
        let mat = matrix![[0, 1], [2, 3]];
        assert_eq!(mat.flatten(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn add() {
        let mut mat = matrix![[0, 1], [2, 3]];
        let res = mat.add(&matrix![[3, 2], [1, 0]]);
        assert!(res.is_ok());
        assert_eq!(mat, matrix![[3, 3], [3, 3]]);
    }

    #[test]
    fn diff() {
        let mut mat = matrix![[3, 3], [3, 3]];
        let res = mat.diff(&matrix![[3, 2], [1, 0]]);
        assert!(res.is_ok());
        assert_eq!(mat, matrix![[0, 1], [2, 3]]);
    }
}

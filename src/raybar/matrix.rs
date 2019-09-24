/*
  matrix primatives for raybar

  todo
  create NxM matrix
  where N -> row
  where M -> col
  where for matrix A, A[n][m] yields the item at row n, col m

*/
extern crate num_traits;
use super::glprimative::{GlPrimative, MatrixNumber};
use super::point::{GlPoint};
use super::vector::{GlVector};
//use std::f64::consts::PI;
//use std::num::{FpCategory};
use num_traits::{One, ToPrimitive, Zero, Float};
use std::ops::{Add, Div, Mul, Sub, Neg};


#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct GlMatrix<T: MatrixNumber> {
    content: Vec<Vec<T>>,
}

impl<
        T: MatrixNumber
    > GlMatrix<T>
{
    #[allow(dead_code)]
    pub fn new(content: Vec<Vec<T>>) -> GlMatrix<T> {
        // for each sup vector, the length must be consitent
        if GlMatrix::vec_sizing(&content) {
            GlMatrix { content: content }
        } else {
            panic!("all rows must be the same size!");
        }
    }
    /*
        generates an identity matrix for a given piece of code
    */
    #[allow(dead_code)]
    pub fn identity(size: usize) -> GlMatrix<T> {
        let mut rows = vec![];
        for i in 0..size {
            let mut row: Vec<T> = vec![];
            for j in 0..size {
                if j == i {
                    row.push(One::one());
                } else {
                    row.push(Zero::zero());
                }
            }
            rows.push(row);
        }
        GlMatrix::new(rows)
    }
    #[allow(dead_code)]
    pub fn translation(x: T, y: T, z: T) -> GlMatrix<T> {
        GlMatrix::new(vec![
            vec![One::one(), Zero::zero(), Zero::zero(), x],
            vec![Zero::zero(), One::one(), Zero::zero(), y],
            vec![Zero::zero(), Zero::zero(), One::one(), z],
            vec![Zero::zero(), Zero::zero(), Zero::zero(), One::one()]
        ])
    }
    #[allow(dead_code)]
    pub fn scaling(x: T, y: T, z: T) -> GlMatrix<T> {
        GlMatrix::new(vec![
            vec![x, Zero::zero(), Zero::zero(), Zero::zero()],
            vec![Zero::zero(), y, Zero::zero(), Zero::zero()],
            vec![Zero::zero(), Zero::zero(), z, Zero::zero()],
            vec![Zero::zero(), Zero::zero(), Zero::zero(), One::one()]
        ])
    }
    #[allow(dead_code)]
    pub fn rotate_x(r: T) -> GlMatrix<T> {
        GlMatrix::new(vec![
            vec![One::one(), Zero::zero(), Zero::zero(), Zero::zero()],
            vec![Zero::zero(), r.cos(), -(r.sin()), Zero::zero()],
            vec![Zero::zero(), r.sin(),     r.cos(), Zero::zero()],
            vec![Zero::zero(), Zero::zero(), Zero::zero(), One::one()],
        ])
    }
    pub fn rotate_y(r: T) -> GlMatrix<T> {
        GlMatrix::new(vec![
            vec![r.cos(), Zero::zero(), r.sin(), Zero::zero()],
            vec![Zero::zero(), One::one(), Zero::zero(), Zero::zero()],
            vec![-(r.sin()), Zero::zero(), r.cos(), Zero::zero()],
            vec![Zero::zero(), Zero::zero(), Zero::zero(), One::one()]
        ])
    }
    pub fn rotate_z(r: T) -> GlMatrix<T> {
        GlMatrix::new(vec![
            vec![r.cos(), -(r.sin()), Zero::zero(), Zero::zero()],
            vec![r.sin(),   r.cos(),  Zero::zero(), Zero::zero()],
            vec![Zero::zero(), Zero::zero(), One::one(), Zero::zero()],
            vec![Zero::zero(), Zero::zero(), Zero::zero(), One::one()]
        ])
    }
    #[allow(dead_code)]
    pub fn map<F, P: MatrixNumber>(&self, func: F) -> GlMatrix<P>
    where F: Fn((usize, usize), &T) -> P {
        let contents = self.content
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row
                .iter()
                .enumerate()
                .map(|(j, val)| {
                    func((i, j), val)
                })
                .collect()
            })
            .collect();
        GlMatrix::new(contents)
    }
    #[allow(dead_code)]
    pub fn add(&self, rhs: GlMatrix<T>) -> Result<GlMatrix<T>, &'static str> {
        if self.get_dimensions() != rhs.get_dimensions() {
            Err("incompatible dimensions")
        } else {
            let sum = self.map(|(i, j), val| {
                let left = val;
                let right = rhs.get(i, j);
                //println!("({:?} {:?}) => {:?} + {:?} = {:?}", i, j, left, right, *left + *right);
                *left + *right
            });
           Ok(sum)
        }
    }
    #[allow(dead_code)]
    pub fn sub(&self, rhs: GlMatrix<T>) -> Result<GlMatrix<T>, &'static str> {
        if self.get_dimensions() != rhs.get_dimensions() {
            Err("incompatible dimensions")
        } else {
            let sum = self.map(|(i, j), val| {
                let left = val;
                let right = rhs.get(i, j);
                //println!("({:?} {:?}) => {:?} + {:?} = {:?}", i, j, left, right, *left + *right);
                *left - *right
            });
           Ok(sum)
        }
    }
    #[allow(dead_code)]
    pub fn transpose(&self) -> GlMatrix<T> {
        let mut rows = vec![];
        for j in 0..self.get_col_size() {
            let column = self.get_column(j);
            rows.push(column);
        }
        GlMatrix::new(rows)
    }
    #[allow(dead_code)]
    pub fn get(&self, n: usize, m: usize) -> &T {
        &self.content[n][m]
    }
    #[allow(dead_code)]
    pub fn set(&mut self, n: usize, m: usize, value: T) {
        self.content[n][m] = value;
    }
    #[allow(dead_code)]
    pub fn get_row_size(&self) -> usize {
        self.content[0].len()
    }
    #[allow(dead_code)]
    pub fn get_col_size(&self) -> usize {
        self.content.len()
    }
    #[allow(dead_code)]
    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.get_row_size(), self.get_col_size())
    }
    #[allow(dead_code)]
    pub fn is_square(&self) -> bool {
        let (m, n) = self.get_dimensions();
        m == n
    }
    

    #[allow(dead_code)]
    pub fn get_row(&self, index: usize) -> Vec<T> {
        self.content[index].clone()
    }
    #[allow(dead_code)]
    pub fn get_column(&self, index: usize) -> Vec<T> {
        // for each row, get the ith value and push it into the vector
        let mut column: Vec<T> = vec![];
        for row in self.content.clone().into_iter() {
            column.push(row[index].clone());
        }
        column
    }
    /*
        removes column and row from the matrix and returns a new smaller matrix
    */
    #[allow(dead_code)]
    pub fn submatrix(&self, row: usize, col: usize) -> GlMatrix<T> {
        let new_contents: Vec<Vec<T>> = self
            .content
            .clone()
            .iter()
            .enumerate()
            .filter(|&(row_i, _column)| row_i != row)
            .map(|(_i, column)| {
                // remve the value at col
                column
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != col)
                    .map(|(_j, col)| col.clone())
                    .collect()
            })
            .collect();
        GlMatrix::new(new_contents)
    }
    #[allow(dead_code)]
    pub fn cofactor(&self, row: usize, col: usize) -> T {
        let minor: T = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
    #[allow(dead_code)]
    pub fn minor(&self, row: usize, col: usize) -> T {
        self.submatrix(row, col).det()
    }
    pub fn det(&self) -> T {
        if self.is_square() {
            self.determinate()
        } else {
            panic!("non square matrices have no determinant!");
        }
    }
    #[allow(dead_code)]
    pub fn is_invertible(&self) -> bool {
        self.det() != Zero::zero()
    }
    #[allow(dead_code)]
    pub fn invert(&self) -> Option<GlMatrix<f64>> {
        let size = self.get_col_size();
        let det = self.det(); //determing the deterinate
                              // a zero determineate indicates there is no inverse to be had
        if !self.is_square() {
            Option::None
        } else if det == Zero::zero() {
            Option::None
        } else {
            let mut invert_contents: Vec<Vec<f64>> = (0..size)
                .map(|_row| (0..size).map(|_val| 0.0).collect())
                .collect();
            let floating_det = det.to_f64().unwrap();
            for row in 0..size {
                for col in 0..size {
                    invert_contents[col][row] = self
                        .cofactor(row, col)
                        .to_f64()
                        .map(|cofactor| cofactor / floating_det)
                        .unwrap()
                }
            }

            Option::Some(GlMatrix::new(invert_contents))
        }
    }
    /*
        dumps the data structre into an array of f64s for the prupose of raytracing
    */
    #[allow(dead_code)]
    pub fn to_floats(&self) -> GlMatrix<f64> {
        let contents = self
            .content
            .iter()
            .map(|row| {
                row.iter()
                    .map(|val| (*val).to_f64().unwrap_or(0.0))
                    .collect()
            })
            .collect();
        GlMatrix::new(contents)
    }
        #[allow(dead_code)]
    fn dot_list(row: &Vec<T>, col: &Vec<T>) -> Option<T> {
        if row.len() == 0 {
            return None; //hack for dealing with null generics
        }
        if row.len() == col.len() {
            let mut i = 0;
            let mut acc = row[i].clone() * col[i].clone();
            loop {
                i = i + 1;
                if i == row.len() {
                    break;
                } else {
                    acc = acc + (row[i].clone() * col[i].clone());
                }
            }
            return Option::Some(acc);
        } else {
            panic!("Invalid Matrix ");
        }
    }

    #[allow(dead_code)]
    fn determinate(&self) -> T {
        if self.get_row_size() == 2 {
            self.det_base()
        } else {
            //panic!("cannpt comput greater than 2 yet");
            let size = self.get_col_size();
            (0..size)
                .into_iter()
                .map(|i| self.cofactor(0, i) * *self.get(0, i))
                .fold(Zero::zero(), |acc, cofactor| acc + cofactor)
        }
    }
    #[allow(dead_code)]
    fn det_base(&self) -> T {
        //for the case where the matrix is 2 x 2
        let a = &self.content[0][0];
        let d = &self.content[1][1];
        let b = &self.content[0][1];
        let c = &self.content[1][0];
        // we deref them here for borrowing
        let ret = (*a * *d) - (*b * *c);
        ret
    }
    #[allow(dead_code)]
    fn vec_sizing(list: &Vec<Vec<T>>) -> bool {
        /*
          for a list of sub lists, if the list's length is 0, return false
          if the list is one, return true
          if the list is greater than that,
            for each list in list, store the size, if it changes, return false
        */
        match list.len() {
            0 => false,
            1 => {
                if list[0].len() == 0 {
                    false
                } else {
                    true
                }
            }
            _ => {
                let acc = list[0].len();
                for sub_list in list[1..list.len()].into_iter() {
                    if acc != sub_list.len() {
                        return false;
                    }
                }
                return true;
            }
        }
    }

    #[allow(dead_code)]
    pub fn mult(&self, scaler: T) -> GlMatrix<T> {
        let contents = self
            .content
            .iter()
            .map(|row| row.iter().map(|val| *val * scaler).collect())
            .collect();
        GlMatrix::new(contents)
    }

}

impl<
        T: MatrixNumber
    > GlPrimative<T, GlMatrix<T>> for GlMatrix<T> {
    type Output = GlMatrix<T>;
    //type Input = GlMatrix<T>;

    #[allow(dead_code)]
    fn dot(&self, b: &GlMatrix<T>) -> Self::Output {
        let mut contents: Vec<Vec<T>> = vec![];
        //for each row, compute all the values
        let row_size = self.get_row_size();
        let a_col_size = self.get_col_size();
        let col_size = b.get_col_size();
        let b_row_size = b.get_row_size();
        // row size must equal col size or we should panic as it is an invalid operation
        if row_size != col_size {
            panic!("invalid matrix operation: invalid dimensions for dot product");
        } else {
            let mut i = 0;
            loop {
                //iterate through each row and grab the jth col
                if i == a_col_size
                 {
                    break;
                } else {
                    let row: Vec<T> = self.get_row(i);
                    let mut new_row: Vec<T> = vec![];
                    let mut j = 0;
                    loop {
                        if j == b_row_size {  // we should be iterating till we get the last col of the row
                            break;
                        } else {
                            let column = b.get_column(j);
                            let dot_product_i_j =
                                GlMatrix::dot_list(&row, &column).unwrap_or(Zero::zero());
                            new_row.push(dot_product_i_j);
                            j = j + 1;
                        }
                    }
                    contents.push(new_row);
                    i = i + 1;
                }
            }
            return GlMatrix::new(contents);
        }
    }


}


impl<
        T: MatrixNumber
    > GlPrimative<T, GlPoint<T>> for GlMatrix<T> {
    type Output = GlPoint<T>;

    fn dot(&self, rhs: &GlPoint<T>) -> Self::Output {
        //rhs.clone()
        let matrix = self.dot(rhs.get_matrix());
        GlPoint::from_matrix(matrix)
    }

}

impl<
        T: MatrixNumber
    > GlPrimative<T, GlVector<T>> for GlMatrix<T> {
    type Output = GlVector<T>;

    fn dot(&self, rhs: &GlVector<T>) -> Self::Output {
        //rhs.clone()
        let matrix = self.dot(rhs.get_matrix());
        GlVector::from_matrix(matrix)
    }
}

impl<T: MatrixNumber> Mul<T> for GlMatrix<T> {
    type Output = GlMatrix<T>;
    fn mul(self, rhs: T) -> GlMatrix<T> {
        self.mult(rhs)
    }
}

impl<T: MatrixNumber> Mul<GlMatrix<T>> for GlMatrix<T> {
    type Output = GlMatrix<T>;
    fn mul(self, rhs: GlMatrix<T>) -> GlMatrix<T> {
        self.dot(&rhs)
    }
}

impl<T: MatrixNumber> Mul<GlPoint<T>> for GlMatrix<T> {
    type Output = GlPoint<T>;
    fn mul(self, rhs: GlPoint<T>) -> GlPoint<T> {
        self.dot(&rhs)
    }
}

impl<T: MatrixNumber> Mul<GlVector<T>> for GlMatrix<T> {
    type Output = GlVector<T>;
    fn mul(self, rhs: GlVector<T>) -> GlVector<T> {
        self.dot(&rhs)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_matrix_new() {
        // can create new test
        let matrix: GlMatrix<f64> =
            GlMatrix::new(vec![vec![1.0, 2.0, 3.0], vec![6.0, 2.0, 7.0], vec![21.0, 45.0, 3.0]]);
        assert_eq!(matrix.get_dimensions(), (3, 3));
        assert!(matrix.is_square());

        assert_eq!(*matrix.get(2, 2), 3.0);
    }
    #[test]
    fn test_dot_product() {
        let matrix_a = GlMatrix::new(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0]]
        );

        let matrix_b = GlMatrix::new(vec![
            vec![7.0, 8.0], 
            vec![9.0, 10.0], 
            vec![11.0, 12.0]]);

        let matrix_c = matrix_a.dot(&matrix_b);
        assert_eq!(matrix_c, GlMatrix::new(vec![vec![58.0, 64.0], vec![139.0, 154.0]]))
    }
    #[test]
    fn test_identity() {
        let matrix = GlMatrix::new(vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]]);
        assert_eq!(GlMatrix::identity(3), matrix);

        let matrix_b = GlMatrix::new(vec![vec![1.0, 0.0, 0.0], vec![0.0, 5.0, 0.0], vec![4.0, 0.0, 2.0]]);

        assert_eq!(matrix_b.dot(&GlMatrix::identity(3)), matrix_b);
    }
    #[test]
    fn test_transpose() {
        let matrix = GlMatrix::new(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let transpose_matrix = GlMatrix::new(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(matrix.transpose(), transpose_matrix);

        let transposed_identity: GlMatrix<f64> = GlMatrix::identity(5).transpose();
        assert_eq!(transposed_identity, GlMatrix::identity(5));
    }
    #[test]
    fn test_submatrix() {
        let matrix_1 = GlMatrix::new(vec![vec![1.0, 5.0, 0.0], vec![-3.0, 2.0, 7.0], vec![0.0, 6.0, -3.0]]);

        let submatrix_1 = GlMatrix::new(vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);

        assert_eq!(matrix_1.submatrix(0, 2), submatrix_1);

        let matrix_2 = GlMatrix::new(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);

        let submatrix_2 = GlMatrix::new(vec![vec![-6.0, 1.0, 6.0], vec![-8.0, 8.0, 6.0], vec![-7.0, -1.0, 1.0]]);
        assert_eq!(matrix_2.submatrix(2, 1), submatrix_2);

        let submatrix_3 = GlMatrix::new(vec![vec![3.0, 5.0, 0.0], vec![2.0, -1.0, -7.0], vec![6.0, -1.0, 5.0]]);
        assert_eq!(submatrix_3.minor(1, 0), 25.0)
    }

    #[test]
    fn test_cofactor() {
        let matrix = GlMatrix::new(vec![vec![3.0, 5.0, 0.0], vec![2.0, -1.0, -7.0], vec![6.0, -1.0, 5.0]]);

        assert_eq!(matrix.cofactor(0, 0), -12.0);
        assert_eq!(matrix.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_det() {
        let matrix_a = GlMatrix::new(vec![vec![1.0, 2.0, 6.0], vec![-5.0, 8.0, -4.0], vec![2.0, 6.0, 4.0]]);

        assert_eq!(matrix_a.cofactor(0, 0), 56.0);
        assert_eq!(matrix_a.cofactor(0, 1), 12.0);
        assert_eq!(matrix_a.cofactor(0, 2), -46.0);

        assert_eq!(matrix_a.det(), -196.0);

        let matrix_b = GlMatrix::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);


        let (x, y, z) = (6.0, 3.0, 6.0);
        let matrix_c = GlMatrix::new(vec![
            vec![1.0, 0.0, 0.0, x],
            vec![0.0, 1.0, 0.0, y],
            vec![0.0, 0.0, 1.0, x],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);

        //let magnitude = Float::sqrt(x.powi(2) + y.powi(2) + z.powi(2));

        assert_eq!(matrix_b.cofactor(0, 0), 690.0);
        assert_eq!(matrix_b.cofactor(0, 1), 447.0);
        assert_eq!(matrix_b.cofactor(0, 2), 210.0);
        assert_eq!(matrix_b.cofactor(0, 3), 51.0);
        assert_eq!(matrix_c.det(), 0.0);
    }
    #[test]
    fn test_invert() {
        let matrix_a = GlMatrix::new(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0]
        ]);

        let inverted_example = GlMatrix::new(vec![
            vec![-0.15384615384615385, -0.15384615384615385, -0.28205128205128205, -0.5384615384615384],
            vec![-0.07692307692307693, 0.12307692307692308, 0.02564102564102564, 0.03076923076923077],
            vec![0.358974358974359, 0.358974358974359, 0.4358974358974359, 0.9230769230769231],
            vec![-0.6923076923076923, -0.6923076923076923, -0.7692307692307693, -1.9230769230769231]
        ]);

        //println!("the det is {:?}", matrix_a.det());
        let inverted = matrix_a.invert().unwrap();
        //println!("inverted: {:?}", inverted);
        assert_eq!(matrix_a.invert().unwrap(), inverted_example);

    }
    #[test]
    pub fn test_plus() {
        let matrix_a = GlMatrix::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
        ]);

        let matrix_b = GlMatrix::new(vec![
            vec![1.0, 1.0, -3.0, 1.0],
            vec![1.0, 1.0, 7.0, 1.0],
        ]);

        let matrix_sum = matrix_a.add(matrix_b).unwrap();

        //println!("{:?}", matrix_sum);

        let matrix_sumation = GlMatrix::new(vec![
            vec![-1.0, -7.0, 0.0, 6.0],
            vec![-2.0, 2.0, 14.0, 4.0],
        ]);


        assert_eq!(matrix_sum, matrix_sumation);
    }
    #[test]
    fn test_dot_point_matrix_product() {
        // create an identity matrix 5, -3, 2

        let translation_matrix = GlMatrix::translation(5.0, -3.0, 2.0);
        let point_matrix = GlMatrix::new(vec![
            vec![-3.0],
            vec![4.0],
            vec![5.0],
            vec![One::one()]
        ]);
        let new_point_matrix = translation_matrix.dot(&point_matrix);
        //println!("{:?}", new_point);
        assert_eq!(new_point_matrix, GlMatrix::new(vec![
            vec![2.0], 
            vec![1.0],
            vec![7.0],
            vec![1.0]
        ]));
    }
    #[test]
    fn test_dot_inverse_point_matrix_product() {
        // create an identity matrix 5, -3, 2

        let translation_matrix = GlMatrix::translation(5.0, -3.0, 2.0);
        let point_matrix = GlMatrix::new(vec![
            vec![-3.0],
            vec![4.0],
            vec![5.0],
            vec![One::one()]
        ]);
        let new_point_matrix = translation_matrix.invert().unwrap().dot(&point_matrix);
        //println!("{:?}", new_point);
        assert_eq!(new_point_matrix, GlMatrix::new(vec![
            vec![-8.0], 
            vec![7.0],
            vec![3.0],
            vec![1.0]
        ]));
    }
    #[test]
    fn test_dot_point_product() {
        let translation_matrix = GlMatrix::translation(5.0, -3.0, 2.0);
        let point = GlPoint::new(-3.0, 4.0, 5.0);
        let new_point = translation_matrix.dot(&point);
        println!("glPoint: {:?}", new_point);
    }

    #[test]
    fn test_scaling_point() {
        let point = GlPoint::new(-4.0, 6.0, 8.0);
        let scaling_transfrom = GlMatrix::scaling(2.0, 3.0, 4.0);
        assert_eq!(scaling_transfrom * point, GlPoint::new(-8.0, 18.0, 32.0));
    }
    #[test]
    fn test_scaling_vector() {
        let vector = GlVector::new(-4.0, 6.0, 8.0);
        let scaling_transfrom = GlMatrix::scaling(2.0, 3.0, 4.0);
        assert_eq!(scaling_transfrom * vector, GlVector::new(-8.0, 18.0, 32.0));
    }
    #[test]
    fn test_scaling_by_inverse() {
        let vector = GlVector::new(-4.0, 6.0, 8.0);
        let scaling_transfrom = GlMatrix::scaling(2.0, 3.0, 4.0).invert().unwrap();
        assert_eq!(scaling_transfrom * vector, GlVector::new(-2.0, 2.0, 2.0));
    }
    #[test]
    fn test_scaling_reflection() {
        let point = GlPoint::new(2.0, 3.0, 4.0);
        let scaling_transform = GlMatrix::scaling(-1.0, 1.0, 1.0);
        assert_eq!(scaling_transform * point, GlPoint::new(-2.0, 3.0, 4.0));
    }
    #[test]
    fn test_rotate_x() {
        let point = GlPoint::new(0.0, 1.0, 0.0);
        let half_quarter = GlMatrix::rotate_x(std::f64::consts::PI/4.0);
        let full_quarter = GlMatrix::rotate_x(std::f64::consts::PI/2.0);

        let expected = GlPoint::new(0.0, 0.7071067811865476, 0.7071067811865475);
        let calculated = half_quarter.dot(&point);
        assert_eq!(calculated, expected);
        assert_eq!(full_quarter * point, GlPoint::new(0.0, 0.00000000000000006123233995736766, 1.0));
    }
    #[test]
    fn test_rotate_y() {
        let point = GlPoint::new(0.0, 0.0, 1.0);
        let half_quarter = GlMatrix::rotate_y(std::f64::consts::PI/4.0);
        let full_quarter = GlMatrix::rotate_y(std::f64::consts::PI/2.0);

        assert_eq!(half_quarter.dot(&point), GlPoint::new(0.7071067811865475, 0.0, 0.7071067811865476));
        assert_eq!(full_quarter.dot(&point), GlPoint::new(1.0, 0.0, 0.00000000000000006123233995736766));
    }
}

use num::{Num, FromPrimitive};
use std::ops::Add;
use std::fmt::{Debug, Display};
use basic::{pow, Convert};
use matrices::solvable::Solvable;
use matrices::vector_help::{add_gaussian, get_scalar_product_of_vectors, reduce_row};
use error::*;

/// A struct representing matrices
#[derive(Clone, Debug, PartialEq)]
pub struct Matrice<T>
    where T: Num + Clone + Add<T> + Copy + Debug + Display + FromPrimitive + Convert
{
    /// Number of rows in the matrice
    pub rows: usize,
    /// Number of columns in the matrice
    pub columns: usize,
    /// Content of the matrice as a vector.
    pub data: Vec<T>,
}

impl<T> Matrice<T>
    where T: Num + Clone + Add<T> + Copy + Debug + Display + FromPrimitive + Convert
{
    /// Builds a matrice with a given number of rows and columns.
    /// Every element in the matrice is initialized with zero.
    /// # Remarks
    /// Returns an instance of the struct matrice
    /// # Examples
    /// ```
    /// let matrice = Matrice::build_empty_matrice(3, 3);
    /// assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], matrice.data);
    /// ```
    pub fn build_empty_matrice(rows: usize, columns: usize) -> Matrice<T> {
        Matrice {
            rows,
            columns,
            data: vec![T::zero(); columns * rows],
        }
    }

    /// Builds a matrice with a given number of rows, columns and the data.
    /// # Remarks
    /// Returns a matrice if the length of the vector equals the number of fields in the matrice.
    ///
    /// Returns an error message otherwise.
    /// # Examples
    /// ```
    /// let matrice = Matrice::build_matrice(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    /// ```
    pub fn build_matrice(rows: usize, columns: usize, data: Vec<T>) -> Result<Matrice<T>, MatholError> {
        if data.len() != rows * columns {
            return Err(MatholError::MatriceCause(MatriceError {
                message: "Vector is not the same length as the product of rows and columns".to_string(),
            }));
        }

        Ok(Matrice {
            rows,
            columns,
            data,
        })
    }

    /// Returns the element at a given position
    /// # Remarks
    /// Returns the element if the position is valid
    ///
    /// Returns an error message if the position is invalid (e.g. outside of bounds)
    /// # Examples
    /// ```
    /// let matrice = Matrice::build_matrice(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    /// assert_eq!(Ok(6), matrice.get_element(1, 2));
    /// assert_eq!(Err("Row is out of bounds"), matrice.get_element(3, 2));
    /// assert_eq!(Err("Column is out of bounds"), matrice.get_element(2, 3));
    /// ```
    pub fn get_element(&self, row: usize, column: usize) -> Result<T, MatholError> {
        if row >= self.rows {
            return Err(MatholError::OutOfBoundsCause(OutOfBoundsError {
                message: "Row is out of bounds".to_string(),
            }));
        }
        if column >= self.columns {
            return Err(MatholError::OutOfBoundsCause(OutOfBoundsError {
                message: "Column is out of bounds".to_string(),
            }));
        }

        Ok(self.data[self.columns * row + column].clone())
    }

    /// Inserts an element at a given position
    /// # Examples
    /// ```
    /// let m = Matrice::build_empty_matrice(2, 2);
    /// m.insert_element(0, 0);
    /// m.insert_element(0, 1);
    /// m.insert_element(1, 0);
    /// m.insert_element(1, 1);
    /// assert_eq!(vec![1, 2, 3, 4], m.data);
    /// ```
    pub fn insert_element(&mut self, element: T, row: usize, column: usize) {
        self.data[row * self.columns + column] = element;
    }

    /// Calculates the trace of a matrice
    /// # Remarks
    /// Returns the trace if the matrice is quadratic, otherwise an error message.
    /// # Examples
    /// ```
    /// let matrice = Matrice::build_matrice(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    /// assert_eq!(Ok(15), matrice.get_trace());
    /// ```
    pub fn get_trace(&self) -> Result<T, MatholError> {
        if self.rows != self.columns {
            return Err(MatholError::MatriceCause(MatriceError {
                message: "The matrice is not quadratic".to_string(),
            }));
        }

        let trace = (0..self.rows).fold(T::zero(), |sum, i| sum + self.get_element(i, i).unwrap());
        Ok(trace)
    }

    /// Adds a matrice to another matrice of the same type
    /// # Remarks
    /// Returns the result of the addition as a new matrice
    ///
    /// If the matrices are not of the same type (not the same number of rows and columns), an error
    /// message is returned.
    /// # Examples
    /// ```
    /// let m1 = Matrice::build_matrice(2, 3, vec![1, 5, -3, 4, 0, 8]).unwrap();
    /// let m2 = Matrice::build_matrice(2, 3, vec![5, 1, 3, -1, 4, 7]).unwrap();
    /// let m3 = m1.add_matrice(&m2).unwrap();
    /// assert_eq!(vec![6, 6, 0, 3, 4, 15], m3.data);
    /// ```
    pub fn add_matrice(&self, matrice: &Matrice<T>) -> Result<Matrice<T>, MatholError> {
        if self.rows != matrice.rows || self.columns != matrice.columns {
            return Err(MatholError::MatriceCause(MatriceError {
                message: "The two matrices do not have the same number of rows or columns".to_string(),
            }));
        }

        let result = Matrice {
            rows: self.rows,
            columns: self.columns,
            data: (0..self.rows*self.columns).fold(Vec::new(), |mut vec, i| {
                vec.push(self.data[i].clone() + matrice.data[i].clone());
                vec
            }),
        };

        Ok(result)
    }

    /// Subtracts a matrice from another matrice of the same type
    /// # Remarks
    /// Returns the result of the subtraction as a new matrice
    ///
    /// If the matrices are not of the same type (not the same number of rows and columns), an error
    /// message is returned.
    /// # Examples
    /// ```
    /// let m1 = Matrice::build_matrice(2, 3, vec![1, 5, -3, 4, 0, 8]).unwrap();
    /// let m2 = Matrice::build_matrice(2, 3, vec![5, 1, 3, -1, 4, 7]).unwrap();
    /// let m3 = m1.subtract_matrice(&m2).unwrap();
    /// assert_eq!(vec![-4, 4, -6, 5, -4, 1], m3.data);
    /// ```
    pub fn subtract_matrice(&self, matrice: &Matrice<T>) -> Result<Matrice<T>, MatholError> {
        if self.rows != matrice.rows || self.columns != matrice.columns {
            return Err(MatholError::MatriceCause(MatriceError {
                message: "The two matrices do not have the same number of rows or columns".to_string(),
            }));
        }

        let result = Matrice {
            rows: self.rows,
            columns: self.columns,
            data: (0..self.rows*self.columns).fold(Vec::new(), |mut vec, i| {
                vec.push(self.data[i].clone() - matrice.data[i].clone());
                vec
            }),
        };

        Ok(result)
    }

    /// Multiplies a matrice with a scalar
    /// # Remarks
    /// Returns the result of the multiplication as a new Matrice
    /// # Examples
    /// ```
    /// let matrice = Matrice::build_matrice(2, 3, vec![1, -5, 3, 4, 1, 0]).unwrap();
    /// assert_eq!(vec![4, -20, 12, 16, 4, 0], matrice.multiply_with_scalar(4).data);
    /// assert_eq!(vec![-3, 15, -9, -12, -3, 0], matrice.multiply_with_scalar(-3).data);
    /// ```
    pub fn multiply_with_scalar(&self, scalar: T) -> Matrice<T> {
        Matrice {
            rows: self.rows,
            columns: self.columns,
            data: (0..self.rows*self.columns).fold(Vec::new(), |mut vec, i| {
                vec.push(self.data[i].clone() * scalar);
                vec
            })
        }
    }

    /// Returns a row from the matrice as a vector
    /// # Remarks
    /// Returns an error message if the given parameter row points to an out-of-bounds row index
    /// # Examples
    /// ```
    /// let a = Matrice::build_matrice(3, 3, vec![1, 4, -2, 0, 1, 1, -3, 2, 5]).unwrap();
    /// assert_eq!(Ok(vec![1, 4, -2]), a.get_row(0));
    /// assert_eq!(Ok(vec![0, 1, 1]), a.get_row(1));
    /// assert_eq!(Ok(vec![-3, 2, 5]), a.get_row(2));
    /// assert_eq!(Err("Row is out of bounds"), a.get_row(3));
    /// ```
    pub fn get_row(&self, row: usize) -> Result<Vec<T>, MatholError> {
        if row >= self.rows {
            return Err(MatholError::OutOfBoundsCause(OutOfBoundsError {
                message: "Row is out of bounds".to_string(),
            }));
        }

        let start = self.columns * row;
        let end = start + self.columns;
        Ok(self.data[start..end].to_vec())
    }

    /// Returns a column from the matrice as a vector
    /// # Remarks
    /// Returns an error message if the given parameter column points to an out-of-bounds column index
    /// # Examples
    /// ```
    /// let b = Matrice::build_matrice(3, 3, vec![3, 0, 1, -2, 1, 5, 2, 3, 8]).unwrap();
    /// assert_eq!(Ok(vec![3, -2, 2]), b.get_column(0));
    /// assert_eq!(Ok(vec![0, 1, 3]), b.get_column(1));
    /// assert_eq!(Ok(vec![1, 5, 8]), b.get_column(2));
    /// assert_eq!(Err("Column is out of bounds"), b.get_column(3));
    /// ```
    pub fn get_column(&self, column: usize) -> Result<Vec<T>, MatholError> {
        if column >= self.columns {
            return Err(MatholError::OutOfBoundsCause(OutOfBoundsError {
                message: "Column is out of bounds".to_string(),
            }));
        }

        let vec = (0..self.rows).fold(Vec::new(), |mut vec, i| {
            vec.push(self.get_element(i, column).unwrap());
            vec
        });

        Ok(vec)
    }

    /// Inserts a new row at the tail end of the matrice
    /// # Remarks
    /// Panics if the new row does not have the same amount of columns as the matrice
    /// # Examples
    /// ```
    /// let mut m = Matrice::build_matrice(2, 2, vec![1, 2, 3, 4]).unwrap();
    /// m.insert_row(&vec![5, 6]);
    /// assert_eq!(3, m.rows);
    /// assert_eq!(vec![1, 2, 3, 4, 5, 6], m.data);
    /// ```
    pub fn insert_row(&mut self, row: &Vec<T>) -> Result<(), MatholError> {
        if row.len() != self.columns {
            return Err(MatholError::LengthCause(LengthError {
                message: format!("Row must have {} columns", self.columns),
            }));
        }

        self.data = [self.data.clone(), row[..].to_vec()].concat();
        self.rows += 1;
        Ok(())
    }

    /// Inserts a new column at the right-sided end of the matrice
    /// # Remarks
    /// Panics if the new column does not have the same amount of rows as the matrice
    /// # Examples
    /// ```
    /// let mut m = Matrice::build_matrice(2, 2, vec![1, 2, 3, 4]).unwrap();
    /// m.insert_column(&vec![5, 6]);
    /// assert_eq!(3, m.columns);
    /// assert_eq!(vec![1, 2, 5, 3, 4, 6], m.data);
    /// ```
    pub fn insert_column(&mut self, column: &Vec<T>) -> Result<(), MatholError> {
        if column.len() != self.rows {
            return Err(MatholError::LengthCause(LengthError {
                message: format!("Column must have {} rows", self.rows),
            }));
        }

        column.iter().fold(0, |j, k| {
            self.data.insert((j + 1) * self.columns + j, *k);
            j + 1
        });

        self.columns += 1;
        Ok(())
    }

    /// Calculates the scalar product of two vectors
    /// # Remarks
    /// Returns the scalar product
    pub fn get_scalar_product_of_vectors(vec1: &Vec<T>, vec2: &Vec<T>) -> T {
        (0..vec1.len()).fold(T::zero(), |sum, x| sum + vec1[x] * vec2[x])
    }

    /// Multiplies a matrice with another matrice
    /// # Remarks
    /// Returns the result of the multiplication as a new Matrice
    ///
    /// Returns an error message if matrix self does not have the same number of columns as matrix other has number of rows
    /// # Examples
    /// ```
    /// let a = Matrice::build_matrice(3, 3, vec![1, 4, -2, 0, 1, 1, -3, 2, 5]).unwrap();
    /// let b = Matrice::build_matrice(3, 3, vec![3, 0, 1, -2, 1, 5, 2, 3, 8]).unwrap();
    /// assert_eq!(vec![-9, -2, 5, 0, 4, 13, -3, 17, 47], a.multiply_with_matrice(&b).unwrap().data);
    /// ```
    pub fn multiply_with_matrice(&self, other: &Matrice<T>) -> Result<Matrice<T>, MatholError> {
        if self.columns != other.rows {
            return Err(MatholError::MatriceCause(MatriceError {
                message: "Matrix A must have the same number of columns as Matrix B has number of rows".to_string(),
            }));
        }

        let mut data = Vec::new();

        for i in 0..self.rows {
            for k in 0..other.columns {
                data.push(get_scalar_product_of_vectors(&self.get_row(i)?, &other.get_column(k)?));
            }
        }

        let matrice = Matrice {
            rows: self.rows,
            columns: other.columns,
            data,
        };

        Ok(matrice)
    }

    /// Calculates the main diagonal product of a matrice starting at a given column
    /// # Remarks
    /// Returns the main diagonal product as a numeric value
    ///
    /// Returns an error message if the column index is out of bounds
    /// # Examples
    /// ```
    /// let m = Matrice::build_matrice(3, 3, vec![1, -2, 3, 2, 0, 1, 6, 5, 1]).unwrap();
    /// assert_eq!(Ok(0), m.get_main_diagonal_product(0));
    /// assert_eq!(Ok(-12), m.get_main_diagonal_product(1));
    /// assert_eq!(Ok(30), m.get_main_diagonal_product(2));
    /// ```
    pub fn get_main_diagonal_product(&self, mut column: usize) -> Result<T, MatholError> {
        if column >= self.columns {
            return Err(MatholError::OutOfBoundsCause(OutOfBoundsError {
                message: "Column is out of bounds".to_string(),
            }));
        }

        let prod = (0..self.rows).fold(T::one(), |prod, i| {
            if column >= self.columns { column = 0 };
            let res = prod * self.get_element(i, column).unwrap();
            column += 1;
            res
        });

        Ok(prod)
    }

    /// Calculates the side diagonal product of a matrice starting at a given column
    /// # Remarks
    /// Returns the side diagonal product as a numeric value
    ///
    /// Returns an error message if the column index is out of bounds
    /// # Examples
    /// ```
    /// let m = Matrice::build_matrice(3, 3, vec![1, -2, 3, 2, 0, 1, 6, 5, 1]).unwrap();
    /// assert_eq!(Ok(0), m.get_side_diagonal_product(0));
    /// assert_eq!(Ok(5), m.get_side_diagonal_product(1));
    /// assert_eq!(Ok(-4), m.get_side_diagonal_product(2));
    /// ```
    pub fn get_side_diagonal_product(&self, mut column: usize) -> Result<T, MatholError> {
        if column >= self.columns {
            return Err(MatholError::OutOfBoundsCause(OutOfBoundsError {
                message: "Column is out of bounds".to_string(),
            }));
        }

        let prod = (0..self.rows).rev().fold(T::one(), |prod, i| {
            if column >= self.columns { column = 0 };
            let res = prod * self.get_element(i, column).unwrap();
            column += 1;
            res
        });

        Ok(prod)
    }

    /// Calculates the determinant of a quadratic matrice
    /// # Remarks
    /// Returns the determinant as a numeric value
    ///
    /// Returns an error message if the matrice is not quadratic
    /// # Examples
    /// ```
    /// let m = Matrice::build_matrice(2, 2, vec![4, 7, -3, 8]).unwrap();
    /// assert_eq!(Ok(53), m.get_determinant());
    /// ```
    pub fn get_determinant(&self) -> Result<T, MatholError> {
        if self.rows != self.columns {
            return Err(MatholError::MatriceCause(MatriceError {
                message: "The matrice is not quadratic".to_string(),
            }));
        }
        if self.rows == 2 {
            return Ok(self.get_main_diagonal_product(0)? - self.get_side_diagonal_product(0)?);
        }
        if self.rows == 3 {
            let (m, s) = (0..self.columns).fold((T::zero(), T::zero()), |(m, s), i| {
                (m + self.get_main_diagonal_product(i).unwrap(), s + self.get_side_diagonal_product(i).unwrap())
            });

            return Ok(m - s);
        }

        let result = (0..self.columns).fold(T::zero(), |sum, i| {
            sum + self.get_element(i, self.columns - 1).unwrap() *
                FromPrimitive::from_i32(pow(-1, i + self.columns - 1)).unwrap() *
                self.get_submatrice(i, self.columns - 1).get_determinant().unwrap()
        });

        Ok(result)
    }

    /// Builds a submatrice of the matrice
    /// # Remarks
    /// Returns the submatrice
    /// #
    /// The matrice is of a 4x4 type and is
    ///
    /// |  1 |  2 |  0 | -1 |
    ///
    /// |  4 |  0 |  3 |  2 |
    ///
    /// |  9 |  0 |  0 |  4 |
    ///
    /// |  8 |  1 |  3 |  1 |
    ///
    /// In that case, the submatrice for the position at row 2 and column 0 is
    ///
    /// |  2 |  0 | -1 |
    ///
    /// |  0 | -3 |  2 |
    ///
    /// |  1 |  3 |  1 |
    ///
    /// ```
    /// let m = Matrice::build_matrice(4, 4, vec![1, 2, 0, -1, 4, 0, -3, 2, 9, 0, 0, 4, 8, 1, 3, 1]).unwrap();
    /// assert_eq!(vec![2, 0, -1, 0, -3, 2, 1, 3, 1], m.get_submatrice(2, 0).data);
    /// ```
    pub fn get_submatrice(&self, row: usize, column: usize) -> Matrice<T> {
        let mut data = vec![];

        for i in 0..self.rows {
            if i == row {
                continue;
            }
            for k in 0..self.columns {
                if k == column {
                    continue;
                }
                data.push(self.get_element(i, k).unwrap());
            }
        }

        Matrice {
            rows: self.rows - 1,
            columns: self.columns - 1,
            data,
        }
    }

    /// Calculates the inverse matrice of a matrice
    /// # Remarks
    /// Returns the inverse matrice
    ///
    /// Returns an error message if the matrice is not quadratic
    /// # Examples
    /// ```
    /// let m = Matrice::build_matrice(3, 3, vec![1, 0, -1, -8, 4, 1, -2, 1, 0]).unwrap();
    /// assert_eq!(vec![1, 1, -4, 2, 2, -7, 0, 1, -4], m.get_inverse_matrice().unwrap().data);
    /// ```
    pub fn get_inverse_matrice(&self) -> Result<Matrice<T>, MatholError> {
        if self.rows != self.columns {
            return Err(MatholError::MatriceCause(MatriceError {
                message: "The matrice is not quadratic".to_string(),
            }));
        }
        let mut matrice = Matrice::build_empty_matrice(self.rows, self.columns);

        for i in 0..self.rows {
            for k in 0..self.columns {
                let det = self.get_submatrice(i, k).get_determinant()? * FromPrimitive::from_i32(pow(-1, i + k)).unwrap();
                matrice.insert_element(det / self.get_determinant()?, k, i);
            }
        }

        Ok(matrice)
    }

    /// Calculates the rank of a matrice
    /// # Remarks
    /// Returns the rank
    ///
    /// If the rank can't be calculated, the method returns an error message
    /// # Examples
    /// ```
    /// let m = Matrice::build_matrice(2, 3, vec![2, 3, 1, 0, 4, 2]).unwrap();
    /// assert_eq!(Ok(2), m.get_rank());
    /// ```
    pub fn get_rank(&self) -> Result<usize, MatholError> {
        let r = if self.rows <= self.columns {
            self.rows
        } else {
            self.columns
        };

        for p in (1..r+1).rev() {
            let submatrices = self.cut_matrices(p);
            for submatrice in submatrices.iter() {
                if submatrice.get_determinant()? != T::zero() {
                    return Ok(p);
                }
            }
        }

        Err(MatholError::MatriceCause(MatriceError {
            message: "Could not calculate rank of matrice".to_string(),
        }))
    }

    /// A helper function for get_rank()
    fn cut_matrices(&self, p: usize) -> Vec<Matrice<T>> {
        let mut vec = Vec::<Matrice<T>>::new();

        let end_row = self.rows - p + 1;

        let end_column = self.columns - p + 1;

        for i in 0..end_row {
            for k in 0..end_column {
                vec.push(Matrice::build_matrice(p, p, self.get_slice(i, k, p)).unwrap());
            }
        }

        vec
    }

    /// A helper function for get_rank()
    fn get_slice(&self, i: usize, k: usize, p: usize) -> Vec<T> {
        let mut vec = Vec::new();

        for q in 0..p {
            let start = (i + q) * self.columns + k;
            let end = start + p;
            vec = [vec, self.data.clone()[start..end].to_vec()].concat();
        }

        vec
    }

    /// Checks if a linear equation system is solvable
    /// # Remarks
    /// Returns a value of the enum Solvable
    /// # Examples
    /// ```
    /// let m = Matrice::build_matrice(2, 3, vec![1, -2, 1, 1, 1, -4]).unwrap();
    /// let c = vec![1, 8];
    /// assert_eq!(Solvable::InfiniteSolutions, m.is_solvable(&c));
    /// ```
    pub fn is_solvable(&self, c: &Vec<T>) -> Solvable {
        let mut ac = self.clone();
        ac.insert_column(&c).unwrap();

        if self.rows != self.columns {
            if self.get_rank().unwrap() == ac.get_rank().unwrap() {
                if self.get_rank().unwrap() == self.columns {
                    Solvable::OneSolution
                } else {
                    Solvable::InfiniteSolutions
                }
            } else {
                Solvable::NoSolution
            }
        } else {
            if self.get_determinant().unwrap() != T::zero() {
                Solvable::OneSolution
            } else {
                if self.get_rank() == ac.get_rank() {
                    Solvable::InfiniteSolutions
                } else {
                    Solvable::NoSolution
                }
            }
        }
    }

    pub fn shuffle(&self, c: &Vec<T>) -> (Vec<Vec<T>>, Vec<T>) {
        let rows = self.rows;
        let columns = self.columns;
        let mut unordered_vec = Vec::new();
        let mut shuffled_vec = Vec::new();
        let mut d = Vec::new();

        for i in 0..rows {
            unordered_vec.push(self.get_row(i).unwrap());
        }

        for k in 0..columns {
            for i in 0..unordered_vec.len() {
                if unordered_vec[i][k] != T::zero() {
                    if !shuffled_vec.contains(&unordered_vec[i]) {
                        shuffled_vec.push(unordered_vec[i].clone());
                        d.push(c[i]);
                    }
                }
            }
        }

        (shuffled_vec, d)
    }

    pub fn solve(&self, c: &Vec<T>) -> Result<(Vec<Vec<f64>>, Vec<f64>), MatholError> {
        if self.is_solvable(&c) != Solvable::OneSolution {
            return Err(MatholError::MatriceCause(MatriceError {
                message: "The linear system is not solvable or it has infinite solutions".to_string(),
            }));
        }

        let (a, d) = self.shuffle(c);

        let mut m = Vec::new();
        let mut s = Vec::new();

        for i in a.iter() {
            let mut b = Vec::new();
            for k in i.iter() {
                b.push(k.to_f64());
            }
            m.push(b);
        }

        for i in d.iter() {
            s.push(i.to_f64());
        }

        for k in 0..self.columns {
            if m[k][k] == 0.0 {
                let tmp = m[k].clone();
                let y = s[k];
                m.remove(k);
                s.remove(k);
                m.push(tmp);
                s.push(y);
            }
            let (r, w) = reduce_row(&m[k], k, &s);
            m[k] = r;
            s[k] = w;
            for i in k+1..self.rows {
                let (vec, u) = add_gaussian(&m[k], &m[i], k, s[k], s[i]);
                m[i] = vec;
                s[i] = u;
            }
        }

        for k in (0..self.columns).rev() {
            for i in (0..k).rev() {
                let (vec, u) = add_gaussian(&m[k], &m[i], k, s[k], s[i]);
                m[i] = vec;
                s[i] = u;
            }
        }

        Ok((m, s))
    }
}
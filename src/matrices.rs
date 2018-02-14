use num::Num;
//use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Matrice<T>
    where T: Num + Clone + PartialEq
{
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<T>,
}

impl<T> Matrice<T>
    where T: Num + Clone
{
    pub fn build_empty_matrice(rows: usize, columns: usize) -> Matrice<T> {
        Matrice {
            rows,
            columns,
            data: vec![T::zero(); columns * rows],
        }
    }

    pub fn build_matrice(rows: usize, columns: usize, data: Vec<T>) -> Result<Matrice<T>, &'static str> {
        if data.len() != rows * columns {
            return Err("Vector is not the same length as the product of rows and columns");
        }

        Ok(Matrice {
            rows,
            columns,
            data,
        })
    }

    pub fn get_element(&self, row: usize, column: usize) -> Result<T, &str> {
        if row >= self.rows {
            return Err("Row is out of bounds");
        }
        if column >= self.columns {
            return Err("Column is out of bounds");
        }

        Ok(self.data[self.rows * row + column].clone())
    }
}
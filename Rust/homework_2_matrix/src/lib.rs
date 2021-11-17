use std::ops::{Add, Deref, DerefMut};

#[derive(Debug)]
pub struct Matrix<T: Clone> {
    pub data : Vec<T>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl<T> Deref for Cell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Cell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Clone> Matrix<T> {
    /// Данните се очаква да бъдат подадени със статичен масив -- вижте по-долу за примери за
    /// конструиране. Какви може да са елементите? Ще тестваме само с два типа: String и i32.
    ///
    /// Очаква се да бъдат подадени по редове, от ляво надясно. Тоест, ако подадем като вход списък
    /// с елементи: 1, 2, 3, 4, се очаква конструираната матрица:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Забележете, че подаваме като вход някакъв slice -- reference тип. Не очакваме матрицата да
    /// държи reference, клонирайте си данните, за да имате ownership.
    ///
    pub fn new(_data: &[T; 4]) -> Matrix<T> {
        let mut new_matrix : Matrix<T> = Matrix { data: vec![] };
        for i in _data {
            new_matrix.data.push(i.clone());
        }
        new_matrix
    }

    /// Връща вектор, който съдържа в себе си всички 4 елемента на матрицата, наредени по редове,
    /// от ляво надясно и от горе надолу, обвити в `Cell`. Тоест, ако матрицата изглежда така:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Очакваме `.by_row` да върне елементите в ред: 1, 2, 3, 4
    ///
    pub fn by_row(&self) -> Vec<Cell<T>> {
        let mut result : Vec<Cell<T>> = vec![];
        for i in &self.data {
            result.push(Cell(i.clone()));
        }
        result
    }

    /// Връща вектор, който съдържа в себе си всички 4 елемента на матрицата, наредени по колони,
    /// от горе надолу и от ляво надясно, Обвити в `Cell`. Тоест, ако матрицата изглежда така:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Очакваме `.by_col` да върне елементите в ред: 1, 3, 2, 4
    ///
    pub fn by_col(&self) -> Vec<Cell<T>> {
        let mut result : Vec<Cell<T>> = vec![];
        result.push(Cell(self.data[0].clone()));
        result.push(Cell(self.data[2].clone()));
        result.push(Cell(self.data[1].clone()));
        result.push(Cell(self.data[3].clone()));
        result
    }
}

impl Add<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn add(self, rhs: Cell<String>) -> Self::Output {
        let mut internal_string  =  (*rhs).clone();
        let number = (*self).abs().clone();
        if *self >=0 {
            let number_as_string : String = String::from(number.to_string() + " ");
            internal_string.insert_str(0, &*number_as_string);
            return Cell(internal_string);
        } else {
            let mut number_as_string : String = String::from(number.to_string());
            let reverse_internal_string = internal_string.chars().rev().collect::<String>().add(" ");
            number_as_string.insert_str(0,&*reverse_internal_string);
            return Cell(number_as_string);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use crate::Matrix;
    use crate::Cell;

    #[test]
    fn test_deref_cell() {
        let mut cell1 : Cell<i32> = Cell(2);
        let cell2 : Cell<String> = Cell(String::from("Petko"));
        assert_eq!(*cell1,2);
        assert_eq!(*cell2,String::from("Petko"));
        *cell1 = 3;
        assert_eq!(*cell1,3);
    }

    #[test]
    fn creating_matrix_and_test_by_row_by_col_with_numbers() {
        let matrix: Matrix<i32> = Matrix::new(&[1,2,3,4]);
        assert_eq!(matrix.by_row(),vec![Cell(1),Cell(2),Cell(3),Cell(4)]);
        assert_eq!(matrix.by_col(),vec![Cell(1),Cell(3),Cell(2),Cell(4)]);
    }

    #[test]
    fn creating_matrix_and_test_by_row_by_col_with_strings() {
        let matrix: Matrix<String> = Matrix::new(&[String::from("Karl Marx"),String::from("Vladimir Lenin"),String::from("Leo Trotsky"),String::from("Joseph Stalin")]);
        assert_eq!(matrix.by_row(),vec![Cell(String::from("Karl Marx")),Cell(String::from("Vladimir Lenin")),Cell(String::from("Leo Trotsky")),Cell(String::from("Joseph Stalin"))]);
        assert_eq!(matrix.by_col(),vec![Cell(String::from("Karl Marx")),Cell(String::from("Leo Trotsky")),Cell(String::from("Vladimir Lenin")),Cell(String::from("Joseph Stalin"))]);
    }

    #[test]
    fn adding_positive_numbers_with_strings() {
        assert_eq!(Cell(22) + Cell(String::from("years ago")), Cell(String::from("22 years ago")));
        assert_eq!(Cell(0) + Cell(String::from("expectation")), Cell(String::from("0 expectation")));
        assert_eq!(Cell(33) + Cell(String::from("years old")), Cell(String::from("33 years old")));

        let cell1 = Cell(1);
        let cell_string = Cell(String::from("and more"));
        assert_eq!(cell1.clone() + cell_string.clone(), Cell(String::from("1 and more")));
        assert_eq!(cell1,Cell(1));
        assert_eq!(cell_string,Cell(String::from("and more")));
    }

    #[test]
    fn adding_negative_numbers_with_strings() {
        assert_eq!(Cell(-4) + Cell(String::from("xirtam")), Cell(String::from("matrix 4")));
        assert_eq!(Cell(-5) + Cell(String::from("sraW ratS")), Cell(String::from("Star Wars 5")));
    }
}

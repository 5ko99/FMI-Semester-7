use std::ops::{Add, Deref, DerefMut, Mul};

#[derive(Debug)]
pub struct Matrix<T: Clone> {
    pub data: Vec<T>,
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
        let mut new_matrix: Matrix<T> = Matrix { data: vec![] };
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
        let mut result: Vec<Cell<T>> = vec![];
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
        let mut result: Vec<Cell<T>> = vec![];
        result.push(Cell(self.data[0].clone()));
        result.push(Cell(self.data[2].clone()));
        result.push(Cell(self.data[1].clone()));
        result.push(Cell(self.data[3].clone()));
        result
    }
}

impl<T: Clone> Deref for Matrix<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Clone> DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Clone + std::cmp::PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Matrix<T>) -> bool {
        let matching = (*self).iter().zip((*other).iter()).filter(|&(a, b)| a == b).count();
        matching == (*self).len() && matching == (*other).len()
    }
}

impl Add<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn add(self, rhs: Cell<String>) -> Self::Output {
        let mut internal_string = (*rhs).clone();
        let number = (*self).abs().clone();
        if *self >= 0 {
            let number_as_string: String = String::from(number.to_string() + " ");
            internal_string.insert_str(0, &*number_as_string);
            return Cell(internal_string);
        } else {
            let mut number_as_string: String = String::from(number.to_string());
            let reverse_internal_string = internal_string.chars().rev().collect::<String>().add(" ");
            number_as_string.insert_str(0, &*reverse_internal_string);
            return Cell(number_as_string);
        }
    }
}

impl Mul<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn mul(self, rhs: Cell<String>) -> Self::Output {
        let mut internal_string = (*rhs).clone();
        let number = (*self).abs().clone();
        let mut string = "".to_string();
        if *self < 0 {
            internal_string = internal_string.chars().rev().collect::<String>();
        }
        for _i in 0..number {
            string.push_str(&*internal_string);
        }
        Cell(string)
    }
}

fn apply_operation_on_matrices(lhs: Matrix<i32>, rhs: Matrix<String>, op: &dyn Fn(Cell<i32>, Cell<String>) -> Cell<String>) -> Matrix<String> {
    let mut result: Matrix<String> = Matrix { data: vec![String::from(""), String::from(""), String::from(""), String::from("")] };
    for i in 0..=3 {
        let left = Cell((*lhs)[i]);
        let right = Cell((*rhs)[i].clone());
        let result_from_add = op(left, right);
        (*result)[i] = (*result_from_add).clone();
    }
    result
}

fn vector_to_string(input: &Vec<Cell<String>>) -> String {
    let mut result: String = "".to_string();
    for i in 0..=3 {
        result.push_str(&**(input[i]).clone());
        if i <3 {
            result.push_str(" ");
        }
    }
    result
}

impl Add<Matrix<String>> for Matrix<i32> {
    type Output = Matrix<String>;
    fn add(self, rhs: Matrix<String>) -> Self::Output {
        apply_operation_on_matrices(self, rhs, &Cell::add)
    }
}

impl Mul<Matrix<String>> for Matrix<i32> {
    type Output = String;

    fn mul(self, rhs: Matrix<String>) -> Self::Output {
        let mut res : Vec<Cell<String>> = vec![Cell("".to_string()),Cell("".to_string()),Cell("".to_string()),Cell("".to_string())];
        res[0] = Cell((*self)[0]) * Cell((*rhs)[0].clone());
        res[1] = Cell((*self)[1]) *Cell((*rhs)[2].clone());
        res[2] = Cell((*self)[2]) * Cell((*rhs)[1].clone());
        res[3] = Cell((*self)[3]) * Cell((*rhs)[3].clone());
        vector_to_string(&res)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use crate::Matrix;
    use crate::Cell;

    // За помощни цели:
    fn string_cell_vec(s1: &str, s2: &str, s3: &str, s4: &str) -> Vec<Cell<String>> {
        [s1, s2, s3, s4].into_iter().map(String::from).map(Cell).collect::<Vec<Cell<String>>>()
    }

    #[test]
    fn test_basic() {
        assert_eq!((Cell(4) + Cell(String::from("badger"))).0, String::from("4 badger"));
        assert_eq!((Cell(2) * Cell(String::from("mushroom"))).0, String::from("mushroommushroom"));

        let matrix1 = Matrix::new(&[1, 2, 3, 4]);
        let matrix2 = Matrix::new(&[
            String::from("one"), String::from("two"),
            String::from("three"), String::from("four")
        ]);
        assert_eq!(matrix1.by_row()[0], Cell(1));
        assert_eq!(matrix1.by_col()[0], Cell(1));

        assert_eq!(
            (matrix1 + matrix2).by_row(),
            string_cell_vec("1 one", "2 two", "3 three", "4 four")
        );

        let matrix1 = Matrix::new(&[1, 1, 1, 1]);
        let matrix2 = Matrix::new(&[
            String::from("one"), String::from("two"),
            String::from("three"), String::from("four")
        ]);
        assert_eq!(matrix1 * matrix2, String::from("one three two four"));
    }

    #[test]
    fn test_deref_cell() {
        let mut cell1: Cell<i32> = Cell(2);
        let cell2: Cell<String> = Cell(String::from("Petko"));
        assert_eq!(*cell1, 2);
        assert_eq!(*cell2, String::from("Petko"));
        *cell1 = 3;
        assert_eq!(*cell1, 3);
    }

    #[test]
    fn creating_matrix_and_test_by_row_by_col_with_numbers() {
        let matrix: Matrix<i32> = Matrix::new(&[1, 2, 3, 4]);
        assert_eq!(matrix.by_row(), vec![Cell(1), Cell(2), Cell(3), Cell(4)]);
        assert_eq!(matrix.by_col(), vec![Cell(1), Cell(3), Cell(2), Cell(4)]);
    }

    #[test]
    fn creating_matrix_and_test_by_row_by_col_with_strings() {
        let matrix: Matrix<String> = Matrix::new(&[String::from("Karl Marx"), String::from("Vladimir Lenin"), String::from("Leo Trotsky"), String::from("Joseph Stalin")]);
        assert_eq!(matrix.by_row(), vec![Cell(String::from("Karl Marx")), Cell(String::from("Vladimir Lenin")), Cell(String::from("Leo Trotsky")), Cell(String::from("Joseph Stalin"))]);
        assert_eq!(matrix.by_col(), vec![Cell(String::from("Karl Marx")), Cell(String::from("Leo Trotsky")), Cell(String::from("Vladimir Lenin")), Cell(String::from("Joseph Stalin"))]);
    }

    #[test]
    fn adding_positive_numbers_with_strings() {
        assert_eq!(Cell(22) + Cell(String::from("years ago")), Cell(String::from("22 years ago")));
        assert_eq!(Cell(0) + Cell(String::from("expectation")), Cell(String::from("0 expectation")));
        assert_eq!(Cell(33) + Cell(String::from("years old")), Cell(String::from("33 years old")));
        assert_eq!(Cell(5) + Cell(String::from("ko")), Cell(String::from("5 ko")));

        let cell1 = Cell(1);
        let cell_string = Cell(String::from("and more"));
        assert_eq!(cell1.clone() + cell_string.clone(), Cell(String::from("1 and more")));
        assert_eq!(cell1, Cell(1));
        assert_eq!(cell_string, Cell(String::from("and more")));
    }

    #[test]
    fn adding_negative_numbers_with_strings() {
        assert_eq!(Cell(-4) + Cell(String::from("xirtam")), Cell(String::from("matrix 4")));
        assert_eq!(Cell(-5) + Cell(String::from("sraW ratS")), Cell(String::from("Star Wars 5")));
    }

    #[test]
    fn multiply_with_positive_number() {
        assert_eq!(Cell(3) * Cell(String::from("woah!")), Cell(String::from("woah!woah!woah!")));
        assert_eq!(Cell(0) * Cell(String::from("woah?")), Cell(String::from("")));
        assert_eq!(Cell(5) * Cell(String::from("yeah!")), Cell(String::from("yeah!yeah!yeah!yeah!yeah!")));
    }

    #[test]
    fn multiply_with_negative_number() {
        assert_eq!(Cell(-3) * Cell(String::from(",regdab")), Cell(String::from("badger,badger,badger,")));
        assert_eq!(Cell(-1) * Cell(String::from("okteP")), Cell(String::from("Petko")));
        assert_eq!(Cell(-4) * Cell(String::from("!hay")), Cell(String::from("yah!yah!yah!yah!")));
    }

    #[test]
    fn compare_matrices() {
        assert_eq!(Matrix::new(&[5, 4, 3, 2]), Matrix::new(&[5, 4, 3, 2]));
        assert_ne!(Matrix::new(&[5, 4, 3, 2]), Matrix::new(&[5, 4, 3, 1]));
        assert_eq!(Matrix::new(&[String::from("1 One"), String::from("2 Two"), String::from("3 Three"), String::from("4 Four")]),
                   Matrix::new(&[String::from("1 One"), String::from("2 Two"), String::from("3 Three"), String::from("4 Four")]));
        assert_ne!(Matrix::new(&[String::from("1 One"), String::from("2 Two"), String::from("3 Three"), String::from("4 Four")]),
                   Matrix::new(&[String::from("1 One"), String::from("3 Two"), String::from("3 Tres"), String::from("4 Four")]));
    }

    #[test]
    fn add_matrices() {
        let matrix1: Matrix<i32> = Matrix::new(&[1, 2, 3, 4]);
        let matrix2: Matrix<String> = Matrix::new(&[String::from("One"), String::from("Two"), String::from("Three"), String::from("Four")]);
        assert_eq!(matrix1 + matrix2, Matrix::new(&[String::from("1 One"), String::from("2 Two"), String::from("3 Three"), String::from("4 Four")]));

        let matrix1: Matrix<i32> = Matrix::new(&[-1, -2, -3, -4]);
        let matrix2: Matrix<String> = Matrix::new(&[String::from("enO"), String::from("owT"), String::from("eerhT"), String::from("ruoF")]);
        assert_eq!(matrix1 + matrix2, Matrix::new(&[String::from("One 1"), String::from("Two 2"), String::from("Three 3"), String::from("Four 4")]));
    }

    #[test]
    fn multiply_matrices() {
        let matrix1: Matrix<i32> = Matrix::new(&[1, 3, 2, 4]);
        let matrix2: Matrix<String> = Matrix::new(&[String::from("One"), String::from("Two"), String::from("Three"), String::from("Four")]);
        assert_eq!(matrix1 * matrix2, String::from("One ThreeThreeThree TwoTwo FourFourFourFour"));

        let matrix1: Matrix<i32> = Matrix::new(&[-1, -3, -2, -4]);
        let matrix2: Matrix<String> = Matrix::new(&[String::from("enO"), String::from("owT"), String::from("eerhT"), String::from("ruoF")]);
        assert_eq!(matrix1 * matrix2, String::from("One ThreeThreeThree TwoTwo FourFourFourFour"));
    }
}

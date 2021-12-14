use std::ops::Deref;

#[derive(Debug)]
pub struct Matrix<T> {
    pub data : Vec<Vec<T>>,
    pub rows : usize,
    pub cols : usize
}

impl<T: Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: &[T]) -> Self {
        let mut arr = Matrix {
            data: vec![vec![data[0].clone();cols];rows],
            rows,
            cols
        };
        let mut cur = 0;
        for i in 0 .. rows {
            for j in 0 .. cols {
                arr.data[i][j] = (data[cur]).clone();
                cur += 1;
            }
        }
        arr
    }

    pub fn by_row(&self) -> RowIter<T> {
        RowIter {
            cur: Some(&self.data[0][0]),
            data: & self.data,
            rows : self.rows,
            cols : self.cols,
            i: 0,
            j: 0,
            first_call : true
        }
    }

    pub fn by_col(&self) -> ColIter<T> {
        ColIter {
            cur: Some(&self.data[0][0]),
            data: & self.data,
            rows : self.rows,
            cols : self.cols,
            i: 0,
            j: 0,
            first_call : true
        }
    }
}

pub struct RowIter<'a,T> {
    pub cur : Option<&'a T>,
    data : &'a Vec<Vec<T>>,
    rows : usize,
    cols : usize,
    i : usize,
    j : usize,
    first_call : bool
}

impl<'a,T: Clone> Iterator for RowIter<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // if self.first_call {
        //     self.first_call = false;
        //     return self.cur;
        // }
        let prev = self.cur;
        self.j += 1;
        if self.j >= self.cols {
            self.j = 0;
            self.i += 1;
        }
        if self.i >= self.rows {
            self.cur = None;
            return prev;
        }
        self.cur = Some(&self.data[self.i][self.j]);

        prev
    }
}

impl<'a,T : Clone> Deref for RowIter<'a,T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data[self.i][self.j]
    }
}

pub struct ColIter<'a,T> {
    pub cur : Option<&'a T>,
    data : &'a Vec<Vec<T>>,
    rows : usize,
    cols : usize,
    i : usize,
    j : usize,
    first_call : bool
}

impl<'a,T: Clone> Iterator for ColIter<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_call {
            self.first_call = false;
            return self.cur;
        }
        self.i += 1;
        if self.i >= self.rows {
            self.i = 0;
            self.j += 1;
        }
        if self.j >= self.cols {
            return None;
        }
        self.cur = Some(&self.data[self.i][self.j]);

        self.cur
    }
}

#[test]
fn test_iteration_0() {
    let data = [1, 2,
        3, 4];
    let matrix = Matrix::new(2, 2, &data);

    assert_eq!(matrix.by_row().collect::<Vec<_>>(), vec![&1, &2, &3, &4]);
    assert_eq!(matrix.by_col().collect::<Vec<_>>(), vec![&1, &3, &2, &4]);
}

#[test]
fn test_iteration_1() {
    let data = [1, 2,
        3, 4,5,6,7,8,9];
    let matrix = Matrix::new(3, 3, &data);

    assert_eq!(matrix.by_row().collect::<Vec<_>>(), vec![&1, &2, &3, &4,&5,&6,&7,&8,&9]);
    assert_eq!(matrix.by_col().collect::<Vec<_>>(), vec![&1, &4, &7, &2,&5,&8,&3,&6,&9]);
}

#[test]
fn test_iteration_2() {
    let data = [1, 2,
        3, 4,5,6,7,8,9];
    let matrix = Matrix::new(3, 3, &data);

    let mut a = matrix.by_row();
    assert_eq!(1,*(a.next()).unwrap());
    assert_eq!(*(matrix.by_row()),1);

    let arr = vec![1,2,3];
    let deref_val : i32 = *(arr.iter().next().unwrap());
    assert_eq!(deref_val,1);
}
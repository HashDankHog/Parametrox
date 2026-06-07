//having a dedicated matrix structure rather than just Vec<Vec<f64>> makes it easier to prevent errors when declaring matrices
//it also allows for easy implementation of matrix operations
// NOTE: the rows and colums attributes might be unnesescary, but at the same time I am not sure
//TODO
//  - add, in a different file, the ability to round to a certain number of digits

use crate::parameter::Parameter;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix {
    //naming this variable element makes any code that reads a specific element of the matrix more readable
    element: Vec<Vec<Parameter>>,
    rows: usize,
    colums: usize,
}
impl Matrix {
    pub fn iterate<F: FnMut(usize, usize) -> Parameter>(&self, mut value: F) -> Matrix {
        let mut result_matrix = create_matrix(self.rows, self.colums);

        for row in 0..result_matrix.rows {
            for colum in 0..result_matrix.colums {
                result_matrix.element[row][colum] = value(row, colum);
                result_matrix.element[row][colum].simplify_expression();
            }
        }

        result_matrix
    }

    pub fn print(&self) {
        for row in 0..self.rows {
            for colum in 0..self.colums {
                print!("{} ", self.element[row][colum].value);
            }
            println!();
        }
        println!();
    }

    pub fn set(&mut self, row: usize, colum: usize, value: &Parameter) {
        

        if (row > self.rows) || (colum > self.colums) {
            panic!("attempted to access an element beyond the bounds of the element");
        }

        self.element[row][colum] = value.clone();

    }

    pub fn multiply_scalar(&self, scalar: &Parameter) -> Matrix {
        self.iterate(|row, colum| self.element[row][colum].clone() * scalar.clone())
    }

    pub fn add_matrix(&self, matrix: &Matrix) -> Matrix {
        if (matrix.rows != self.rows) || (matrix.colums != self.colums) {
            panic!("cannot add matrices of different dimensions");
        }
        
        self.iterate(|row, colum| self.element[row][colum].clone() + matrix.element[row][colum].clone())
    }

    //strassens algorithm not implemented yet due to inefficency for n < 100
    pub fn multiply_matrix(&self, matrix: &Matrix) -> Matrix {
        if matrix.rows != self.colums {
            panic!("yo dimensions aint correct twin");
        }

        let result_matrix = create_matrix(self.rows, matrix.colums);

        result_matrix.iterate(|row, colum| {
            let mut element = Parameter::default();

            for matrix_row in 0..self.colums {
                element = element + self.element[row][matrix_row].clone() * matrix.element[matrix_row][colum].clone();
            }

            element
        })
    }

    pub fn transpose(&self) -> Matrix {
        let result_matrix = create_matrix(self.colums, self.rows);
        result_matrix.iterate(|row, colum| self.element[colum][row].clone())
    }

    //inefficent due to usage of clone trait
    pub fn submatrix(&self, row: usize, colum: usize) -> Matrix {
        let mut result_matrix = Matrix {
            //NOTE: using clone is a performance hit and needs to be fixed
            element: self.element.clone(),
            rows: self.rows - 1,
            colums: self.colums - 1,
        };

        result_matrix.element.remove(row);

        for row in 0..result_matrix.rows {
            result_matrix.element[row].remove(colum);
        }

        result_matrix
    }

    //inefficent for a large number of reasons
    pub fn determinant(&self) -> Parameter {
        if self.rows != self.colums {
            panic!("cannot take the determinant of a non square matrix");
        }

        let size = self.rows;
        if size == 2 {
            return self.element[0][0].clone() * self.element[1][1].clone()
                - self.element[0][1].clone() * self.element[1][0].clone();
        }

        let mut determinant = Parameter::default();

        for colum in 0..size {
            let submatrix = self.submatrix(0, colum);
            let cofactor_sign = f64::from(-1.0).powf(colum as f64);
            let cofactor_sign_parameter = Parameter {expression: vec![cofactor_sign.to_string()], value: cofactor_sign};
            let cofactor =  cofactor_sign_parameter * submatrix.determinant(); //might want to move cofactor into its own method

            determinant = determinant + self.element[0][colum].clone() * cofactor.clone();
        }

        determinant
    }

    pub fn adjoint(&self) -> Matrix {
        if self.rows != self.colums {
            panic!("matrix must be square");
        }

        self.iterate(|row, colum| {
            let sign = f64::from(-1.0).powf((colum + row) as f64);
            Parameter{expression: vec![sign.to_string()], value: sign} * self.submatrix(row, colum).determinant()
        })
        .transpose()
    }

    //TODO: rewrite this code to not panic when determinant is 0 and instead return a recoverable error
    pub fn inverse(&self) -> Matrix {
        if self.rows != self.colums {
            panic!("matrix must be square");
        }

        let determinant = self.determinant();

        let inverse_determinant = Parameter{expression: vec![String::from("1")], value: 1.0} / determinant;
        let mut result_matrix = self.multiply_scalar(&inverse_determinant);

        result_matrix = result_matrix.adjoint();

        result_matrix
    }
}

//NOTE: inconsistent typing with the set function
pub fn create_matrix(rows: usize, colums: usize) -> Matrix {
    Matrix {
        element: vec![vec![Parameter { expression: vec![String::from("0")], value: 0.0 }; colums]; rows],
        rows: rows,
        colums: colums,
    }
}

pub fn identity_matrix(size: usize) -> Matrix {
    let mut identity_matrix = create_matrix(size, size);

    for diag_element in 0..size {
        identity_matrix.element[diag_element][diag_element] = Parameter{expression: vec![String::from("1")], value: 1.0};
    }

    identity_matrix
}

//First time ever writing unit test: it sucks
// like it really sucks omg
// OMG AND ILL HAVE TO REWRITE ALL OF THE PANIC ONES IF I DECIDE THAT I NEED A DIFFERENT BEHAVIOR FAH
#[cfg(test)]
mod tests {
    

use super::*;
    struct TestMatrix {
        identity: Matrix,
        empty: Matrix,
        vector: Matrix,
        three_by_three: Matrix,
        two_by_three: Matrix,
        three_by_two: Matrix,
    }
    impl Default for TestMatrix {
        fn default() -> TestMatrix {
            TestMatrix {
                identity: Matrix {
                    element: vec![vec![Parameter { expression: vec![String::from("1")], value: 1.0}, Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } ], 
                        vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("1")], value: 1.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } ], 
                        vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("1")], value: 1.0 } ]],
                    rows: 3,
                    colums: 3
                },
                empty: Matrix {
                    element: vec![vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } ], 
                        vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } ], 
                        vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } ]],
                    rows: 3,
                    colums: 3
                },
                vector: Matrix {
                    element: vec![vec![Parameter { expression: vec![String::from("0")], value: 0.0 } ], 
                        vec![Parameter { expression: vec![String::from("1")], value: 1.0 } ], 
                        vec![Parameter { expression: vec![String::from("2")], value: 2.0 } ]],
                    rows: 3,
                    colums: 1
                },
                three_by_three: Matrix {
                    element: vec![vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("6")], value: 6.0 } , Parameter { expression: vec![String::from("8")], value: 8.0 } ], 
                        vec![Parameter { expression: vec![String::from("3")], value: 3.0 } , Parameter { expression: vec![String::from("1")], value: 1.0 } , Parameter { expression: vec![String::from("7")], value: 7.0 } ], 
                        vec![Parameter { expression: vec![String::from("5")], value: 5.0 } , Parameter { expression: vec![String::from("4")], value: 4.0 } , Parameter { expression: vec![String::from("2")], value: 2.0 } ]],
                    rows: 3,
                    colums: 3
                },
                two_by_three: Matrix {
                    element: vec![vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("1")], value: 1.0 } , Parameter { expression: vec![String::from("2")], value: 2.0 } ], 
                        vec![Parameter { expression: vec![String::from("3")], value: 3.0 } , Parameter { expression: vec![String::from("4")], value: 4.0 } , Parameter { expression: vec![String::from("5")], value: 5.0 } ]],
                    rows: 2,
                    colums: 3
                },
                three_by_two: Matrix {
                    element: vec![vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("3")], value: 3.0 } ], 
                        vec![Parameter { expression: vec![String::from("1")], value: 1.0 } , Parameter { expression: vec![String::from("4")], value: 4.0 } ], 
                        vec![Parameter { expression: vec![String::from("2")], value: 2.0 } , Parameter { expression: vec![String::from("5")], value: 5.0 } ]],
                    rows: 3,
                    colums: 2
                },
            }
        }
    }
    
    #[test]
    fn identity_matrix_test() {
        let matrices = TestMatrix::default();
        let result_matrix = identity_matrix(3);
        let expected_matrix = matrices.identity;
        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn create_matrix_test() {
        let matrices = TestMatrix::default();
        let result_matrix = create_matrix(3, 3);
        let expected_matrix = matrices.empty;
        assert_eq!(result_matrix, expected_matrix);
    }

    //NOTE: test should be rewritten at a later date for a test case besides the identity matrix
    #[test]
    fn inverse_test() { // I refuse to fix this fuck -0.0 and fuck floating point ts john works in theory
        let matrices = TestMatrix::default();

        let input_matrix = matrices.identity.clone();
        let result_matrix = input_matrix.inverse();

        let expected_matrix = matrices.identity;

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    #[should_panic]
    fn inverse_test_panic() {
        let matrices = TestMatrix::default();

        let input_matrix = matrices.two_by_three;
        let _result_matrix = input_matrix.inverse();
    }

    #[test]
    fn adjoint_test() {
        let matrices = TestMatrix::default();

        let input_matrix = matrices.three_by_three;
        let result_matrix = input_matrix.adjoint();

        let expected_matrix = Matrix {
            element: vec![vec![Parameter { expression: vec![String::from("-26")], value: -26.0 } , Parameter { expression: vec![String::from("20")], value: 20.0 } , Parameter { expression: vec![String::from("34")], value: 34.0 } ], 
                vec![Parameter { expression: vec![String::from("29")], value: 29.0 } , Parameter { expression: vec![String::from("-40")], value: -40.0 } , Parameter { expression: vec![String::from("24")], value: 24.0 } ], 
                vec![Parameter { expression: vec![String::from("7")], value: 7.0 } , Parameter { expression: vec![String::from("30")], value: 30.0 } , Parameter { expression: vec![String::from("-18")], value: -18.0 } ]],
            rows: 3,
            colums: 3
        };

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    #[should_panic]
    fn adjoint_test_panic() {
        let matrices = TestMatrix::default();

        let input_matrix = matrices.two_by_three;
        let _result_matrix = input_matrix.adjoint();
    }

    #[test]
    fn determinant_test() {
        let matrices = TestMatrix::default();

        let input_matrix = matrices.three_by_three;
        let result = input_matrix.determinant();
        let expected_value = 230.0;

        //might not work due to floating point, which will need to be fixed somehow
        assert_eq!(result.value, expected_value);
    }

    #[test]
    #[should_panic]
    fn determinant_test_panic() {
        let matrices = TestMatrix::default();

        let input_matrix = matrices.three_by_two;
        let _result_expression = input_matrix.determinant();
    }

    #[test]
    fn submatrix_test() {
        let matrices = TestMatrix::default();

        let input_matrix = matrices.identity.clone();
        let result_matrix = input_matrix.submatrix(0, 0);

        let expected_matrix = identity_matrix(2);

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn transpose_test() {
        let matrices = TestMatrix::default();

        let input_matrix = matrices.two_by_three.clone();
        let result_matrix = input_matrix.transpose();

        let expected_matrix = matrices.three_by_two;

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn multiply_matrix_test_vector() {
        let matrices = TestMatrix::default();

        let input_matrix_1 = matrices.identity.clone();
        let input_matrix_2 = matrices.vector.clone();

        let result_matrix = input_matrix_1.multiply_matrix(&input_matrix_2);

        let expected_matrix = matrices.vector;

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn multiply_matrix_test_non_vector() {
        let matrices = TestMatrix::default();

        let input_matrix_1 = matrices.identity.clone();
        let input_matrix_2 = matrices.three_by_three.clone();

        let result_matrix = input_matrix_1.multiply_matrix(&input_matrix_2);

        let expected_matrix = matrices.three_by_three;

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    #[should_panic]
    fn multiply_matrix_test_panic() {
        let matrices = TestMatrix::default();

        let input_matrix_1 = matrices.identity.clone();
        let input_matrix_2 = matrices.two_by_three.clone();

        let _result_matrix = input_matrix_1.multiply_matrix(&input_matrix_2);
    }

    #[test]
    fn add_matrix_test() {
        let matrices = TestMatrix::default();

        let input_matrix_1 = matrices.identity.clone();
        let input_matrix_2 = matrices.empty.clone();

        let result_matrix = input_matrix_1.add_matrix(&input_matrix_2);

        let expected_matrix = matrices.identity;

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    #[should_panic]
    fn add_matrix_test_panic() {
        let matrices = TestMatrix::default();

        let input_matrix_1 = matrices.vector.clone();
        let input_matrix_2 = matrices.empty.clone();

        let _result_matrix = input_matrix_1.add_matrix(&input_matrix_2);
    }

    #[test]
    fn multiply_scalar_test() {
        let matrices = TestMatrix::default();
        
        let input_matrix_1 = matrices.identity.clone();
        let scalar = Parameter{expression: vec![String::from("1")], value: 1.0};

        let result_matrix = input_matrix_1.multiply_scalar(&scalar);
        
        let expected_matrix = matrices.identity;
    
        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn set_test() {
        let matrices = TestMatrix::default();

        let mut matrix = matrices.empty;
        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0};
        matrix.set(0, 0, &parameter_1);
        matrix.set(1, 1, &parameter_1);
        matrix.set(2, 2, &parameter_1);
        let expected_matrix = matrices.identity;
        assert_eq!(matrix, expected_matrix);
    }

    #[test]
    #[should_panic]
    fn set_test_panic() {
        let matrices = TestMatrix::default();

        let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0};
        let mut matrix = matrices.empty;
        matrix.set(3, 3, &parameter_1);
    }
}

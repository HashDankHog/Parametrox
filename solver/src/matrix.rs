//having a dedicated matrix structure rather than just Vec<Vec<f64>> makes it easier to prevent errors when declaring matrices
//it also allows for easy implementation of matrix operations
// NOTE: the rows and colums attributes might be unnesescary, but at the same time I am not sure
//TODO
//  - add, in a different file, the ability to round to a certain number of digits
#[derive(PartialEq, Debug)]
struct Matrix
{
    //naming this variable element makes any code that reads a specific element of the matrix more readable
    element: Vec<Vec<f64>>,
    rows: usize,
    colums: usize
}
impl Matrix
{
    fn print(&self) {
        for row in 0..self.rows
        {
            for colum in 0..self.colums
            {
                print!("{} ", self.element[row][colum]);
            }
            println!();
        }
        println!();
    }
    fn set(&mut self, element: [usize; 2], value: f64) {
        let row = element[0];
        let colum = element[1];
        if (row > self.rows) || (colum > self.colums) 
        {
            panic!("attempted to access an element beyond the bounds of the element");
        }
        self.element[row][colum]=value;
    }
    fn multiply_scalar(&self, scalar: f64) -> Matrix {
        let mut result_matrix = create_matrix(self.rows, self.colums);

        for row in 0..self.rows
        {
            for colum in 0..self.colums
            {
                result_matrix.element[row][colum] = self.element[row][colum] * scalar;
            }
        }
        result_matrix
    }
    fn add_matrix(&self, matrix: &Matrix) -> Matrix {
        if (matrix.rows != self.rows) || (matrix.colums != self.colums)
        {
            panic!("cannot add matrices of different dimensions");
        }

        let mut result_matrix = create_matrix(self.rows, self.colums);

        for row in 0..self.rows
        {
            for colum in 0..self.colums
            {
                result_matrix.element[row][colum] = self.element[row][colum] + matrix.element[row][colum];
            }
        }
        result_matrix
    }
    //strassens algorithm not implemented yet due to inneficeeny for n < 100
    fn multiply_matrix(&self, matrix: &Matrix) -> Matrix {
        if matrix.rows != self.colums
        {
            panic!("yo dimensions aint correct twin");
        }

        let mut result_matrix = create_matrix(self.rows, matrix.colums);
    
        for row in 0..self.rows
        {
            for colum in 0..matrix.colums
            {
                //colum_mult is not a good variable name
                for colum_mult in 0..self.colums
                {
                    result_matrix.element[row][colum] += self.element[row][colum_mult]*matrix.element[colum_mult][colum];
                } 
            }
        }
        result_matrix
    }
    fn transpose(&self) -> Matrix {
        let mut result_matrix = create_matrix(self.colums, self.rows);
        for row in 0..self.colums {
            for colum in 0..self.rows {
                result_matrix.element[row][colum] = self.element[colum][row];
            }
        }
        result_matrix
    }
}

//NOTE: inconsistent typing with the set function
fn create_matrix(rows: usize, colums: usize) -> Matrix {
    Matrix {
        element: vec![vec![0.0; colums]; rows],
        rows: rows,
        colums: colums
    }
}
fn identity_matrix(size: usize) -> Matrix {
    let mut identity_matrix = create_matrix(size,size);
    for diag_element in 0..size {
        identity_matrix.element[diag_element][diag_element] = 1.0;
    }
    identity_matrix
}


//First time ever writing unit test: it sucks
// like it really sucks omg
// OMG AND ILL HAVE TO REWRITE ALL OF THE PANIC ONES IF I DECIDE THAT I NEED A DIFFERENT BEHAVIOR FAH
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_matrix_test() {
        let result_matrix = identity_matrix(2);
        let expected_matrix = Matrix {
            element: vec![vec![1.0, 0.0],
                          vec![0.0, 1.0]
                        ],
            rows: 2,
            colums: 2
        };
        assert_eq!(result_matrix,expected_matrix);
    }

    #[test]
    fn create_matrix_test() {
        let result_matrix = create_matrix(2,2);
        let expected_matrix = Matrix {
            element: vec![vec![0.0, 0.0],
                          vec![0.0, 0.0]
                         ],
            rows: 2,
            colums: 2
        };
        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn transpose_test() {
        let matrix = Matrix {
            element: vec![vec![0.0, 1.0, 2.0],
                          vec![3.0, 4.0, 5.0]
                        ],
            rows: 2,
            colums: 3          
        };

        let result_matrix = matrix.transpose();

        let expected_matrix = Matrix {
            element: vec![vec![0.0, 3.0],
                          vec![1.0, 4.0],
                          vec![2.0, 5.0]
                        ],   
            rows: 3,
            colums: 2   
        };

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn multiply_matrix_test_vector() {
        let matrix_1 = Matrix {
            element: vec![vec![0.0, 1.0, 2.0],
                          vec![3.0, 4.0, 5.0]
                          ],
            rows: 2,
            colums: 3
        };
        let matrix_2 = Matrix {
            element: vec![vec![0.0],
                          vec![1.0],
                          vec![2.0]
                          ],
            rows: 3,
            colums: 1
        };

        let result_matrix = matrix_1.multiply_matrix(&matrix_2);
        let expected_matrix = Matrix{
            element: vec![vec![5.0],
                          vec![14.0]
                          ],
            rows: 2,
            colums: 1
        };
        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn multiply_matrix_test_non_vector() {
        let matrix_1 = Matrix {
            element: vec![vec![0.0, 1.0, 2.0],
                          vec![3.0, 4.0, 5.0]
                          ],
            rows: 2,
            colums: 3
        };
        let matrix_2 = Matrix {
            element: vec![vec![0.0, 1.0],
                          vec![2.0, 3.0],
                          vec![4.0, 5.0]
                          ],
            rows: 3,
            colums: 2
        };

        let result_matrix = matrix_1.multiply_matrix(&matrix_2);
        let expected_matrix = Matrix{
            element: vec![vec![10.0, 13.0],
                          vec![28.0, 40.0]
                          ],
            rows: 2,
            colums: 2
        };
        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    #[should_panic]
    fn multiply_matrix_test_panic() {
        let matrix_1 = Matrix {
            element: vec![vec![0.0, 1.0],
                          vec![2.0, 3.0]
                          ],
            rows: 2,
            colums: 2
        };
        let matrix_2 = Matrix {
            element: vec![vec![0.0, 1.0],
                          vec![2.0, 3.0],
                          vec![4.0, 5.0]
                          ],
            rows: 3,
            colums: 2
        };

        let result_matrix = matrix_1.multiply_matrix(&matrix_2);   
    }
    
    #[test]
    fn add_matrix_test() {
        let matrix_1 = Matrix {
            element: vec![vec![0.0, 1.0],
                          vec![2.0, 3.0]
                          ],
            rows: 2,
            colums: 2
        };
        let matrix_2 = Matrix {
            element: vec![vec![0.0, 1.0],
                          vec![2.0, 3.0]
                          ],
            rows: 2,
            colums: 2
        };
        
        let result_matrix = matrix_1.add_matrix(&matrix_2);

        let expected_matrix = Matrix {
            element: vec![vec![0.0, 2.0],
                          vec![4.0, 6.0]
                          ],
            rows: 2,
            colums: 2
        };

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    #[should_panic]
    fn add_matrix_test_panic() {
        let matrix_1 = Matrix {
            element: vec![vec![0.0, 1.0],
                          vec![2.0, 3.0]
                          ],
            rows: 2,
            colums: 2
        };
        let matrix_2 = Matrix {
            element: vec![vec![0.0, 1.0],
                          ],
            rows: 2,
            colums: 2
        };
        
        let result_matrix = matrix_1.add_matrix(&matrix_2);
    }

    #[test]
    fn multiply_scalar_test() {
        let matrix_1 = Matrix {
            element: vec![vec![0.0, 1.0],
                          vec![2.0, 3.0]
                          ],
            rows: 2,
            colums: 2
        };

        let result_matrix = matrix_1.multiply_scalar(2.0);
        
        let expected_matrix = Matrix {
            element: vec![vec![0.0, 2.0],
                          vec![4.0, 6.0]
                          ],
            rows: 2,
            colums: 2
        };

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn set_test() {
        let mut matrix = Matrix {
            element: vec![vec![0.0, 0.0],
                          vec![0.0, 0.0]
                          ],
            rows: 2,
            colums: 2
        };
        matrix.set([0,0],1.0);
        let expected_matrix = Matrix {
            element: vec![vec![1.0, 0.0],
                          vec![0.0, 0.0]
                          ],
            rows: 2,
            colums: 2
        };

        assert_eq!(matrix, expected_matrix);
    }

    #[test]
    #[should_panic]
    fn set_test_panic() {
        let mut matrix = Matrix {
            element: vec![vec![0.0, 0.0],
                          vec![0.0, 0.0]
                          ],
            rows: 2,
            colums: 2
        };
        matrix.set([2,2],1.0);
    }

}

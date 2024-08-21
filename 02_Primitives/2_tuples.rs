use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
  }
}

fn main() {
  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("Matrix:\n{}", matrix);
  let transposed_matrix = transpose(matrix);
  println!("Transpose:\n{}", transposed_matrix);
}

fn transpose(matrix: Matrix) -> Matrix {
  Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
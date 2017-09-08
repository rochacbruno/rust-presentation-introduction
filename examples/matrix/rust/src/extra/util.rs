pub fn load_matrix(size: usize) -> Matrix {
    let mut m = Matrix::new(size, size);
    let data: Vec<u32> = (0..size as u32).collect();

    for row in 0..size {
        for col in 0..data.len() {
            m[(col, row)] = data[col];
        }
    }
    m
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let n = 3;
    let mut transposed = [[0; 3]; 3];

    for i in 0..n {
        for j in i..n {
            transposed[i][j] = matrix[j][i];
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Original:");
    for row in matrix {
        println!("{row:?}");
    }

    let transposed = transpose(matrix);

    println!("\nTransposed:");
    for row in transposed {
        println!("{row:?}");
    }
}

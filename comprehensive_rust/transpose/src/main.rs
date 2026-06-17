/*
 *           ⎛⎡1, 2, 3⎤⎞    ⎡1, 2, 3⎤
 * transpose ⎜⎜4, 5, 6⎟⎟ == ⎜4, 5, 6⎟
 *           ⎝⎣7, 8, 9⎦⎠    ⎣7, 8, 9⎦
 */
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let n = 3;
    let mut transposed = [[0; 3]; 3];
    for i in 0..n {
        for j in 0..n {
            transposed[i][j] = matrix[j][i];
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

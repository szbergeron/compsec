fn main() {
    println!("doubletranspose output: {}", doubletranspose("wearealltogether", &[1,3,0,2], &[2,0,1,3]));
}

// rows is the outer, cols is the inner dimension
fn doublematrix<T>(rows: usize, cols: usize, default: T) -> Box<[Box<[T]>]> where T: Copy {
    let mut components: Vec<Box<[T]>> = Vec::new();
    for _ in 0..rows {
        //components.push(Box::new([default; cols]));
        let v = vec![default; cols];
        components.push(v.into_boxed_slice());
    }
    components.into_boxed_slice()
}

fn doubletranspose(content: &str, rows: &[usize], cols: &[usize]) -> String {
    //let mut matrix = doublematrix(rows.len(), cols.len());
    let mut matrix = doublematrix(rows.len(), cols.len(), ' ');
    let mut iter = content.chars();
    for row in 0..rows.len() {
        for col in 0..cols.len() {
            matrix[row][col] = iter.next().expect("input was cut short");
        }
    }

    let mut output = doublematrix(rows.len(), cols.len(), ' ');
    // do permute
    for row in 0..rows.len() {
        for col in 0..cols.len() {
            output[rows[row]][cols[col]] = matrix[row][col];
        }
    }

    let mut strout = String::new();
    for row in 0..rows.len() {
        for col in 0..cols.len() {
            strout.push(output[row][col]);
        }
    }

    strout
}

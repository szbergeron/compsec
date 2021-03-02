fn main() {
    println!("doubletranspose output: {}", doubletranspose("wearealltogether", &[1,3,0,2], &[2,0,1,3]));

    do_feistal();
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

    println!("input:");
    print!("    ");
    for i in 0..cols.len() {
        print!(" {}  ", i + 1);
    }
    for row in 0..rows.len() {
        print!("\n {} |", row + 1);
        for col in 0..cols.len() {
            matrix[row][col] = iter.next().expect("input was cut short");
            print!(" {} |", matrix[row][col]);
        }
    }
    print!("\n");

    let mut output = doublematrix(rows.len(), cols.len(), ' ');
    // do permute
    for row in 0..rows.len() {
        for col in 0..cols.len() {
            //output[rows[row]][cols[col]] = matrix[row][col];
            output[row][col] = matrix[rows[row]][cols[col]];
        }
    }

    let mut strout = String::new();

    println!("output:");

    print!("    ");
    for i in 0..cols.len() {
        print!(" {}  ", cols[i] + 1);
    }
    for row in 0..rows.len() {
        print!("\n {} |", rows[row] + 1);
        for col in 0..cols.len() {
            print!(" {} |", output[row][col]);
            //print!("  {} |", matrix[rows[row]][cols[col]]);
            strout.push(output[row][col]);
        }
    }
    print!("\n");

    strout
}

use std::rc::Rc;

#[derive(Debug)]
struct LeftBlock {
    ith: usize,

    derived_from: Rc<Operation>, // should be an Xor
}

#[derive(Debug)]
struct RightBlock {
    ith: usize,
    derived_from: Rc<Operation>,
    //
}

#[derive(Debug)]
struct Xor {
    firstop: Rc<Operation>,
    secondop: Rc<Operation>,
}

#[derive(Debug)]
enum Operation {
    CoreRight(),
    CoreLeft(),
    IdentityRight(RightBlock),
    IdentityLeft(LeftBlock),
    IdentityKey(Rc<Key>),
    Xor(Xor),
    OpResult { left: Rc<Operation>, right: Rc<Operation> },
    Zero(),
    //Xor { firstop: Operation, secondop: Operation },
    //
}

//struct 

/*enum Combination {
}*/

/*enum Block {
    Right(RightBlock),
    Left(LeftBlock),
    Combined(LeftBlock, RightBlock),
}*/

#[derive(Debug)]
struct Key {
    ith: usize,
    derived_from: Option<Rc<Key>>,
}

fn feistal_1(_right: Rc<Operation>, _key: Rc<Key>) -> Rc<Operation> {
    Rc::new(Operation::Zero())
}

fn feistal_2(right: Rc<Operation>, _key: Rc<Key>) -> Rc<Operation> {
    right
}

fn feistal_3(_right: Rc<Operation>, key: Rc<Key>) -> Rc<Operation> {
    Rc::new(Operation::IdentityKey(key))
}

fn feistal_4(right: Rc<Operation>, key: Rc<Key>) -> Rc<Operation> {
    Rc::new(Operation::Xor(Xor {
        firstop: right.clone(),
        secondop: Rc::new(Operation::IdentityKey(key))}))
}

fn feistal_symbolic(rounds: usize, function: &dyn Fn(Rc<Operation>, Rc<Key>) -> Rc<Operation>) -> Rc<Operation> {
    let mut curleft = Rc::new(Operation::CoreLeft());
    let mut curright = Rc::new(Operation::CoreRight());
    let mut curkey = Rc::new(Key { ith: 0, derived_from: None });

    for round in 0..rounds {
        let newleft = Rc::new(Operation::IdentityLeft(LeftBlock { ith: round, derived_from: curright.clone() }));
        let opresult: Rc<Operation> = function(curright.clone(), curkey.clone());
        let newright = Rc::new(Operation::IdentityRight(RightBlock {
            ith: round,
            derived_from: Rc::new(Operation::Xor(Xor { firstop: curleft.clone(), secondop: opresult}))}));

        curleft = newleft;
        curright = newright;
        curkey = Rc::new(Key { ith: curkey.ith + 1, derived_from: Some(curkey.clone())});
    }

    Rc::new(Operation::OpResult { left: curleft, right: curright })
}

fn do_feistal() {
    println!("f1 result: {:#?}", feistal_symbolic(4, &feistal_1));
    println!("f2 result: {:#?}", feistal_symbolic(4, &feistal_2));
    println!("f3 result: {:#?}", feistal_symbolic(4, &feistal_3));
    println!("f4 result: {:#?}", feistal_symbolic(4, &feistal_4));
}

/*fn feistal_symbolic(rounds: usize, function: &dyn Fn(Rc<Block>, Rc<Key>) -> (Rc<Block>, Rc<Key>)) {
    for round in 0..rounds {
    }
}*/

fn a5_1_keystream(input: &[u8], mut key_x: u32, mut key_y: u32, mut key_z: u32) -> Vec<u8> {
    for _byte in input {
        for _bit in 0..8 {
            // not implemented, need to do the xor stuff with the stream and the registers
        }
    }

    panic!()
}

//fn feistel_encode(plaintext

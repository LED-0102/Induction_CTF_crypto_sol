use std::fs::File;
use std::io::Read;
use hex;
fn main() -> std::io::Result<()> {
    let mut file = File::open("/home/darshil/LED/CTFs/Induction CTF/crypto1/output.txt")?;

    let mut ans = String::new();
    file.read_to_string(&mut ans)?;
    let key = "Noob!".as_bytes().to_owned();
    println!("{}", ans);
    let mut byte: Vec<u8> = vec![];
    if let Ok(bytes) = hex::decode(ans) {
        byte = bytes;
    }
    let statements = vec![
        "You are not smart".as_bytes().to_owned(), "No way you're smart".as_bytes().to_owned(), "Noob".as_bytes().to_owned(), "Pro Noob".as_bytes().to_owned()
    ];

    for i in 0..16 {
        let mut cl = byte.clone();
        for j in 0..5 {
            if (i & (1 << j)) >> j == 1 {
                xor_vectors(&statements[j], &mut cl);
            }
        }
        xor_vectors(&key, &mut cl);
        if let Ok(my_string) = String::from_utf8(cl.clone()) {
            if my_string.starts_with("flag{"){
                println!("{i}");
                println!("{my_string}");
                break;
            }
            // println!("{}", &my_string); // Print the first 8 characters
        } else {
            println!("Invalid");
        }
    }
    Ok(())
}

fn xor_vectors(b: &Vec<u8>, c: &mut Vec<u8>) {
    for i in 0..c.len() {
        c[i] ^= b[i%b.len()];
    }
}


mod column;
mod des;
mod vigenere;

use std::io::{self, stderr, stdout, Write};

fn main() {
    vigenere_test();
    column_test();
    des_test();
    des_test_file();
}

fn vigenere_test() {
    use vigenere::{decode, encode};

    eprintln!("{:-^40}", "-");
    eprintln!("vigenere");
    eprintln!();

    let plain = Vec::from("asdfasdf");
    let cipher = Vec::from("akgkakgk");
    let key = Vec::from("asdf");

    eprint!("plaintext: ");
    stderr().write(plain.as_slice()).unwrap();
    eprintln!();

    eprint!("key: ");
    stderr().write(key.as_slice()).unwrap();
    eprintln!();

    eprint!("encrypted: ");
    //encode
    io::stderr()
        .write_all(encode(&plain, &key).as_slice())
        .unwrap();
    eprintln!();

    eprint!("decrypted: ");
    //decode
    io::stderr()
        .write_all(decode(&cipher, &key).as_slice())
        .unwrap();
    eprintln!();

    stdout().flush().unwrap();
    stderr().flush().unwrap();
}

fn column_test() {
    use column::{decode, encode};

    eprintln!("{:-^40}", "-");
    eprintln!("column");
    eprintln!();

    let plain = Vec::from("asdfasdfqwerqwerq");
    let cipher = Vec::from("aaqqqffrrxsswwxddeex");
    let key = Vec::from("asdf");

    eprint!("plaintext: ");
    stderr().write(plain.as_slice()).unwrap();
    eprintln!();

    eprint!("key: ");
    stderr().write(key.as_slice()).unwrap();
    eprintln!();

    eprint!("encrypted: ");
    //encode
    io::stderr()
        .write_all(encode(&plain, &key).as_slice())
        .unwrap();
    eprintln!();

    eprint!("decrypted: ");
    //decode
    io::stderr()
        .write_all(
            match decode(&cipher, &key) {
                Ok(plaintext) => plaintext,
                Err(info) => panic!("{}", info),
            }
            .as_slice(),
        )
        .expect("io error");
    eprintln!();

    stdout().flush().unwrap();
    stderr().flush().unwrap();
}

fn des_test() {
    use des::{decrypt_bytes, encrypt_bytes};

    eprintln!("{:-^40}", "-");
    eprintln!("des");
    eprintln!();

    let plain = b"lorem ipsum";
    let key = b"password";
    let encrypted = encrypt_bytes(plain, key).clone();
    let decrypted = decrypt_bytes(encrypted.as_slice(), key);

    eprint!("plaintext: ");
    stderr().write(plain).unwrap();
    eprintln!("\nbyte array: {:?}", plain);
    eprintln!();

    eprint!("key: ");
    stderr().write(key).unwrap();
    eprintln!("\nbyte array: {:?}", key);
    eprintln!();

    eprint!("encrypted: ");
    stderr().write(encrypted.as_slice()).unwrap();
    eprintln!("\nbyte array: {:?}", encrypted);
    eprintln!();

    eprint!("decrypted: ");
    stderr().write(decrypted.as_slice()).unwrap();
    eprintln!("\nbyte array: {:?}", decrypted);
    eprintln!();

    stdout().flush().unwrap();
    stderr().flush().unwrap();
}

fn des_test_file() {
    use des::{decrypt_bytes, encrypt_bytes};
    use std::fs::{read, read_to_string, File};
    eprintln!("{:-^40}", "-");
    eprintln!("des(from stdin)");
    eprintln!();

    let key = b"password";

    eprint!("key: ");
    stderr().write(key).unwrap();
    eprintln!("\nbyte array: {:?}", key);
    eprintln!();

    let plaintext = read_to_string("des_plaintext.txt").expect("read des_plaintext.txt error");
    let encrypted = encrypt_bytes(plaintext.as_bytes(), key);
    let mut encrypted_file =
        File::create("des_encrypted.dat").expect("create des_encrypted.dat error");
    encrypted_file
        .write_all(encrypted.as_slice())
        .expect("write des_encrypted.dat error");

    // let decrypted = encrypt_bytes(plaintext.as_bytes(), key);
    // let decrypted_file = File::create("des_decrypted.txt").expect("create des_decrypted.txt error");
    // decrypted_file.write_all(decrypted);

    //let encrypted_file = read("des_encrypted.dat").expect("create des_encrypted.dat error");

    File::create("des_decrypted.txt")
        .expect("create des_decrypted.txt error")
        .write_all(
            decrypt_bytes(
                read("des_encrypted.dat")
                    .expect("create des_encrypted.dat error")
                    .as_slice(),
                key,
            )
            .as_slice(),
        )
        .expect("write des_decrypted.dat error");

    stdout().flush().unwrap();
    stderr().flush().unwrap();
}

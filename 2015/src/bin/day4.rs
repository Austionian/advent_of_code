fn main() {
    let mut res = 0;
    loop {
        let digest = md5::compute(format!("bgvyzdsv{res}"));
        if &digest[0..3] == &[0x00, 0x00, 0x00] {
            println!("{res}");
            break;
        }
        res += 1;
    }
}

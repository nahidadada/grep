mod grep;
fn main() {
    let flags = grep::Flags::new();

    let arr: Vec<&str> = flags.files.iter().map(|e| e.as_str()).collect();
    let ret = grep::grep(&flags.pattern, &flags, &arr);
    if ret.is_ok() {
        let arr = ret.unwrap();
        for s in arr.iter() {
            println!("{}", s);
        }
    }
}

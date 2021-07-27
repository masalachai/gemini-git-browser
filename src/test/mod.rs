use crate::util::md_to_gemtext;
use std::{fs::File, io::Read};

#[test]
fn test_md_to_gemtext() {
    let mut f = File::open("README.md").unwrap();
    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();

    let mut gem_f = File::open("tests/resources/README.gmi").unwrap();
    let mut gem_buf = String::new();

    gem_f.read_to_string(&mut gem_buf).unwrap();

    let gemtext = md_to_gemtext(buf.as_str()).unwrap();

    assert_eq!(gem_buf, gemtext);
}

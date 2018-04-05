extern crate tini;
use tini::Ini;

static INPUT: &'static str = "./examples/example.ini";
static OUTPUT: &'static str = "./examples/test.ini";
static SPLIT: &'static str = "=----------------------------------------------------------=";

fn main() {
    let config = Ini::from_file(INPUT).unwrap();
    println!(">> read from `{0}`\n{1}\n{2}\n{1}", INPUT, SPLIT, config);
    let n1: u32 = config.get("section_one", "name1").unwrap_or(0);
    println!(">> entry `name1` from `section_one` = {}", n1);
    let n2: Vec<bool> = config.get_vec("section_three", "frst4").unwrap_or(vec![false]);
    println!(">> entry `frst4` from `section_three` = {:?}", n2);
    let test = Ini::new()
                   .section("section_one")
                   .item("a", "1")
                   .item("b", "2")
                   .section("section_two")
                   .item("c", "3")
                   .item("d", "4");
    println!(">> built `{0}` config\n{1}\n{2}\n{1}", OUTPUT, SPLIT, test);
    test.to_file(OUTPUT).unwrap();
}

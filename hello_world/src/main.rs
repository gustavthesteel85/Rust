fn main() {
    println!("Hello, world!");

    // イテレーター
    let objective: Option<i32> = Some(1);
    match objective {
        Some(x) if x % 2 == 0 => println!("偶数です": {}, x),
        Some(x) => println!("奇数です": {}, x),
        None => println!("値がありません"),
    }

    let mut v = vec![];
    v.push(1);
}

pub enum Oprion<T> {
    None,
    Some(T),
}

trait Animal {
    fn lifespan(&self) -> u32;
    fn scientific_name(&self) -> String;
}

// 犬の構造体
struct Dog;

// aaa
impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }
    fn scientific_name(&self) -> String {
        "Canis lupus familiaris".to_string()
    }
}

struct Cat

impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        13
    }
    fn scientific_name(&self) -> String {
        "Felis catus ".to_string()
    }
}

// 寿命と学術名
fn show_animal_data<T: Animal>(animal: T) {
    println!("Lifespan: {} years", animal.lifespan())
    println!("Scientific name: {} years", animal.scientific_name())
}

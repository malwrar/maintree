//! Goal: store trait implementers in a wrapped vec without making pushes unwieldy.
extern crate rand;
use rand::Rng;

trait Common {
    fn value(&self) -> i32;
}

struct Implementer1 { member: bool }
impl Common for Implementer1 {
    fn value(&self) -> i32 {
        self.member as i32
    }
}

struct Implementer2 { member: f32 }
impl Common for Implementer2 {
    fn value(&self) -> i32 {
        self.member as i32
    }
}

struct Implementer3 { member: u32 }
impl Common for Implementer3 {
    fn value(&self) -> i32 {
        self.member as i32
    }
}

struct Storage {
    storage: Vec<Box<dyn Common>>,
}

impl Storage {
    fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    fn push<T: Common + 'static>(&mut self, item: T) {
        // TODO: I'm not sure what the consequences of `+ 'static` are, besides
        //       ensuring acceptable types for `item` aren't going to expire
        //       when they go out of scope. Need to figure that out.
        self.storage.push(Box::new(item));
    }

    fn iter(&self) -> std::slice::Iter<Box<dyn Common>> {
        self.storage.iter()
    }
}

fn foo() -> f32 {
    let ret = rand::thread_rng().gen_range(0.0..100.0);
    println!("(btw, rand val is {})", ret);
    ret
}

fn main() {
    let mut storage = Storage::new();
    
    let blah = Implementer1 { member: true };
    storage.push(blah);
    let sum = 1.0 + foo();
    storage.push(Implementer2 { member: sum });
    storage.push(Implementer3 { member: 9001 });

    storage.iter()
        .for_each(|item| println!("{}", item.value()))
}
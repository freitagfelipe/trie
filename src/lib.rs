pub fn hello_world() {
    println!("Hello world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        hello_world();
    }
}
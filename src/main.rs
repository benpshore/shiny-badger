fn greeting() -> String {
    String::from("Hello from shiny-badger!")
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greets() {
        assert!(greeting().contains("shiny-badger"));
    }
}

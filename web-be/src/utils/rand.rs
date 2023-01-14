use rand::{thread_rng, Rng};

pub fn generate_numbers(len: u32) -> String {
    let mut s = String::new();
    let mut rng = thread_rng();
    for _ in 0..len {
        let num = rng.gen_range(0..9);
        s.push_str(num.to_string().as_str());
    }

    s
}

#[cfg(test)]
mod tests {
    use super::generate_numbers;

    #[test]
    fn test_generate_numbers() {
        let r = generate_numbers(6);
        println!("{r}");
    }
}

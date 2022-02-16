#[allow(unused_macros)]
macro_rules! get_input {
    ($filename:expr) => {
        std::str::from_utf8(&std::fs::read($filename).unwrap()).unwrap()
    };
}

pub fn read<T>(in_string: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    in_string.parse::<T>().unwrap()
}

// Prints the count if it is a power of two
#[derive(Debug)]
pub struct BaseTwoCounter {
    pub count: usize,
    pub pow_2: usize
}
impl BaseTwoCounter {
    /// Increment by one
    pub fn inc(&mut self) {
        self.count += 1;
        if self.count >= self.pow_2 {
            println!("count: {}", self.count);
            self.pow_2 *= 2;
        }
    }
}
impl Default for BaseTwoCounter {
    fn default() -> Self {
        Self {
            count: 0,
            pow_2: 1
        }
    }
}
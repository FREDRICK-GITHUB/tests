pub fn add_two (a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder (a: i32, b: i32) -> i32  {
    a + b
}

#[cfg(test)]
mod tests {
    /* this brings into scope the parent's items to the private function in this module */
    use super::*;

    #[test]
    fn internal () {
        assert_eq!(4, add_two(2));
    }

}
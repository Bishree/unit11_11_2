pub fn print_and_return_10 (a:i32) -> i32 {
    println!("I've got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use crate::print_and_return_10;
    use super::*;

    #[test]
    fn this_test_will_pass (){
        let value = print_and_return_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_fail () {
        let value = print_and_return_10(8);
        assert_eq!(5 , value);
    }

    #[test]
    #[ignore]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}

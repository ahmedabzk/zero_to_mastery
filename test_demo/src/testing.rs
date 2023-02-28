fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

// divide a and b
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0{
        return None
    }else{
        Some(a / b)
    }
    
}

// Takes two strings and places them immediately one after another
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}


#[cfg(test)]
mod test {
    use crate::testing::{clamp, div, concat};

    #[test]
    fn check_lower() {
        let result_num = clamp(10, 100, 1000);
        let expected = 100;

        assert_eq!(result_num, expected, "should get 100");
    }
    #[test]
    fn check_upper() {
        let result_num = clamp(5000, 100, 1000);
        let expected = 1000;

        assert_eq!(result_num, expected, "should get 1000");
    }

    #[test]
    fn check_div(){
        let result = div(1, 1);
        let expected = Some(1);

        assert_eq!(result, expected);
    }

     #[test]
    fn check_zero_div(){
        let result = div(1, 0);
        let expected = None;

        assert_eq!(result, expected, "cannot divide by zero");
    }

    #[test]
    fn check_concat(){
        let result = concat("a", "b");
        let expected = String::from("ab");

        assert_eq!(result, expected);
    }
}

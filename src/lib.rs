#![no_std]

/// Splits a string reference into exactly three u16 numbers where the
/// numbers are separated by a delimiter.
/// 
/// Returns None if there are not exactly three numbers or if the 
/// numbers do not parse or cannot fit into a u16 sized value.
pub fn split_string_3(value: &str, delimiter: &str) -> Option<(u16, u16, u16)> {
    let mut segments = value.split(delimiter);

    let first = segments.next()?.parse::<u16>().ok()?;
    let second = segments.next()?.parse::<u16>().ok()?;
    let third = segments.next()?.parse::<u16>().ok()?;

    if segments.next().is_some() {
        None
    } else {
        Some((first, second, third))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let res = split_string_3("1,2,3", ",");
        assert_eq!(res, Some((1, 2, 3)));
    }

    #[test]
    fn missing_one() {
        let res = split_string_3("1,2", ",");
        assert_eq!(res, None);
    }

    #[test]
    fn hole() {
        let res = split_string_3("1,,3", ",");
        assert_eq!(res, None);
    }

    #[test]
    fn not_number() {
        let res = split_string_3("A,2,3", ",");
        assert_eq!(res, None);
    }

    #[test]
    fn out_of_range_3() {
        let res = split_string_3("1,2,65536", ",");
        assert_eq!(res, None);
    }

    #[test]
    fn out_of_range_2() {
        let res = split_string_3("1,65536,3", ",");
        assert_eq!(res, None);
    }

    #[test]
    fn out_of_range_1() {
        let res = split_string_3("65536,2,3", ",");
        assert_eq!(res, None);
    }

    #[test]
    fn low_range_pass() {
        let res = split_string_3("0,0,0", ",");
        assert_eq!(res, Some((0, 0, 0)));
    }

    #[test]
    fn low_range_fail_1() {
        let res = split_string_3("-1,0,0", ",");
        assert_eq!(res, None);
    }

    #[test]
    fn low_range_fail_2() {
        let res = split_string_3("0,-1,0", ",");
        assert_eq!(res, None);
    }

    #[test]
    fn low_range_fail_3() {
        let res = split_string_3("0,0,-1", ",");
        assert_eq!(res, None);
    }

    #[test]
    fn high_range_pass() {
        let res = split_string_3("65535,65535,65535", ",");
        assert_eq!(res, Some((65535, 65535, 65535)));
    }

    #[test]
    fn too_many_numbers() {
        let res = split_string_3("1,2,3,4", ",");
        assert_eq!(res, None);
    }
}

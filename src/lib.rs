#[cfg(test)]
mod my_specific_tests {
    #[test]
    fn it_works() {
        let result = 2;
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod my_second_specific_tests {
    #[test]
    fn it_works() {
        let result = 2+2;
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod my_other_specific_tests {
    #[test]
    fn it_works() {
        let result = 2;
        assert_eq!(result, 4);
    }
}


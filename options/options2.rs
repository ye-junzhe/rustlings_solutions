// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(t) = optional_target {
            let word = t;
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // NOTE: If let solution

        // for _ in 0..range {
        //     if let Some(poped) = optional_integers.pop() {
        //         let integer = poped.unwrap();
        //         assert_eq!(integer, cursor);
        //         cursor -= 1;
        //     }
        // }

        // NOTE: While let solution
        let mut integer: Option<i8>;

        while let Some(poped) = optional_integers.pop() {
            if cursor > 0 {
                println!("cursor is: {}", cursor);
                let integer = poped;
                // println!("integer is: {}", integer);
                assert_eq!(integer.unwrap(), cursor);
                cursor -= 1;
            } else {
                integer = None;
            }
        }

        assert_eq!(cursor, 0);
    }
}

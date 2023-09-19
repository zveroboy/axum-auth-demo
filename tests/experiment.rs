#[cfg(target_family = "unix")]
mod rand {
    use std::fs::File;
    use std::io::Read;
    use std::sync::OnceLock;

    // delete
    trait Draw {
        fn draw(&self);
    }

    struct Button;
    impl Draw for Button {
        fn draw(&self) {
            print!("Draw Button");
        }
    }

    struct Table;
    impl Draw for Table {
        fn draw(&self) {
            print!("Draw Table");
        }
    }

    fn test() {
        let components: Vec<Box<dyn Draw>> = vec![Box::new(Button), Box::new(Table)];
        let s = Screen { components };
    }

    struct Screen {
        components: Vec<Box<dyn Draw>>,
    }
    // ^ delete

    static FILE: OnceLock<File> = OnceLock::new();

    fn open_urandom() -> File {
        File::open("/dev/urandom").expect("urandom is unavailable")
    }

    fn static_open_urandom() -> &'static File {
        FILE.get_or_init(|| {
            println!("static_open_urandom Called");
            File::open("/dev/urandom").expect("urandom is unavailable")
        })
    }

    pub fn create_rand_bytes(length: usize) -> Vec<u8> {
        let mut rnd = static_open_urandom();

        let mut buffer = vec![0u8; length];
        rnd.read_exact(&mut buffer).expect("read urandom failed");

        buffer
    }

    // #[async_trait]
    // tra

    pub struct RandomHex<'a> {
        urandom: &'a File,
        length: usize,
    }

    // trait TraitName {
    //     fn new(length: usize) -> Self;
    // }

    impl<'a> RandomHex<'a> {
        pub fn new(length: usize) -> Self {
            RandomHex {
                urandom: static_open_urandom(),
                length,
            }
        }
    }

    impl<'a> IntoIterator for RandomHex<'a> {
        type Item = <RandomHexIterator<'a> as Iterator>::Item;

        type IntoIter = RandomHexIterator<'a>;

        fn into_iter(self) -> Self::IntoIter {
            RandomHexIterator {
                urandom: self.urandom,
                length: self.length,
            }
        }
    }

    pub struct RandomHexIterator<'a> {
        urandom: &'a File,
        length: usize,
    }

    impl<'a> Iterator for RandomHexIterator<'a> {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            let mut buffer = vec![0u8; self.length];
            self.urandom
                .read_exact(&mut buffer)
                .expect("read urandom failed");

            //buffer
            //     .iter()
            //     .map(|byte| format!("{:02x?}", byte))
            //     .fold(Some(String::new()), |acc, hexes| {
            //         Some(acc? + &hexes)
            //     })

            let symbols = buffer
                .iter()
                .map(|byte| format!("{:02x?}", byte))
                .fold(String::new(), |acc, hexes| acc + &hexes);

            Some(symbols)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rand::*;

    #[tokio::test]
    async fn iter_test() {
        // #[derive(Debug, Copy, Clone)]
        // struct Demo(i32);

        // let numbers = [Demo(1), Demo(2), Demo(3), Demo(4), Demo(5)];

        // for mut n in numbers {
        //     n.0 += 1;
        //     println!("{:?}", n);
        // }

        // println!("{:?}", numbers);
        // assert_eq!(1, numbers[0].0);

        // println!("{:?}", symbols);

        // let mut numbers_range: RangeInclusive<char> = '0'..='9';

        // println!("{}", numbers_range.len());

        // let mut range: RangeInclusive<i16> = 0..=5;

        // assert_eq!(5, range.len());

        // let arr = create_rand_bytes(8);
        // // arr[1] = 254;
        // // let formatted = format!("{:02x?}", arr[1]);
        // // let hexes = formatted.as_bytes();

        // // println!("{:?} {:?}", formatted, hexes);
        // let sym: String = arr
        //     .iter()
        //     .map(|byte| format!("{:02x?}", byte))
        //     .fold(String::new(), |acc, hexes| {
        //         acc + &hexes
        //         // acc.push_str(&hexes);
        //         // acc
        //     });

        // println!("{}", sym);

        let rnd = RandomHex::new(16);

        for p in rnd.into_iter().take(3) {
            println!("{p}");
        }

        // for p in rnd.iter().take(3) {
        //     println!("{p}");
        // }

        let rnd = RandomHex::new(16);

        let mut i = 0;
        for p in rnd {
            if i > 2 {
                break;
            }
            println!("{p}");
            i += 1;
        }
    }
}

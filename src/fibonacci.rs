pub mod fibonacci {
    pub fn pri(order: u128) {
        let mut i: u128 = 1;
        while i <= order {
            print!("{} ",f(i));
            i = i + 1;
        }
    }
    fn f(x: u128) -> u128 {
        if x == 0 {
            return 0;
        }
        if x == 1 {
            return 1;
        }
        f(x - 1) + f(x - 2)
    }
}

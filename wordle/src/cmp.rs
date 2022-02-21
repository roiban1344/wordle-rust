//comparator

macro_rules! define{
    ($name: ident, $operator: tt) => {
        pub struct $name<T: PartialOrd> {
            current: Option<T>,
        }

        impl<T: PartialOrd> $name<T> {
            pub fn new() -> $name<T> {
                $name { current: None }
            }

            pub fn apply(&mut self, val: T) -> bool {
                if let Some(current) = &self.current {
                    if val $operator *current {
                        self.current = Some(val);
                        true
                    } else {
                        false
                    }
                } else {
                    self.current = Some(val);
                    true
                }
            }
        }
    };
}

define!(Chmax, >=);
define!(Chmin, <=);

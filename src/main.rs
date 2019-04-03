extern crate rand;
use rand::Rng;

use std::collections::VecDeque;



trait RadixSort<T> {
    fn radix_sort(&mut self, usize);
}

trait Sort {
    const MAX: Self;

    fn as_usize(self) -> usize;
}

macro_rules! impl_RadixSort {
    ($($T:ty; $N:expr),*) => {$(
        impl<D> RadixSort<[$T; $N]> for D
            where for <'a> &'a mut D: IntoIterator<Item = &'a mut [$T; $N]> {
            fn radix_sort(&mut self, capacity: usize) {
                let mut radix = vec![VecDeque::<[$T; $N]>::with_capacity(capacity); <$T as Sort>::as_usize(<$T as Sort>::MAX) + 1];

                for i in 0..$N {
                    for e in self.into_iter() {
                        radix[<$T as Sort>::as_usize(e[i].clone())].push_back(e.clone());
                    }

                    let mut iter = self.into_iter();
                    for e in radix.iter_mut() {
                        for c in e.iter_mut() {
                            if let Some(item) = iter.next() {
                                *item = c.clone();
                            }
                        }

                        e.clear();
                    }
                }
            }
        }
    )*}
}


impl Sort for u16 {
    const MAX: Self = 65_535;

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl_RadixSort!(u16; 2);


fn main() {
    let mut rng = rand::thread_rng();
    let mut data: Vec<_> = (0..1_048_576).map(|_| rng.gen_range(u32::min_value(), u32::max_value() - 1)).collect();

    // println!("data = {:?}", data);
    data = unsafe {
        use std::mem::transmute;

        let mut data = transmute::<Vec<u32>, Vec<[u16; 2]>>(data);
        data.radix_sort(1);
        transmute::<Vec<[u16; 2]>, Vec<u32>>(data)
    };
    // println!("data = {:?}", data);
}
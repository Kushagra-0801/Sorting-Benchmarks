struct LeonardoNumber {
    b: u32,
    c: u32,
}

impl Default for LeonardoNumber {
    fn default() -> Self {
        Self { b: 1, c: 1 }
    }
}

impl LeonardoNumber {
    pub fn gap(&self) -> u32 {
        self.b - self.c
    }

    pub fn get(&self) -> u32 {
        self.b
    }

    pub fn companion(&self) -> u32 {
        self.c
    }

    pub fn next(&mut self) {
        let s = self.b;
        self.b = self.b + self.c + 1;
        self.c = s;
    }

    pub fn prev(&mut self) {
        let s = self.c;
        self.c = self.b - self.c - 1;
        self.b = s;
    }
}

fn sift<T: Ord + Copy>(arr: &mut [T], b: &mut LeonardoNumber) {
    todo!()
}

fn trinkle<T: Ord + Copy>(arr: &mut [T], p: u64, b: &mut LeonardoNumber) {
    todo!()
}

pub fn smoothsort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len == 0 {
        return;
    }
    let mut p = 1;
    let mut b = LeonardoNumber::default();
    for q in 0..len {
        if p % 8 == 3 {
            sift(&mut arr[..q - 1], &mut b);
            b.next();
            b.next();
            p >>= 2;
        } else if p % 4 == 1 {
            if (q + b.companion() as usize) < len {
                sift(&mut arr[..q - 1], &mut b);
            } else {
                trinkle(&mut arr[..q - 1], p, &mut b);
            }
            p <<= 1;
            loop {
                b.prev();
                if b.get() <= 1 {
                    break;
                }
                p <<= 1;
            }
        }
        p += 1;
    }
    trinkle(&mut arr[..len - 1], p, &mut b);
}

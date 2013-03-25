fn padleft(content: ~str, len: uint, fill: ~str) -> ~str {
    let mut ret = content;
    while str::len(ret) < len {
        ret = fill + ret;
    }
    return ret;
}

// Section 1.3.2, Exercise 10
struct mat<T> { rows : uint, cols : uint, data : ~[T] }

impl<T : Copy> mat<T> {
    fn get(&self, i:uint, j:uint) -> T { return self.data[self.cols*i + j]; }
    fn set(&mut self, i:uint, j:uint, x:T) { self.data[self.cols*i + j] = x; }
}

impl<T : Copy> Index<(uint,uint), T> for mat<T> {
    fn index(&self, (i,j):(uint,uint)) -> T { return self.get(i,j); }
}

fn find_cell_size<T : Copy + ToStr>(m : &mat<T>) -> uint {
    // return 4
    let mut max_size = 1;
    for uint::range(0, m.rows) |i| {
        for uint::range(0, m.cols) |j| {
            let rendered = m.get(i,j).to_str();
            max_size = cmp::max(max_size, str::len(rendered));
        }
    }
    max_size
}

impl<T : Copy + ToStr> ToStr for mat<T> {
    fn to_str(&self) -> ~str {
        let cellsize = find_cell_size(self);

        let mut ret = ~"[ ";
        for uint::range(0, self.rows) |i| {
            if i > 0 {
                ret += "| ";
            }
            for uint::range(0, self.cols) |j| {
                let rendered = self.get(i,j).to_str();
                ret += padleft(rendered, cellsize, ~" ");
                if j+1 < self.cols {
                    ret += ", ";
                } else if i+1 < self.rows {
                    ret += " |\n";
                } else {
                    ret += " ]"
                }
            }
        }
        ret
    }
}

fn build_matrix_data<T : Copy>(rows: uint, cols: uint,
                               f : &fn(uint, uint) -> T) -> ~[T] {
    let mut ret = ~[];
    for uint::range(0, rows) |i| {
        for uint::range(0, cols) |j| { ret.push(f(i,j)); }
    }
    ret
}

fn build_matrix<T : Copy>(rows: uint, cols: uint,
                          f : &fn(uint, uint) -> T) -> mat<T> {
    let mut ret = mat { rows: rows, cols: cols, data: ~[] };
    ret.data = build_matrix_data(rows, cols, f);
    ret
}


fn find_a_saddle_point<T : Copy + Ord>(m:&mat<T>) -> Option<(uint, uint)> {
    let mut row_mins = ~[];
    let mut col_maxs = ~[];
    for uint::range(0, m.rows) |i| {
        row_mins.push(m.get(i, 0));
    }
    for uint::range(0, m.cols) |j| {
        col_maxs.push(m.get(0, j));
    }
    for uint::range(0, m.rows) |i| {
        for uint::range(0, m.cols) |j| {
            let x = m.get(i, j);
            let r = row_mins[i];
            let c = col_maxs[j];
            if x < r { row_mins[i] = x; }
            if x > c { col_maxs[j] = x; }
        }
    }
    for uint::range(0, m.rows) |i| {
        for uint::range(0, m.cols) |j| {
            let x = m.get(i,j);
            let r = row_mins[i];
            let c = col_maxs[j];
            if !(r < x) && !(c > x) { return Some((i,j)); }
        }
    }
    None
}

// find some saddle point, if any, in a 9*8 matrix.
fn ex10() {
    // let mat = build_matrix(9, 8, |i,j| { 1000 + 8*(i+1) + (j+1) } );
    let mat = build_matrix::<int>(9, 8, |i,j| {
        (i as int)*(2*((i%2) as int)-1)
            + (j as int)*(2*((j%2) as int)-1) } );
    let ans = find_a_saddle_point(&mat);
    io::println(fmt!("%s\n%s:%?", mat.to_str(), "ans", ans));
}

fn filled_vec<T:Copy>(len:uint, x:T) -> ~[T] {
    // let mut ret : ~[T] = ~[x, ..len];

    // There has *got* to be a better way.
    let mut v = do vec::build_sized(len) |push| {
        for uint::range(0, 256) |_i| { push(x); }
    };
    v
}

// Section 1.3.2, Exercise 13
fn count_char_frequency<R:io::Reader>(r:R) -> ~[uint] {
    let mut vlen = 256;
    let mut ret = filled_vec(vlen, 0);
    // let mut ret = [0, ..256];
    let mut i = r.read_byte();
    while i >= 0 {
        // if (i as char) == '*' { break }
        ret[i] += 1;
        i = r.read_byte();
    }
    ret
}


fn char_range(lo: char, hi: char, it: &fn(char) -> bool) {
    fn range_step(start: char, stop: char, step: int, it: &fn(char) -> bool) {
        let mut i = start;
        if step >= 0 {
            while i < stop {
                if !it(i) { break }
                i = ((i as int) + step) as char;
            }
        }
        else {
            while i > stop {
                if !it(i) { break }
                i = ((i as int) + step) as char;
            }
        }
    }
    range_step(lo, hi, 1, it);
}

fn print_char_frequency<R:io::Reader>(r:R) {
    let freq = count_char_frequency(r);
    fn print_one(c:char, freq:&[uint]) {
        let f = freq[c as uint];
        if f != 0 { io::println(fmt!("%c %u", c, f)); }
    }

    for char_range('0','9') |c| { print_one(c, freq); }
    for char_range('a','z') |c| { print_one(c, freq); }
    for char_range('A','Z') |c| { print_one(c, freq); }
}


// character frequencies in input stream.
fn ex13() {
    // Point-free style
    let input = "abcaabeASfkjanfakjcnkajwqdkjfwqekjAA";
    io::with_str_reader(input, print_char_frequency);
    // Explicit style
    // do io::with_str_reader(input) |r| { print_char_frequency(r); }
    io::println("");

    let path : Path = path::GenericPath::from_str("/Users/pnkfelix/Documents/Books/hugo_les_miserables.txt");
    match io::file_reader(&path) {
        Ok(r) => print_char_frequency(r),
        Err(why) => io::println(fmt!("file read failed: %s", why))
    }
}

fn generate_magic_square(n:uint) -> mat<uint> {
    let mut m = build_matrix(n, n, |_i,_j| 0);
    let mut b = build_matrix(n, n, |_i,_j| false);
    let mut i = n/2+1;
    let mut j = n/2;
    let mut c = 0;
    let mut a = 1;
    while (c < n*n) {
        m.set(i,j, a);
        b.set(i,j, true);
        c += 1;
        a += 1;
        if !b.get((i+1) % n, (j+1) % n) {
            i = (i + 1) % n;
            j = (j + 1) % n;
        } else {
            i = (i+2) % n
        }
    }
    m
}

// generate magic square of order 23. (sample from book is of order 7.)
fn ex21() {
    // let n = 7;
    let n = 23;
    let m = generate_magic_square(n);
    io::println(fmt!("Magic square of order %u:\n%s\n", n, m.to_str()));
}

fn main() {
    // ex10();
    // ex13();
    ex21();
}

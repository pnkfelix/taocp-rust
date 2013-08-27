use std::io;
use std::os;
use std::path::{Path, GenericPath};
use std::hashmap::HashMap;
use std::str;

struct Key ( [char, ..5] );
//#[deriving(IterBytes, Eq)]
//struct Key ( ~[char] );

struct Entry {
    word: Key, class: Class, counts: ~[uint]
}
type WordsMap = HashMap<Key, Entry>;
// struct WordsData { m: WordsMap }
struct WordsData { m: ~[Entry] }

// Taken from SGB gb_words.w
//
// Every word in words.dat has been classifed as 'common', 'advanced',
// or 'unusual'.  Each word has also been assigned seven frequency
// counts c_1,...c_7.  These counts show how often the word has
// occurred in different publication contexts:
//   -- c_1 times in the American Hertiage Intermediate Corpus of elementary school material;
//   -- c_2 tiems in the Brown Corpus of reading material from America;
//   -- c_3 times in the Lancaster-Oslo/Bergen Corpus of reading material from Brtain;
//   -- c_4 times in the Melbourne-Surrey Corpus of newspaper material from Australia;
//   -- c_5 times in the Revised Standard Version of the Bible;
//   -- c_6 times in The TeXbook and The METAFONTbook by D. E. Knuth;
//   -- c_7 times in Concrete Mathematic by Graham, Knuth, and Patashnik

enum Class { common, advanced, unusual }

static max_c : &'static [uint]
             = &'static [15194, 3560, 4467, 460, 6976, 756, 362 ];

static default_wt_vector : &'static [uint]
                         = &'static [ 100, 10, 4, 2, 2, 1, 1, 1, 1 ];


struct counts {
    c: [uint, ..7]
}

struct wt_vector {
    a: uint,
    b: uint,
    w: [uint, ..7]
}

// The weight of each word is computed from wt_vector by using the formula
//                          { a, if the word is 'common'
//   c_1w_1 + ... + c_7w& + { b, if the word is 'advanced'
//                          { 0, if the word is 'unusual'
impl wt_vector {
    fn weight(&self, e:&Entry) -> uint {
        (self.w[0] * e.counts[0]
         + self.w[1] * e.counts[1]
         + self.w[2] * e.counts[2]
         + self.w[3] * e.counts[3]
         + self.w[4] * e.counts[4]
         + self.w[5] * e.counts[5]
         + self.w[6] * e.counts[6]
         + match e.class { common => self.a, advanced => self.b, unusual => 0 })
    }
}

fn words_from(r:@io::Reader) -> WordsData {
    debug!("starting words_from");
    let mut d = WordsData { m: ~[] };
    r.each_line(|line| {
            debug!("words_from line: %s", line);
            if line.char_at(0) == '*' { // comments have * in first column
                    true
            } else {
                let word = [line.char_at(0),
                            line.char_at(1),
                            line.char_at(2),
                            line.char_at(3),
                            line.char_at(4) ];
                let rest = line.as_bytes().slice_from(5);

                debug!("words_from lines sliced: %?", rest);
                // let mut wt;
                let class = if rest.len() == 0 {
                    unusual
                } else {
                    match rest[0] as char {
                        '*' => common,
                        '+' => advanced,
                        ' ' => unusual,
                        _   => fail!("unrecognized input")
                    }
                };
                let mut counts = ~[0,0,0, 0,0,0,0];

                if rest.len() > 0 {
                    let rest = rest.slice_from(1);
                    let mut accum = 0;
                    let mut i = 0;
                    for c in rest.iter() {
                        let c = *c as char;
                        if c == ',' {
                            debug!("setting counts[%u] to accum: %u", i, accum);
                            counts[i] = accum;
                            accum = 0;
                            i += 1;
                        } else if '0' <= c && c <= '9' {
                            accum = accum * 10 + (c as uint - '0' as uint);
                        }
                    }
                }
                let e = Entry {
                    word: Key(word),
                    class: class,
                    counts: counts
                };
                d.m.push(e);
                true
            }
        });
    d
}

fn read_words(words_dat:Path) -> WordsData {
    match io::file_reader(&words_dat) {
        Ok(r) => words_from(r),
        Err(s) => fail!("read_words failed %s", s)
    }
}

fn print_words(data: &WordsData) {
    for v in data.m.iter() {
        let s = str::from_chars(*v.word);
        let c = match v.class {
            common   => "  common",
            unusual  => " unusual",
            advanced => "advanced"
        };
        println(fmt!("key: %s %s %?", s, c, v.counts));
    }
}

type Graph = GraphImpl;

struct GraphImpl;

impl WordsData {
    fn words(&self, n: uint, wt_vector: Option<~[uint]>, wt_threshold: uint, seed: uint) -> Graph {
        // Taken from SGB gb_words.w
        //
        // All words will be sorted by weight, where weights are
        // computed from a table pointed to by wt_vector (if None,
        // then default weights are used).  The first vertex of the
        // graph will be the word of largest weight, the second vertex
        // will have second-largest weight, and so on.  Words of equal
        // weight will appear in pseudo-random order, as determined by
        // the value of seed in a system-independent fashion.  The
        // first n words in order of decreasing weight are chosen to
        // be the vertices of the graph.  However, if fewer than n
        // words have weight >= wt_threshold, the graph will contain
        // only the words that qualify.  In such cases the graph will
        // have fewer than n vertices -- possibly none at all.
        //
        // Exception: The special case n == 0 is equivalent to the
        //case when n has been set to the highest possible value.  It
        //casues all qualifying words to appear.
        let _ = (n,wt_vector, wt_threshold, seed);
        fail!("unimplemented");
    }
}

fn main() {
    debug!("Hello World");
    // let p = 
    let args = os::args();
    let p = if args.len() > 1 {
        os::args()[1]
    } else {
        ~"/Users/pnkfelix/Dev/Knuth/sgb/words.dat"
    };
    let d = read_words(GenericPath::from_str(p));
    println(fmt!("Hello Words"));
    let _ = d;
    print_words(&d);
}

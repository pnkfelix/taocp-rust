use std::char;
use std::fmt;
use std::fmt::Show;
use std::str;

#[deriving(Clone)]
#[repr(u8)]
enum Facing {
    FaceDown = 1u8,
    FaceUp   = 0u8,
}

#[deriving(Clone)]
#[repr(u8)]
enum Suit {
    Clubs    = 1u8,
    Diamonds = 2u8,
    Hearts   = 3u8,
    Spades   = 4u8,
}

#[deriving(Clone)]
#[repr(u8)]
enum Rank {
    Ace  =  1u8, R2  =  2u8, R3   =  3u8, R4    =  4u8,
    R5   =  5u8, R6  =  6u8, R7   =  7u8, R8    =  8u8,
    R9   =  9u8, R10 = 10u8, Jack = 11u8, Queen = 12u8,
    King = 13u8,
}

/// Keeping fidelity with Knuth's presentation, Cards carry a "title"
/// even though it is redundant since it can be computed from the
/// fields.  Making it a struct instead of a type alias makes it
/// easier to derive Clone for Card itself, and may have other
/// as-yet-undetermined benefits.
struct CardTitle {
    bytes: [u8, ..5],
}

impl CardTitle {
    fn to_string(&self) -> String {
        str::from_utf8(self.bytes).unwrap().into_string()
    }
}

impl Clone for CardTitle {
    fn clone(&self) -> CardTitle {
        CardTitle {
            bytes: [self.bytes[0], self.bytes[1],
                    self.bytes[2], self.bytes[3],
                    self.bytes[4]],
        }
    }
}

trait ToCardTitle { fn to_card_title(self) -> CardTitle; }

impl ToCardTitle for [u8, ..5] { fn to_card_title(self) -> CardTitle {
    CardTitle { bytes: self } }
}

impl<'a> ToCardTitle for &'a [u8, ..5] { fn to_card_title(self) -> CardTitle {
    CardTitle { bytes: *self } }
}

impl<'a> ToCardTitle for &'a [u8] { fn to_card_title(self) -> CardTitle {
    let len = self.len();
    assert!(len <= 5);
    let mut bytes = [0u8, ..5];
    for i in range(0u, len) {
        bytes[4 - i] = self[len - 1 - i]
    }
    for i in range(len, 5) {
        bytes[4 - i] = b' ';
    }
    CardTitle { bytes: [self[0], self[1], self[2], self[3], self[4]] } }
}

impl<'a> ToCardTitle for ||:'a -> CardTitle { fn to_card_title(self) -> CardTitle {
    self() }
}

#[deriving(Clone)]
struct Card {
    next: u16,
    tag: Facing,
    suit: Suit,
    rank: Rank,
    title: CardTitle,
}

/// The lifetime parameter `'a` is implicitly relating this reference to some CardStore.
#[deriving(Clone)]
struct CardRef {
    // Invariant: the referent has a 0 `next` field.
    index: u16,
}

struct CurrentCardRef<'a> {
    store: &'a mut CardStore,
    cardref: CardRef,
}

/// The lifetime parameter `'a` is implicitly relating this reference to some CardStore.
#[deriving(Clone)]
struct Pile {
    top: u16,
}

struct CurrentPile<'a> {
    store: &'a mut CardStore,
    pile: Pile,
}

impl Suit {
    pub fn unicode_char(&self) -> char {
        match *self {
            Spades   => '\u2660',
            Hearts   => '\u2661',
            Diamonds => '\u2662',
            Clubs    => '\u2663',
        }
    }
    pub fn ace_char(&self) -> char {
        match *self {
            Spades   => '\U0001F0A1',
            Hearts   => '\U0001F0B1',
            Diamonds => '\U0001F0C1',
            Clubs    => '\U0001F0D1',
        }
    }
    pub fn ascii_char(&self) -> char {
        match *self {
            Spades   => 'S',
            Hearts   => 'H',
            Diamonds => 'D',
            Clubs    => 'C',
        }
    }
}

impl fmt::Show for Suit {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{:c}", self.unicode_char())
    }
}

impl fmt::Show for Rank {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{:s}", self.ascii_string())
    }
}

impl Rank {
    pub fn ascii_string(&self) -> String {
        match *self {
            Ace   => "A".to_string(),
            King  => "K".to_string(),
            Queen => "Q".to_string(),
            Jack  => "J".to_string(),
            _     => (*self as u32).to_string()
        }
    }
    pub fn offset_from_ace(&self) -> u8 {
        (*self as u8) - 1
    }
}

enum FaceFormat {
    Compact,
    Legible,
    Ascii,
}

impl Card {
    pub fn face_unicode_char(&self) -> char {
        char::from_u32(self.suit.ace_char() as u32 + self.rank.offset_from_ace() as u32).unwrap()
    }
    pub fn face_string(&self, ff: FaceFormat) -> String {
        match ff {
            Compact => format!("{}", self.face_unicode_char()),
            Legible => format!("{}{}", self.rank.ascii_string(), self.suit.unicode_char()),
            Ascii   => format!("{}{}", self.rank.ascii_string(), self.suit.ascii_char())
        }
    }
    pub fn back_ascii_string(&self) -> String {
        format!("[]")
    }
    pub fn back_string(&self, ff: FaceFormat) -> String {
        match ff {
            Compact => 
        }
    }
    pub fn unicode_string(&self) -> String {
        match self.tag {
            FaceUp => self.face_unicode_string(),
            FaceDown => self.back_ascii_string(),
        }
    }
    pub fn ascii_string(&self) -> String {
        match self.tag {
            FaceUp => self.face_ascii_string(),
            FaceDown => self.back_ascii_string(),
        }
    }
}

impl fmt::Show for Card {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        // write!(w, "{:c}", self.face_unicode_char())
        // write!(w, "{:s}", self.face_unicode_string())
        write!(w, "{:s}", self.unicode_string())
    }
}

impl Pile {
    fn top_card<'a>(&self, store: &'a CardStore) -> &'a Card {
        store.cards.get(self.top as uint)
    }

    fn write_all(&self,
                 store: &CardStore,
                 w: &mut fmt::Formatter,
                 on_card: |&Card, &mut fmt::Formatter| -> fmt::Result)
                 -> fmt::Result {
        use std::fmt::Show;
        let mut cursor = self.top as uint;
        while cursor != 0 {
            let card = store.cards.get(cursor);
            try!(on_card(card, w));
            cursor = card.next as uint;
            if cursor != 0 { try!(write!(w, ",")) }
        }
        Ok(())
    }

    fn count(&self, store: &CardStore) -> u16 {
        let mut count = 0u16;
        let mut cursor = self.top as uint;
        while cursor != 0 {
            let card = store.cards.get(cursor);
            count += 1;
            cursor = card.next as uint;
        }
        count
    }

    /// Takes newcard by value since this operation "consumes" it
    /// (namely by making it part of this pile and thus breaking its
    /// invariant that it represents a single card).
    fn place_face_up(&mut self, store: &mut CardStore, newcard: CardRef) {
        let card = store.cards.get_mut(newcard.index as uint);
        card.next = self.top;
        self.top = newcard.index;
    }

    // Exercse 2.1-3: removes the top card of the pile (if the pile is not empty)
    // and provides the address of this card.
    fn pop_v1(&mut self, store: &mut CardStore) -> Option<CardRef> {
        if self.top != 0 {
            let top_orig = self.top;
            let card = store.cards.get_mut(top_orig as uint);
            self.top = card.next;
            card.next = 0u16;
            Some(CardRef { index: top_orig })
        } else {
            None
        }
    }

    fn pop(&mut self, store: &mut CardStore) -> Option<CardRef> {
        if self.top != 0 {
            let top_orig = self.top;
            let card = store.cards.get_mut(top_orig as uint);
            self.top = card.next;
            card.next = 0u16;
            Some(CardRef { index: top_orig })
        } else {
            None
        }
    }
}

impl<'a> CurrentPile<'a> {
    fn place_face_up(&mut self, newcard: CardRef) { self.pile.place_face_up(self.store, newcard) }
    fn count(&self) -> u16 { self.pile.count(self.store) }
    fn top_card(&'a self) -> &'a Card { self.pile.top_card(self.store) }

    // The reason pop returns CardRef and not CurrentCardRef is
    // because I still want to be able to manipulate the resulting
    // Pile, and so I cannot move the borrowed CardStore from the
    // current Pile to a hypothetical new CurrentCardRef.  (And
    // copying it is not an option since it is an `&mut` reference.)
    //
    // N.B.: `&'a mut self` and `&mut self` are very different, even
    // when `'a` does not occur elsewhere in the method signature, if
    // `'a` is bound by the trait itself.  It forces `self` to be
    // borrowed mutably for the duration of the lifetime `'a`, and
    // consumes the mutable borrow itself (right?), *even when* the
    // returned value has no outstanding references to the lifetime.
    // Doing that basically neuters the reference that the caller
    // still has in their hands.  (I am sure this is useful for some
    // patterns where one is doing interesting things with encoding
    // certain constraints via lifetimes.  But it also invalidates the generality of certain
    // rules-of-thumb that I have been thinking about.)

    // fn pop(&'a mut self) -> Option<CardRef> { self.pile.pop(self.store) }
    fn pop(&mut self) -> Option<CardRef> { self.pile.pop(self.store) }

    fn write_all_titles(&self, w: &mut fmt::Formatter) -> fmt::Result {
        self.pile.write_all(self.store, w, |card,w|write!(w, "{}\n", card.title.to_string()))
    }
    fn write_all_ascii(&self, w: &mut fmt::Formatter) -> fmt::Result {
        self.pile.write_all(self.store, w, |card,w|write!(w, "{:s}", card.ascii_string()))
    }
    fn write_all_unicode(&self, w: &mut fmt::Formatter) -> fmt::Result {
        self.pile.write_all(self.store, w, |card,w|write!(w, "{:s}", card.unicode_string()))
    }
    fn write_all_compact(&self, w: &mut fmt::Formatter) -> fmt::Result {
        self.pile.write_all(self.store, w, |card,w|write!(w, "{:s}", card.unicode_char()))
    }
}

impl CardRef {
    fn title_string(&self, store: &CardStore) -> String {
        store.cards.get(self.index as uint).title.to_string()
    }
}


impl<'a> fmt::Show for CurrentCardRef<'a> {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{:s}", self.cardref.title_string(self.store))
    }
}

impl<'a> fmt::Show for CurrentPile<'a> {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        // let bytes = self.store.cards.get(self.top as uint).title.bytes;
        // write!(w, "{:s}", str::from_utf8(bytes).unwrap())
        self.pile.write_all(self.store, w, |card,w|card.fmt(w))
    }
}

struct CardStore {
    cards: Vec<Card>,
}

impl CardStore {
    /// This method is really just meant for constructing hypothetical
    /// card stores of arbitrary size with mostly garbage data.
    fn prefilled(size: u16) -> Box<CardStore> {
        box CardStore { cards: Vec::from_elem(size as uint, VOID_CARD) }
    }
    /// This method is really just meant for constructing hypothetical
    /// card stores of arbitrary size with mostly garbage data.
    fn replace_at<'a>(&'a mut self, idx: u16, card: Card) -> Pile {
        *self.cards.get_mut(idx as uint) = card;
        Pile { top: idx }
    }

    fn new() -> CardStore {
        CardStore { cards: Vec::new() }
    }
    fn current_pile_at<'a>(&'a mut self, idx: u16) -> CurrentPile<'a> {
        assert!((idx as uint) < self.cards.len());
        CurrentPile { store: self, pile: Pile { top: idx } }
    }
}

impl Card {
    fn new<FC:ToCardTitle>(tag: Facing, suit: Suit, rank: Rank, to_title: FC) -> Card {
        Card { next: 0, tag: tag, suit: suit, rank: rank, title: to_title.to_card_title() }
    }
    fn atop(self, next: u16) -> Card {
        Card { next: next, ..self }
    }
}

static VOID_CARD : Card = Card {
    next: 0, tag: FaceUp, suit: Clubs, rank: Ace, title: CardTitle { bytes: [0,0,0,0,0] },
};

fn main() {
    let mut card_store = CardStore::prefilled(400);

    // #6268 : The solution to the "nested method call" problem may
    // also be applicable here, where it may be nice to be able to
    // mutably borrow the card_store for a series of method calls that
    // return "reservations for future borrows": i.e.  borrows that
    // one cannot act on in this scope, but can act on in some other
    // scope (perhaps some parent scope is what would make sense).
    //
    // But for now I'll just have two separate calls, discarding the results
    // here and remaking them later.

    card_store.replace_at(100, Card::new(FaceDown,  Clubs, R10, b" 10 C"));
    card_store.replace_at(386, Card::new(FaceUp,   Spades,  R3, b"  3 S").atop(100));
    card_store.replace_at(242, Card::new(FaceUp, Diamonds,  R2, b"  2 D").atop(386));
    println!("Hello World; Card size: {} bytes", std::mem::size_of::<Card>());
    let c10_str = {
        let c10 = card_store.current_pile_at(100);
        format!("{}", c10)
    };
    let s03_str = {
        let s03 = card_store.current_pile_at(386);
        format!("{}", s03)
    };
    let d02 = card_store.current_pile_at(242);
    let d02_str = format!("{}", d02);
    println!("c10: `{}' s03: `{}' d02: `{}'", c10_str, s03_str, d02_str);
    println!("d02 count: {}", d02.count());

    let mut pile = d02;
    println!("pile pre-pop:  {} ", pile);
    let old_top = pile.pop().unwrap();
    let old_top_str = old_top.title_string(pile.store);
    println!("pile post-pop: {} top: {}", pile, old_top_str);
}

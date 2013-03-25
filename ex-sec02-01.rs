enum card_tag { facedown, faceup }
enum card_suit { clubs, diamonds, hearts, spades }

impl ToStr for card_suit {
    fn to_str(&self) -> ~str {
        match self {
            // U+2660–2667
            &clubs    => ~"\u2663",
            &diamonds => ~"\u2662",
            &hearts   => ~"\u2661",
            &spades   => ~"\u2660"
        }
    }
}

struct card {
    tag: card_tag,
    suit: card_suit,
    rank: u8, // 1..13
    next: Option<~card>,
    title: [char * 5]
        // contents of title field are derivable from those above it,
        // but I am more interested in trying to be faithful to
        // Knuth's presentation here.
}

impl card {
    fn rank_to_str(&self) -> ~str { rank_to_str(self.rank) }
}

impl ToStr for card {
    fn to_str(&self) -> ~str {
        let content = self.rank_to_str() + self.suit.to_str();
        match self.tag {
            faceup => content,
            facedown => ~"(" + content + ~")"
        }
    }
}

fn rank_to_str(r:u8) -> ~str {
    match r {
        1     => ~"A",
        2..10 => r.to_str(),
        11    => ~"J",
        12    => ~"Q",
        13    => ~"K",
        _     => fail!()
    }
}

struct pile { cards: Option<~card> }

impl pile {
    fn place_top_faceup(&mut self, newcard: ~card) {
        self.cards = Some(place_faceup(newcard, copy self.cards));
    }
    fn pop_top(&mut self) -> ~card {
        // TODO: How do I eliminate the (3) copies here and below?
        let mut cards = copy self.cards;
        match cards {
            Some(ref mut card) => {
                let mut next = None;
                next <-> card.next;
                self.cards = copy next;
                copy *card }
            None => fail!()
        }
    }
    fn place_bot_facedown(&mut self, newcard: ~card) {
        let mut newcard = newcard;
        newcard.tag = facedown;
        self.cards = match copy self.cards {
            None => Some(newcard),
            Some(cards) => { let mut cards = cards; Some(place_bot(newcard, cards)) }
        }
    }
}

fn place_faceup(newcard: ~card, pile: Option<~card>) -> ~card {
    let mut newcard = newcard;
    newcard.next = pile;
    newcard.tag = faceup;
    newcard
}

fn place_bot(newcard: ~card, cards: ~card) -> ~card {
    let mut cards = cards;
    // TODO: How do I eliminate the copy here?
    match copy cards.next {
        None => { cards.next = Some(newcard); }
        Some(next) => cards.next = Some(place_bot(newcard, next))
    }
    cards
}

impl ToStr for pile {
    fn to_str(&self) -> ~str {
        let mut ret = ~"";
        let mut printed = false;
        let mut cards = &self.cards;
        loop {
            match cards {
                &None => break,
                &Some(ref card) => {
                    if printed { ret += "," }
                    ret += card.to_str();
                    printed = true;
                    cards = &card.next;
                }
            }
        }
        ret
    }
}

/*
// version 1:
ex-sec02-01.rs:29:43: 29:52 error: moving out of immutable field
ex-sec02-01.rs:29             Some(card) => { c += 1; pile = card.next },
                                                             ^~~~~~~~~
fn count_cards(pile: Option<~card>) -> uint {
    let mut c = 0;
    let mut pile = pile;
    loop {
        match pile {
            Some(card) => { c += 1; pile = card.next },
            None => break
        }
    }
    return c;
}
*/

/*
// version 2:
ex-sec02-01.rs:47:12: 47:25 error: mismatched types: expected `&core::option::Option<~card>` but found enum or structure
ex-sec02-01.rs:47             Some(card) => { c += 1; pile = &card.next },
                              ^~~~~~~~~~~~~
fn count_cards(pile: Option<~card>) -> uint {
    let mut c = 0;
    let mut pile = &pile;
    loop {
        match pile {
            Some(card) => { c += 1; pile = &card.next },
            None => break
        }
    }
    return c;
}
*/

/*
// version 3:
ex-sec02-01.rs:81:44: 81:48 error: illegal borrow: borrowed value does not live long enough
ex-sec02-01.rs:81             Some(card) => { c += 1; pile = &card.next },
                                                              ^~~~
ex-sec02-01.rs:76:44: 86:1 note: borrowed pointer must be valid for the block at 76:44...
ex-sec02-01.rs:76 fn count_cards(pile: Option<~card>) -> uint {
ex-sec02-01.rs:77     let mut c = 0;
ex-sec02-01.rs:78     let mut pile = &pile;
ex-sec02-01.rs:79     loop {
ex-sec02-01.rs:80         match *pile {
ex-sec02-01.rs:81             Some(card) => { c += 1; pile = &card.next },
                  ...
ex-sec02-01.rs:80:8: 83:9 note: ...but borrowed value is only valid for the match at 80:8
ex-sec02-01.rs:80         match *pile {
ex-sec02-01.rs:81             Some(card) => { c += 1; pile = &card.next },
ex-sec02-01.rs:82             None => break
ex-sec02-01.rs:83         }
ex-sec02-01.rs:80:14: 80:19 error: moving out of dereference of immutable & pointer
ex-sec02-01.rs:80         match *pile {
                                ^~~~~
fn count_cards(pile: Option<~card>) -> uint {
    let mut c = 0;
    let mut pile = &pile;
    loop {
        match *pile {
            Some(card) => { c += 1; pile = &card.next },
            None => break
        }
    }
    return c;
}
*/

/*
// version 4:
ex-sec02-01.rs:99:18: 99:23 error: by-move pattern bindings may not occur behind @ or & bindings
ex-sec02-01.rs:99             &Some(card) => { c += 1; pile = &card.next },
                                    ^~~~~
fn count_cards(pile: Option<~card>) -> uint {
    let mut c = 0;
    let mut pile = &pile;
    loop {
        match pile {
            &Some(card) => { c += 1; pile = &card.next },
            &None => break
        }
    }
    return c;
}
*/

// version 5:
fn count_cards(pile: Option<~card>) -> uint {
    let mut c = 0;
    let mut pile = &pile;
    loop {
        match pile {
            &Some(ref card) => { c += 1; pile = &card.next },
            &None => break
        }
    }
    return c;
}

// version 6:
fn count_cards_alt1(pile: Option<~card>) -> uint {
    let mut c = 0;
    let mut pile = &pile;
    loop {
        match *pile {
            Some(ref card) => { c += 1; pile = &card.next },
            None => break
        }
    }
    return c;
}

// version 7: (classic non-tail-recursive list traversal)
fn count_cards_alt2(pile: &Option<~card>) -> uint {
    match *pile {
        Some(ref card) => 1 + count_cards_alt2(&card.next),
        None => 0
    }
}

// version 8: similar to 7 but shares interface with version 1--6.
fn count_cards_alt3(pile: Option<~card>) -> uint {
    fn recur(pile: &Option<~card>) -> uint {
        match pile {
            &Some(ref card) => 1 + recur(&card.next),
            &None => 0
        }
    }
    recur(&pile)
}

fn make_hand() -> ~card {
    let hand = ~card { tag: facedown, suit: clubs, rank: 10, next: None,
                       title: [' ', '1', '0', ' ', 'C'] };
    let hand = ~card { tag: faceup, suit: spades, rank: 3, next: Some(hand),
                       title: [' ', ' ', '3', ' ', 'S'] };
    let hand = ~card { tag: faceup, suit: diamonds, rank: 2, next: Some(hand),
                       title: [' ', ' ', '2', ' ', 'D'] };
    hand
}

fn ex03() {
    io::println("Ex 03");
    let hand = make_hand();
    let mut pile = pile{cards:Some(hand)};
    let AceD = ~card{ tag: facedown, suit: diamonds, rank: 1, next: None,
                      title: [' ', ' ', 'A', ' ', 'D'] };
    pile.place_top_faceup(AceD);
    io::println(fmt!("pile: %s", pile.to_str()));
    let card = pile.pop_top();
    io::println(fmt!("card: %s", card.to_str()));
}

fn ex04() {
    io::println("Ex 04");
    let hand = make_hand();
    let mut pile = pile{cards:Some(hand)};
    let AceD = ~card{ tag: facedown, suit: diamonds, rank: 1, next: None,
                      title: [' ', ' ', 'A', ' ', 'D'] };
    pile.place_bot_facedown(AceD);
    io::println(fmt!("pile: %s", pile.to_str()));
}

fn main() {
    ex03();
    ex04();
}
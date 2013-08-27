enum card_suit { clubs, diamonds, hearts, spades }
struct card { suit: card_suit,
              rank: u8, // 1..13
              next: Option<@mut card> }

// === THE PUZZLE: ===

// Make four functions, place_top, place_bot, pop_top, and pop_bot,
// which respectively:
//
// -- Push a card onto the top of the stack (represented by the
//    owned linked list above); return the new top of the stack.
//
// -- Place a card beneath the stack.  For an empty stack, the placed
//    card is returned as the top of the newly formed stack;
//    otherwise, the old stack top is returned (since the stack is
//    imperatively modified).
//
// -- Pop the top of the stack, returning a tuple of the popped card
//    and the new, potentially empty stack
//
// -- Remove the bottom of the stack (i.e. "cheat"), returning a tuple
//    of the removed card and the new, potentially empty stack

/*
// [c1, ..., cN], cX -o [cX, c1, ..., cN]
fn place_top(pile: Option<~card>, newcard: ~card) -> ~card;

// [c1, ..., cN], cX -o [c1, ..., cN, cX]
fn place_bot(pile: Option<~card>, newcard: ~card) -> ~card;

// [c1, c2, ..., cN] -o (c1, [c2, ..., cN])
fn pop_top(pile: ~card) -> (~card, Option<~card>);

// [c1, ..., cN-1, cN] -o (Some(cN), [c1, ..., cN-1])
fn pop_bot(pile: ~card) -> (~card, Option<~card>)

*/


// [c1, ..., cN], cX -o [cX, c1, ..., cN]
fn place_top(pile: Option<@mut card>, newcard: @mut card) -> @mut card {
    newcard.next = pile;
    newcard
}

// [c1, ..., cN], cX -o [c1, ..., cN, cX]
fn place_bot(pile: Option<@mut card>, newcard: @mut card) -> @mut card {
    place_bot_rec(pile, newcard)
}

fn place_bot_rec(pile: Option<@mut card>, newcard: @mut card) -> @mut card {
    fn recur(pile: Option<@mut card>, newcard: @mut card) -> @mut card {
        match pile {
            None => newcard,
            Some(cards) => {
                let mut cards = cards;
                cards.next = Some(recur(copy cards.next, newcard)); // FIXME Q1.
                cards 
            }
        }
    }
    recur(pile, newcard)
}

// Question Q1: How can I change the code above to get rid of the
//   `copy`; I "just" want to walk down the list...

// Question Q2: What interface "makes most sense" for place_bot
//   function?  In particular, what form should the pile take:
//   -- fn place_bot(Option<~card>, ~card) -> ~card,
//   -- fn place_bot(&Option<~card>, ~card) -> ~card, or
//   -- fn place_bot(~Option<~card>, ~card) -> ~card  ?


// [c1, c2, ..., cN] -o (c1, [c2, ..., cN])
fn pop_top(pile: @mut card) -> (@mut card, Option<@mut card>) {
    let mut c = pile;
    let mut rest = None;
    c.next <-> rest;
    (c, rest)
}


// [c1, ..., cN-1, cN] -o (Some(cN), [c1, ..., cN-1])
fn pop_bot(pile: @mut card) -> (@mut card, Option<@mut card>) {
    fn recur(pile: @mut card) -> (@mut card, Option<@mut card>) {
        let mut pile = pile;
        let mut remaining = None;
        pile.next <-> remaining;
        match remaining {
            None => (pile, None),
            Some(cards) => {
                let (last, newcards) = recur(cards);
                pile.next = newcards; // FIXME Q3 (updates whole way back up).
                (last, Some(pile))
            }
        }
    }
    recur(pile)
}

// Question Q3: Is there a way to write pop_bot so it only modifies
//   the linked-list *once*, at next-to-last cons-cell in the series?

// Question Q4: I had a really hard time coming up with even the
//   solution above.  (I had spent much time last night struggling
//   with trying to do it via direct modifications, but even then
//   I never really consider using the swap operation to avoid
//   lending `pile`, as I had originally done when I wrote it
//   as `match pile.next { ... }`.  Any suggestions for heuristics
//   or design-principles for attacking such problems here?




// === Example hand, example usage, and printing routines follow. ===


fn make_hand() -> @mut card {
    let hand = @mut card { suit: clubs, rank: 10, next: None };
    let hand = @mut card { suit: spades, rank: 3, next: Some(hand) };
    let hand = @mut card { suit: diamonds, rank: 2, next: Some(hand) };
    hand
}

fn main() {
    let mut hand : @mut card = make_hand();
    hand.report(~"initial hand: ");
    let AceD = @mut card{ suit: diamonds, rank: 1, next: None };
    AceD.report(~"place top: ");
    let hand = place_top(Some(hand), AceD);
    hand.report(~"new hand: ");
    let SixD = @mut card{ suit: diamonds, rank: 6, next: None };
    SixD.report(~"place bot: ");
    let hand = place_bot(Some(hand), SixD);
    hand.report(~"new hand: ");
    let (top, rest) = pop_top(hand);
    top.report(~"popped top: ");
    let hand = rest.unwrap();
    hand.report(~"new hand: ");
    let (bot, rest) = pop_bot(hand);
    bot.report(~"popped bot: ");
    let hand = rest.unwrap();
    hand.report(~"new hand: ");
}

// Below are "just" some notation niceties that should not effect
// the semantics of the code + algorithms above.

impl ToStr for card_suit {
    fn to_str(&self) -> ~str {
        match self { &spades   => ~"\u2660", &hearts   => ~"\u2661",
                     &diamonds => ~"\u2662", &clubs    => ~"\u2663" } }
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

impl card {
    fn rank_to_str(&self) -> ~str { rank_to_str(self.rank) }
    fn report(&self, prefix: ~str) { io::println(prefix + self.to_str()); }
}

impl ToStr for card {
    fn to_str(&self) -> ~str {
        let mut ret = self.rank_to_str() + self.suit.to_str();
        match &self.next {
            &None => (),
            &Some(ref n) => ret = ret + "," + n.to_str()
        }
        ret
    }
}

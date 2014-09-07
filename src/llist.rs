enum card_suit { clubs, diamonds, hearts, spades }
struct card { suit: card_suit,
              rank: u8, // 1..13
              next: Option<~card> }

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

// [c1, ..., cN], cX -> [cX, c1, ..., cN]
fn place_top(pile: Option<~card>, newcard: ~card) -> ~card {
    let mut newcard = newcard;
    newcard.next = pile;
    newcard
}

// [c1, ..., cN], cX -> [c1, ..., cN, cX]
fn place_bot(pile: Option<~card>, newcard: ~card) -> ~card {
    place_bot_rec_1(pile, newcard)
}

// Q1, Option 1:
fn place_bot_rec_1(pile: Option<~card>, newcard: ~card) -> ~card {
    fn recur(pile: Option<~card>, newcard: ~card) -> ~card {
        match pile {
            None => newcard,
            Some(~card {suit, rank, next}) => {
                let next = Some(recur(next, newcard)); // FIXME Q1.
                ~card {suit: suit, rank: rank, next: next}
            }
        }
    }
    recur(pile, newcard)
}

// Q1, Option 2:
fn place_bot_rec_2(pile: Option<~card>, newcard: ~card) -> ~card {
    fn recur(pile: &mut ~card, newcard: ~card) {
        match pile.next {
            None => {
                pile.next = Some(newcard);
            }
            Some(ref mut next) => {
                recur(next, newcard);
            }
        }
    }
    match pile {
        None => newcard,
        Some(c) => {
            let mut card = c;
            recur(&mut card, newcard);
            card
        }
    }
}

// Question Q1: How can I change the code above to get rid of the
//   `copy`; I "just" want to walk down the list...
//
// * Above are two versions.  A version using a loop could be done but I
//   believe it is blocked by borrow checker bugs that would be fixed
//   with my rewrite.

// Question Q2: What interface "makes most sense" for place_bot
//   function?  In particular, what form should the pile take:
//   -- fn place_bot(Option<~card>, ~card) -> ~card,
//   -- fn place_bot(&Option<~card>, ~card) -> ~card, or
//   -- fn place_bot(~Option<~card>, ~card) -> ~card  ?
//
// * I would use `Option<~card>` if you want to keep the code more-or-less
//   as you wrote it.  Another appealing option would be something like
//
//     fn append(list: &mut Option<~card>, ~card)
//
//   This version would overwrite the list in place. Part of the
//   appeal of this is that it would allow you to append to lists
//   without the function taking ownership of them, which is sometimes
//   needed.  We can only pass by value when the value resides in an
//   owned location, but someone who possesed an `&mut Option<~card>`
//   could invoke `append()`.  The difference is that, in the
//   `append()` version, there is never a moment where the list has
//   been moved into the callee.


// [c1, c2, ..., cN] -> (c1, [c2, ..., cN])
fn pop_top(pile: ~card) -> (~card, Option<~card>) {
    let mut c = pile;
    let mut rest = None;
    std::util::swap(&mut rest, &mut c.next);
    (c, rest)
}


// [c1, ..., cN-1, cN] -> (Some(cN), [c1, ..., cN-1])
fn pop_bot(pile: ~card) -> (~card, Option<~card>) {
    fn recur(pile: ~card) -> (~card, Option<~card>) {
        let mut pile = pile;
        let mut remaining = None;
        std::util::swap(&mut remaining, &mut pile.next);
        pile.next = None;
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

fn pop_bot_2(pile: ~card) -> (~card, Option<~card>) {
    use std::util;
    let top = Some(pile);
    let mut cursor : &Option<~card> = &top;
    loop {
        match cursor {
            &Some(~card{ next: None, suit: _, rank: _ }) =>
                break,
            &Some(~card{ next: ref next, suit: _, rank: _ }) =>
                cursor = next
        }
    }
    let mut last : Option<~card> = None;
    util::swap(cursor, &mut last);
    return (last.unwrap(), Some(pile))
}

/* // (pop_bot_1 no longer type checks)
fn pop_bot_1(pile: ~card) -> (~card, Option<~card>) {
    match pile.next {
        None => return (pile, None),
        Some(_) => {
            let mut next_to_last = pile;
            while next_to_last.next.unwrap().next.is_some() {
                next_to_last = next_to_last.next.unwrap();
            }
            let last = next_to_last.next.unwrap();
            next_to_last.next = None;
            return (last, Some(pile));
        }
    }
}
*/

// Question Q3: Is there a way to write pop_bot so it only modifies
//   the linked-list *once*, at next-to-last cons-cell in the series?
//
// * Yes, see pop_bot_1 above.  It feels a bit awkward though,
//   particularly the call to `get()`.  You could write this
//   iteratively, I think, under the new borrow checker rules.

// Question Q4: I had a really hard time coming up with even the
//   solution above.  (I had spent much time last night struggling
//   with trying to do it via direct modifications, but even then
//   I never really consider using the swap operation to avoid
//   lending `pile`, as I had originally done when I wrote it
//   as `match pile.next { ... }`.  Any suggestions for heuristics
//   or design-principles for attacking such problems here?
//
// * Not sure.  I think that (1) I need to land the borrow checker rewrite;
//   (2) it probably is worth improving the liveness analysis to consider
//   partial moves and replacements; and (3) I guess we need to try and
//   come up with a better answer. ;)
//
//   I guess the key point is that you can use `&mut` to drill down into
//   your structure, but when you do that, you lose access to the outer
//   levels until your `&mut` pointer expires.  But that is sufficient
//   for these sorts of problems, generally speaking.



// === Example hand, example usage, and printing routines follow. ===


fn make_hand() -> ~card {
    let hand = ~card { suit: clubs, rank: 10, next: None };
    let hand = ~card { suit: spades, rank: 3, next: Some(hand) };
    let hand = ~card { suit: diamonds, rank: 2, next: Some(hand) };
    hand
}

fn main() {
    let hand : ~card = make_hand();
    hand.report("initial hand: ");
    let AceD = ~card{ suit: diamonds, rank: 1, next: None };
    AceD.report("place top: ");
    let hand = place_top(Some(hand), AceD);
    hand.report("new hand: ");
    let SixD = ~card{ suit: diamonds, rank: 6, next: None };
    SixD.report("place bot: ");
    let hand = place_bot_rec_1(Some(hand), SixD);
    hand.report("new hand: ");
    let SevenD = ~card{ suit: diamonds, rank: 7, next: None };
    SevenD.report("place bot: ");
    let hand = place_bot_rec_2(Some(hand), SevenD);
    hand.report("new hand: ");
    let (top, rest) = pop_top(hand);
    top.report("popped top: ");
    let hand = rest.unwrap();
    hand.report("new hand: ");
    let (bot, rest) = pop_bot(hand);
    bot.report("popped bot: ");
    let hand = rest.unwrap();
    hand.report("new hand: ");
}

// Below are "just" some notation niceties that should not effect
// the semantics of the code + algorithms above.

impl ToStr for card_suit {
    fn to_str(&self) -> ~str {
        match self { &spades   => ~"\u2664", &hearts   => ~"\u2665",
                     &diamonds => ~"\u2666", &clubs    => ~"\u2667" } }
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
    fn report(&self, prefix: &str) { println(fmt!("%s%s", prefix, self.to_str())); }
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

use std::io;

enum card_suit { clubs, diamonds, hearts, spades }
struct card { suit: card_suit,
              rank: u8, // 1..13
              next: Option<~card> }

// [c1, ..., cN], cX -> [cX, c1, ..., cN]
fn place_top(pile: Option<~card>, newcard: ~card) -> ~card
{

}

// [c1, ..., cN], cX -> [c1, ..., cN, cX]
fn place_bot(pile: Option<~card>, newcard: ~card) -> ~card
{

}

// [c1, c2, ..., cN] -> (c1, [c2, ..., cN])
fn pop_top(pile: ~card) -> (~card, Option<~card>)
{

}

// [c1, ..., cN-1, cN] -> (Some(cN), [c1, ..., cN-1])
fn pop_bot(pile: ~card) -> (~card, Option<~card>)
{

}

fn make_hand() -> ~card {
    let hand = ~card { suit: clubs, rank: 10, next: None };
    let hand = ~card { suit: spades, rank: 3, next: Some(hand) };
    let hand = ~card { suit: diamonds, rank: 2, next: Some(hand) };
    hand
}

fn main() {
    let hand : ~card = make_hand();
    hand.report(~"initial hand: ");
    let AceD = ~card{ suit: diamonds, rank: 1, next: None };
    AceD.report(~"place top: ");
    let hand = place_top(Some(hand), AceD);
    hand.report(~"new hand: ");
    let SixD = ~card{ suit: diamonds, rank: 6, next: None };
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

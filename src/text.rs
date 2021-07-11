use super::*;

/// The type of a parser that accepts (and ignores) any number of characters.
pub type Padding<E> = Repeated<Ignored<Filter<fn(&char) -> bool, E>, char>>;

/// The type of a parser that accepts (and ignores) any number of characters before or after another pattern.
pub type Padded<P, O> = PaddedBy<PaddingFor<Padding<<P as Parser<char, O>>::Error>, P, Vec<()>, O>, Padding<<P as Parser<char, O>>::Error>, O, Vec<()>>;

/// A trait containing text-specific functionality that extends the [`Parser`] trait.
pub trait TextParser<O>: Parser<char, O> {
    /// Parse a pattern, allowing whitespace both before and after.
    fn padded(self) -> Padded<Self, O> where Self: Sized {
        whitespace().padding_for(self).padded_by(whitespace())
    }
}

impl<O, P: Parser<char, O>> TextParser<O> for P {}

/// A parser that accepts (and ignores) any number of whitespace characters.
pub fn whitespace<E: Error<char>>() -> Padding<E> {
    filter((|c: &char| c.is_whitespace()) as _).ignored().repeated()
}
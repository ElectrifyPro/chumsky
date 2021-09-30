use super::*;

pub trait Strategy<I, E: Error> {
    fn recover(&self, stream: &mut StreamOf<I, E>) -> Result<(), ()>;
}

#[derive(Copy, Clone)]
pub struct NestedDelimiters<I>(pub I, pub I);

impl<I: Clone + PartialEq, E: Error> Strategy<I, E> for NestedDelimiters<I> {
    fn recover(&self, stream: &mut StreamOf<I, E>) -> Result<(), ()> {
        let mut balance = 0;
        loop {
            if match stream.next() {
                (_, _, Some(t)) if t == self.0 => { balance += 1; true },
                (_, _, Some(t)) if t == self.1 => { balance -= 1; true },
                (_, _, Some(_)) => false,
                (_, _, None) => break Err(()),
            } {
                if balance == 0 {
                    break Ok(());
                } else if balance < 0 {
                    // The end of a delimited section is not a valid recovery pattern
                    break Err(());
                }
            } else if balance == 0 {
                // A non-delimiter token before anything else is not a valid recovery pattern
                break Err(());
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Recovery<A, S, F>(pub(crate) A, pub(crate) S, pub(crate) F);

impl<I: Clone, O, A: Parser<I, O, Error = E>, S: Strategy<I, E>, F: Fn() -> O, E: Error<Token = I>> Parser<I, O> for Recovery<A, S, F> {
    type Error = E;

    fn parse_inner(&self, stream: &mut StreamOf<I, Self::Error>) -> PResult<O, Self::Error> {
        match self.0.try_parse_inner(stream) {
            (a_errors, Ok(a_out)) => (a_errors, Ok(a_out)),
            (mut a_errors, Err(a_err)) => {
                println!("Recovering from {}...", stream.offset());

                let res = if self.1.recover(stream).is_ok() {
                    a_errors.push(a_err);
                    (a_errors, Ok(((self.2)(), None)))
                } else {
                    (a_errors, Err(a_err))
                };

                println!("Recovered to {}.", stream.offset());

                res
            },
        }
    }
}
//! Error folding utilities

pub trait InteratorResultExt<E>: Iterator + Sized {
    fn fold_errs(self) -> Result<(), Vec<E>>;
}

impl<I, E> InteratorResultExt<E> for I where I: Iterator<Item=Result<(), E>> {
    fn fold_errs(self) -> Result<(), Vec<E>> {
        self.fold(Ok(()), |acc, x| {
            match (acc, x) {
                (Ok(_), Err(e)) => Err(vec![e]),
                (Err(mut es), Err(e)) => { es.push(e); Err(es) },
                (x, _) => x,
            }
        })
    }
}

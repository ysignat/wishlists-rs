#[derive(Debug)]
pub struct DataError(anyhow::Error);

impl<E> From<E> for DataError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

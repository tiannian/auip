/// IntoInner
pub trait IntoInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}

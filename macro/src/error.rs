pub trait ExtError {
    /// Returns `self` combined with the other error.
    fn into_combine(self, another: syn::Error) -> Self;

    fn into_result<T>(self) -> syn::Result<T>;
}

impl ExtError for syn::Error {
    fn into_combine(mut self, another: syn::Error) -> Self {
        self.combine(another);
        self
    }

    fn into_result<T>(self) -> syn::Result<T> {
        Err(self)
    }
}

macro_rules! format_err {
    ($spanned:expr, $($msg:tt)*) => {
        ::syn::Error::new(
            <_ as ::syn::spanned::Spanned>::span(&$spanned),
            format_args!($($msg)*)
        )
    }
}

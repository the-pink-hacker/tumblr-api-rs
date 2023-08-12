#[macro_export]
macro_rules! tags {
    ($tag: literal, $($tags: literal),*$(,)?) => {
        concat!($tag, ",", tags!($($tags),*))
    };
    ($tag: literal) => {
        $tag
    };
}

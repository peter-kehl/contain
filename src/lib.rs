mod tests;

#[macro_export]
macro_rules! pattern {
    ($id:ident) => {
        Pat($id)
    }
}

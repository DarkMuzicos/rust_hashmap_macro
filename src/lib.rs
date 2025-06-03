/// Creates a `HashMap` with given key-value pairs.
///
/// # Examples
///
/// ```
/// use hashmap_macro::hashmap;
/// 
/// let map = hashmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
///  
/// //or
///
/// let map = hashmap!{
///     ("a", 1),
///     ("b", 2)
/// };
/// assert_eq!(map["b"], 2);
/// ```
///
/// The trailing comma is optional.
///
/// Supports both `key => value` and `key, value` syntaxes.
#[macro_export]
macro_rules! hashmap {
    () => {
        {
            let hm = ::std::collections::HashMap::new();
            hm
        }
    };
    ( $( $k:expr => $v:expr ),+ $(,)? )  => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($k, $v);
            )+
            hm
        }
    };
    ( $( ($k:expr, $v:expr) ),+ $(,)? ) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($k, $v);
            )+
            hm
        }
    }
}


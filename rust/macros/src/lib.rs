#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)* ) => {
        {
            use std::collections::HashMap;
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($key, $value);
            )*
            temp_map
        }
    };
}

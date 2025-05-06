pub mod common_repo;
pub mod common_types;
pub mod common_controller;
pub mod err_codes;
pub mod error;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub(crate) use map;

use std::str;

pub trait FromSql {
    fn from_sql(raw: &Option<~[u8]>) -> Self;
}

macro_rules! from_str_impl(
    ($t:ty) => (
        impl FromSql for Option<$t> {
            fn from_sql(raw: &Option<~[u8]>) -> Option<$t> {
                match *raw {
                    None => None,
                    Some(ref buf) => {
                        let s = str::from_bytes_slice(buf.as_slice());
                        Some(FromStr::from_str(s).unwrap())
                    }
                }
            }
        }
    )
)

macro_rules! from_option_impl(
    ($t:ty) => (
        impl FromSql for $t {
            fn from_sql(raw: &Option<~[u8]>) -> $t {
                // FIXME when you can specify Self types properly
                let ret: Option<$t> = FromSql::from_sql(raw);
                ret.unwrap()
            }
        }
    )
)

from_str_impl!(int)
from_option_impl!(int)
from_str_impl!(i8)
from_option_impl!(i8)
from_str_impl!(i16)
from_option_impl!(i16)
from_str_impl!(i32)
from_option_impl!(i32)
from_str_impl!(i64)
from_option_impl!(i64)
from_str_impl!(uint)
from_option_impl!(uint)
from_str_impl!(u8)
from_option_impl!(u8)
from_str_impl!(u16)
from_option_impl!(u16)
from_str_impl!(u32)
from_option_impl!(u32)
from_str_impl!(u64)
from_option_impl!(u64)
from_str_impl!(float)
from_option_impl!(float)
from_str_impl!(f32)
from_option_impl!(f32)
from_str_impl!(f64)
from_option_impl!(f64)

impl FromSql for Option<~str> {
    fn from_sql(raw: &Option<~[u8]>) -> Option<~str> {
        do raw.chain_ref |buf| {
            Some(str::from_bytes(buf.as_slice()))
        }
    }
}
from_option_impl!(~str)

pub trait ToSql {
    fn to_sql(&self) -> Option<~[u8]>;
}

macro_rules! to_str_impl(
    ($t:ty) => (
        impl ToSql for $t {
            fn to_sql(&self) -> Option<~[u8]> {
                Some(self.to_str().into_bytes())
            }
        }
    )
)

macro_rules! to_option_impl(
    ($t:ty) => (
        impl ToSql for Option<$t> {
            fn to_sql(&self) -> Option<~[u8]> {
                do self.chain |val| {
                    val.to_sql()
                }
            }
        }
    )
)

to_str_impl!(int)
to_option_impl!(int)
to_str_impl!(i8)
to_option_impl!(i8)
to_str_impl!(i16)
to_option_impl!(i16)
to_str_impl!(i32)
to_option_impl!(i32)
to_str_impl!(i64)
to_option_impl!(i64)
to_str_impl!(uint)
to_option_impl!(uint)
to_str_impl!(u8)
to_option_impl!(u8)
to_str_impl!(u16)
to_option_impl!(u16)
to_str_impl!(u32)
to_option_impl!(u32)
to_str_impl!(u64)
to_option_impl!(u64)
to_str_impl!(float)
to_option_impl!(float)
to_str_impl!(f32)
to_option_impl!(f32)
to_str_impl!(f64)
to_option_impl!(f64)

impl<'self> ToSql for &'self str {
    fn to_sql(&self) -> Option<~[u8]> {
        Some(self.as_bytes().to_owned())
    }
}

impl ToSql for Option<~str> {
    fn to_sql(&self) -> Option<~[u8]> {
        do self.chain_ref |val| {
            val.to_sql()
        }
    }
}

impl<'self> ToSql for Option<&'self str> {
    fn to_sql(&self) -> Option<~[u8]> {
        do self.chain |val| {
            val.to_sql()
        }
    }
}
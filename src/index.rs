// Jackson Coxson

use plist::{Dictionary, Value};

pub trait PlistIndex {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;
    fn index_into_dict<'v>(&self, d: &'v Dictionary) -> Option<&'v Value>;
}

impl PlistIndex for &str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Dictionary(dict) => dict.get(self),
            _ => None,
        }
    }

    fn index_into_dict<'v>(&self, d: &'v Dictionary) -> Option<&'v Value> {
        d.get(self)
    }
}

impl PlistIndex for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Array(arr) => arr.get(*self),
            _ => None,
        }
    }

    fn index_into_dict<'v>(&self, _: &'v Dictionary) -> Option<&'v Value> {
        None
    }
}

pub trait PlistExt {
    fn get_by<I: PlistIndex>(&self, index: I) -> Option<&Value>;
}

impl PlistExt for Value {
    fn get_by<I: PlistIndex>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }
}

impl PlistExt for Dictionary {
    fn get_by<I: PlistIndex>(&self, index: I) -> Option<&Value> {
        index.index_into_dict(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let p = crate::plist!({
            "hi mom": {
                "look": {
                    "nested": 420
                }
            }
        });

        assert_eq!(
            p.get_by("hi mom")
                .and_then(|x| x.get_by("look"))
                .and_then(|x| x.get_by("nested"))
                .and_then(|x| x.as_unsigned_integer()),
            Some(420)
        );

        let p = crate::plist!(dict {
            "hi mom": {
                "look": {
                    "nested": 123
                }
            }
        });

        assert_eq!(
            p.get_by("hi mom")
                .and_then(|x| x.get_by("look"))
                .and_then(|x| x.get_by("nested"))
                .and_then(|x| x.as_unsigned_integer()),
            Some(123)
        );
    }
}

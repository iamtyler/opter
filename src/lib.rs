/****************************************************************************
*
*   lib.rs
*   opter
*
*   Copyright (c) 2015 Tyler Cole
*
*   Details:
*   - options come in four flavors
*     - flag is name without value
*     - ordinal is value without name
*     - named is value with a name
*     - value is simply a raw value
*   - full names start with "--" and short names with "-"
*   - short name flags can be stacked with "-"
*     - "-abc" represents the flags a, b, and c
*   - "-" with no following characters is a value
*   - "--" signifies end of options and is not emitted
*     - all values following "--" are simply passed along
*
***/

use std::env;
use std::iter;


/****************************************************************************
*
*   Opt
*
***/

#[derive(Debug)]
#[derive(PartialEq)]
pub struct NamedOpt {
    pub name : String,
    pub value : String,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Opt {
    Flag(String),
    Named(NamedOpt),
    Ordinal(String),
    Value(String),
}

impl Opt {
    pub fn flag_from_str (name : &str) -> Opt {
        return Opt::Flag(name.to_string());
    }

    pub fn named_from_str (name : &str, value : &str) -> Opt {
        return Opt::Named(NamedOpt {
            name : name.to_string(),
            value : value.to_string()
        });
    }

    pub fn ordinal_from_str (value : &str) -> Opt {
        return Opt::Ordinal(value.to_string());
    }

    pub fn value_from_str (value : &str) -> Opt {
        return Opt::Value(value.to_string());
    }
}


/****************************************************************************
*
*   Opts
*
***/

pub struct Opts<I> where I : Iterator<Item = String> {
    iter  : I,
    flags : Vec<char>,
    name  : String,
    drain : bool,
    done  : bool,
}

impl<I> Opts<I> where I : Iterator<Item = String> {
    fn next_opt (&mut self) -> Option<Opt> {
        // Check for done
        if self.done {
            return None;
        }

        // Process flags
        if let Some(c) = self.flags.pop() {
            return Some(Opt::Flag(c.to_string()));
        }

        // Get next value
        let value;
        match self.iter.next() {
            Some(a) => value = a,
            None => {
                // Iterator is done
                self.done = true;

                if !self.name.is_empty() {
                    // Previous value was a name so emit a flag
                    let temp = self.name.clone();
                    self.name.clear();
                    return Some(Opt::Flag(temp));
                }
                else {
                    // All done
                    return None;
                }
            }
        }

        // If draining just send along the value
        if self.drain {
            return Some(Opt::Value(value));
        }

        // Handle -- value
        if value == "--" {
            self.drain = true;
            return self.next_opt();
        }

        // Parse as name or flags
        let mut name = String::new();
        if value.starts_with("--") {
            name.push_str(&value[2..]);
        }
        else if value.starts_with("-") {
            if value.chars().count() > 2 {
                // Parse flags
                for c in value.chars().skip(1) {
                    self.flags.push(c);
                }
                self.flags.reverse();
            }
            else {
                // Save single char name
                name.push_str(&value[1..]);
            }
        }

        // Emit option
        if !self.name.is_empty() {
            // Named or flag
            if !name.is_empty() {
                // Previous was name, current is name, so previous as flag
                let temp = name;
                name = self.name.clone();
                self.name = temp;
                return Some(Opt::Flag(name));
            }
            else if !self.flags.is_empty() {
                // Previous was name, current is flags, so previous as flag
                name = self.name.clone();
                self.name.clear();
                return Some(Opt::Flag(name));
            }
            else {
                // Previous was name, current is ordinal, so current as named
                name = self.name.clone();
                self.name.clear();
                return Some(Opt::Named(NamedOpt {
                    name  : name,
                    value : value
                }));
            }
        }
        else if !name.is_empty() {
            // Previous was not name, current is name
            self.name = name;
            return self.next_opt();
        }
        else if !self.flags.is_empty() {
            // Previous was not name, current is flags
            return self.next_opt();
        }

        // Previous was not name, current is ordinal, so current as ordinal
        return Some(Opt::Ordinal(value));
    }
}

impl<I> Iterator for Opts<I> where I : Iterator<Item = String> {
    type Item = Opt;

    fn next (&mut self) -> Option<Opt> {
        return self.next_opt();
    }
}


/****************************************************************************
*
*   Public functions
*
***/

pub fn parse<I, II> (values : II) -> Opts<II::IntoIter> where
    I : Iterator<Item = String>,
    II : IntoIterator<Item = String, IntoIter = I>
{
    return Opts {
        iter  : values.into_iter(),
        flags : Vec::new(),
        name  : String::new(),
        drain : false,
        done  : false,
    };
}

pub fn parse_env () -> Opts<iter::Skip<env::Args>> {
    return parse(env::args().skip(1));
}



/****************************************************************************
*
*   Tests
*
***/

#[cfg(test)]
mod tests {
    use super::Opt;

    fn args_from_str (s : &str) -> Vec<String> {
        return s.split(" ").map(|s| s.to_string()).collect();
    }

    fn options_from_str (s : &str) -> Vec<Opt> {
        return super::parse(args_from_str(s)).collect();
    }

    #[test]
    fn full () {
        let actual = options_from_str("a -bc -d e --f g - h -- i -j --k");
        let spec = vec![
            Opt::ordinal_from_str("a"),
            Opt::flag_from_str("b"),
            Opt::flag_from_str("c"),
            Opt::named_from_str("d", "e"),
            Opt::named_from_str("f", "g"),
            Opt::ordinal_from_str("-"),
            Opt::ordinal_from_str("h"),
            Opt::value_from_str("i"),
            Opt::value_from_str("-j"),
            Opt::value_from_str("--k"),
        ];

        assert_eq!(actual, spec);
    }

    #[test]
    fn trailing_flag () {
        let actual = options_from_str("a b -c");
        let spec = vec![
            Opt::ordinal_from_str("a"),
            Opt::ordinal_from_str("b"),
            Opt::flag_from_str("c"),
        ];

        assert_eq!(actual, spec);
    }
}

// cargo-deps: serde="*"

use std::iter::Peekable;
use std::iter::Skip;
// use serde::de::DeserializeSeed;

use std::io;
use std::str::Lines;

#[derive(Copy, Clone)]
pub struct Deserializer<'a> {
    input: &'a str,
    line_idx: usize,
}

#[derive(Copy, Clone)]
pub struct LineAccess<'a> {
    inner: Deserializer<'a>,
    common: &'a str,
    depth: usize,
}

pub struct Commons<'a> {
    current: LineAccess<'a>,
    first: bool,
}

impl<'a> LineAccess<'a> {
    fn new(inner: Deserializer<'a>, depth: usize) -> Self {
        Self { inner, common: "", depth }
    }
    fn clear_state(&mut self) {
        //empty the iterator
        while let Some(_) = self.next() {  }
        self.common = "";
    }
    fn subdivide(&self) -> Commons<'a> {
        let mut sub = self.clone();
        sub.common = "";
        sub.depth += 1;
        Commons::new(sub)
    }
    fn scope(&self) -> Option<&str> {
        self.common.split('.').nth(self.depth)
    }
    fn scope_peek(&self) -> Option<&str> {
        if self.common == "" {
            let line = self.inner.peek()?;
            let lhs = line.split('=').next()?;
            lhs.split('.').nth(self.depth)
        } else {
            self.scope()
        }
    }
}

impl<'a> Commons<'a> {
    const fn new(current: LineAccess<'a>) -> Self {
        Commons { current: current, first: true }
    }
}

fn get_prefix(line: &str, depth: usize) -> Option<&str> {
    let lhs = line.split('=').next()?;
    let idx = lhs.split('.').take(depth+1).map(|x| x.trim().len() + 1).sum::<usize>();
    line.get(.. idx.saturating_sub(1))
}

impl<'a> Iterator for LineAccess<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.inner.peek()?;
        let properties = get_prefix(line, self.depth)?;
        if self.common == "" {
            self.common = properties;
        } 
        //dbg!(self.depth, self.common);
        if self.common.starts_with(properties) {
            self.inner.line_idx += 1;
            Some(line)
        } else {
            None
        }
        
    }
}

impl<'a> Iterator for Commons<'a> {
    type Item = LineAccess<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cur = self.current.clone();
        let line = cur.next().unwrap_or("");
        let common_prefix = 
        if self.current.depth == 0 {
            ""
        } else {
            get_prefix(cur.common, self.current.depth.saturating_sub(1))?
        };
        if !self.first {
            //println!("empty");
            cur.clear_state();
            let line = cur.next()?;
            self.current.clear_state();
            
                //dbg!(self.current.depth, line, common_prefix);
            if !line.starts_with(common_prefix) {
                return None;
            }
        }
        self.first = false;
        Some(self.current)
    }
}


impl<'a> Deserializer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, line_idx: 0 }
    }
    fn read(&mut self) -> Option<&'a str> {
        let line = self.input.lines().nth(self.line_idx);
        self.line_idx += 1;
        line
    }
    fn peek(&self) -> Option<&'a str> {
        self.input.lines().nth(self.line_idx)
    }
    fn rest_of_lines(&self) -> Skip<Lines<'a>> {
        self.input.lines().skip(self.line_idx)
    }
    fn split(&self) -> Option<(&'a str, &'a str)> {
        let mut parts = self.peek()?.split('=');
        let lhs = parts.next()?;
        let rhs = parts.next()?;
        Some((lhs, rhs))
    }
    fn properties(&self) -> Option<impl Iterator<Item=&'a str>> {
        let (lhs, _) = self.split()?;
        Some(lhs.split('.'))
    }
    fn get_cmn(&self, depth: usize) -> LineAccess<'a> {
        LineAccess::new(*self, depth)
    }
    fn commons(&self, depth: usize) -> Commons<'a> {
        Commons::new(self.get_cmn(depth))
    }
}

const INPUT: &str = 
"_.converter.version=20050823
_.file_name=CAMPV001_BASE.a3da
_.property.version=20050706
camera_root.0.interest.rot.x.type=0
camera_root.0 =xd
camera_root.1.interest.rot.x.type=0
camera_root.1.interest.rot.y.type=69
";

//#[test]
fn main() {
    let mut de = Deserializer::new(INPUT);
    for common in de.commons(0).take(3) {
        println!("----------------------");
        println!("scope: {}", common.scope_peek().unwrap_or("no scope"));
        for c1 in common.subdivide() {
            println!("++++++++++");
            for line in c1 {
                println!("{}", line);
            }
        }
    }
}

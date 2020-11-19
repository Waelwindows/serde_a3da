use serde::{ser, Serialize};
use slab_tree::*;

use crate::error::*;

type Result<T> = std::result::Result<T, SerializeError>;

struct Serializer {
    tree: Tree<String>,
    cur: Option<NodeId>,
}

impl Serializer {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for Serializer {
    fn default() -> Self {
        let tree = TreeBuilder::new().with_root("root".to_string()).build();
        Self { tree, cur: None, }
    }
}

struct SubSerializer<'a> {
    inner: &'a mut Serializer,
    count: usize,
    buf: String,
    previous_root: Option<NodeId>,
}

impl<'a> SubSerializer<'a> {
    fn new(inner: &'a mut Serializer) -> SubSerializer<'a> {
        Self { inner, count: 0, buf: String::new(), previous_root: None }
    }
}

pub fn to_string<T: ?Sized>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(write_tree(&serializer.tree))
}

pub fn to_writer<W: std::io::Write, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    write_to_writer_inner(writer, &serializer.tree);
    Ok(())
}

impl Serializer {
    fn get_cur_mut(&mut self) -> Option<NodeMut<'_, String>> {
        self.cur.and_then(move |x| self.tree.get_mut(x))
    }

    fn get_mut(&mut self) -> NodeMut<'_, String> {
        if let Some(cur) = self.cur {
            self.get_cur_mut().unwrap()
        } else {
            //Root is guarranteed to exist
            self.tree.root_mut().unwrap()
        }
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = SerializeError;

    type SerializeSeq = SubSerializer<'a>;
    type SerializeTuple = SubSerializer<'a>;
    type SerializeTupleStruct = SubSerializer<'a>;
    type SerializeTupleVariant = SubSerializer<'a>;
    type SerializeMap = SubSerializer<'a>;
    type SerializeStruct = SubSerializer<'a>;
    type SerializeStructVariant = SubSerializer<'a>;

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<()> {
         self.get_mut().append(v.to_string());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
         self.get_mut().append(v.to_string());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.get_mut().append(v.to_string());
        Ok(())
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.get_mut().append(v.to_string());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.get_mut().append(v.to_string());
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.get_mut().append(v.to_string());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        variant_index.serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SubSerializer::new(self))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(SubSerializer::new(self))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(SubSerializer::new(self))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let mut root = self.get_mut();
        let mut node = root.append("type".to_string());
        node.append(variant_index.to_string());
        Ok(SubSerializer::new(self))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SubSerializer::new(self))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(SubSerializer::new(self))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SubSerializer::new(self))
    }
}

trait TreeExt {
    fn merge(&mut self, at: NodeId, other: Self) -> Option<()>;
}

impl<T: Clone + PartialEq + std::fmt::Debug> TreeExt for Tree<T> {
    fn merge(&mut self, at: NodeId, other: Self) -> Option<()> {
        println!("self:");
        debug_tree(&self);
        println!("other:");
        debug_tree(&other);
        let mut stack: Vec<NodeId> = vec![at];
        //Return success if the other tree is empty
        let root = if let Some(r) = other.root() { r } else { return Some(()) };

        for node in root.traverse_pre_order().skip(1) {
            if stack.len() == 0 {
               return Some(())
            }
            let last_idx = stack.len() - 1;
            let prev_id = stack[last_idx];
            let mut prev = self.get_mut(prev_id)?;

            let id = prev.append(node.data().clone()).node_id();

            if node.children().count() > 0 {
                stack.push(id);
            } else {
                if node.node_id() == node.parent()?.last_child()?.node_id() {
                    // NodeRef::anncestors() also includes the current node
                    let mut depth = 0;
                    for ann in node.ancestors() {
                        if Some(ann.node_id()) == ann.parent().map(|x| x.node_id()) {
                            depth += 1;
                        }
                    }
                    let depth = node.ancestors().count() - 1;
                    dbg!(depth);
                    dbg!(node.data());
                    for _ in 0..depth {
                        println!("pop");
                        stack.pop();
                    }
                }
            }
        }
        Some(())
    }
}
#[test]
fn tree_test() {
    let mut tree = TreeBuilder::new().with_root(1).build();
    let mut root = tree.root_mut().unwrap();
    root.append(2).append(3);
    let four = root.append(4).node_id();

    debug_tree(&tree);

    let mut other = TreeBuilder::new().with_root(0).build();
    let mut root = other.root_mut().unwrap();
    let mut mid = root.append(5);
    mid.append(7);
    let mut last = mid.append(8);
    last.append(10);
    last.append(11);
    root.append(9);

    debug_tree(&other);

    tree.merge(four, other);
    println!("{}", write_tree(&tree));

    // debug_tree(&tree);
    // let mut tree = TreeBuilder::new().with_root(1).build();
    // let mut other = TreeBuilder::new().with_root(0).build();
    // let mut root = other.root_mut().unwrap();
    // let mut mid = root.append(5);
    // mid.append(7);
    // tree.merge(tree.root_id().unwrap(), other);
    // debug_tree(&tree);
    panic!()
}

impl<'a> ser::SerializeSeq for SubSerializer<'a> {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize
    {
        let prev = self.inner.cur;
        let mut root = self.inner.get_mut();
        let node = root.append(self.count.to_string());
        self.inner.cur = Some(node.node_id());
        value.serialize(&mut *self.inner)?;
        self.inner.cur = prev;
        self.count += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let mut root = self.inner.get_mut();
        let mut node = root.append("length".to_string());
        node.append(self.count.to_string());
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for SubSerializer<'a> {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize
    {
        <Self as ser::SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        <Self as ser::SerializeSeq>::end(self)
    }
}

impl<'a> ser::SerializeStruct for SubSerializer<'a> {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize
    {
        let mut root = self.inner.get_mut();
        let node = root.append(key.to_string()).node_id();

        let prev = self.inner.cur;
        self.inner.cur = Some(node);
        value.serialize(&mut *self.inner)?;
        self.inner.cur = prev;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}


//TODO: is this correct behaviour and is it valid to impl for this format?
impl<'a> ser::SerializeMap for SubSerializer<'a> {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize
    {
        self.previous_root = self.inner.cur;
        let mut root = self.inner.get_mut();
        key.serialize(&mut *self.inner)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize
    {
        let mut root = self.inner.get_mut();
        value.serialize(&mut *self.inner)?;
        self.inner.cur = self.previous_root;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for SubSerializer<'a> {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize 
    {
        let mut str = to_string(value)?; 
        str += ", ";
        self.buf += &str;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let mut tuple = (&self.buf[..self.buf.len() - 2 - 1]).to_string();
        tuple.push(')');
        let mut root = self.inner.get_mut();
        root.append(tuple);
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for SubSerializer<'a> {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize
    {
        <Self as ser::SerializeTupleStruct>::serialize_field(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        <Self as ser::SerializeTupleStruct>::end(self)
    }
}

impl<'a> ser::SerializeStructVariant for SubSerializer<'a> {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        <SubSerializer as ser::SerializeStruct>::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        <SubSerializer as ser::SerializeStruct>::end(self)
    }
}

fn debug_tree<T: std::fmt::Debug>(tree: &Tree<T>) {
    let mut out = String::new();
    tree.write_formatted(&mut out);
    println!("{}", out);
}

fn write_to_writer_inner<W: std::io::Write, T: ToString>(mut writer: W, tree: &Tree<T>) -> Result<()> {
    let root = if let Some(root) = tree.root() { root } else { return Ok(()) };
    for node in root.traverse_post_order().filter(|x| x.children().count() == 0 ) {
        let ancestors: Vec<_> = node.ancestors().collect();
        let mut line = ancestors.into_iter().rev().skip(1).fold(String::new(), |a, b| a + &b.data().to_string() + ".");
        line.remove(line.len() - 1);
        line.push('=');
        line += &node.data().to_string();
        line.push('\n');
        writer.write(line.as_bytes())?;
    }
    Ok(())
}

fn write_tree<T: ToString>(tree: &Tree<T>) -> String {
    let mut string = String::new();
    let root = if let Some(root) = tree.root() { root } else { return string };
    for node in root.traverse_post_order().filter(|x| x.children().count() == 0 ) {
        let ancestors: Vec<_> = node.ancestors().collect();
        let mut line = ancestors.into_iter().rev().skip(1).fold(String::new(), |a, b| a + &b.data().to_string() + ".");
        line.remove(line.len() - 1);
        line.push('=');
        line += &node.data().to_string();
        line.push('\n');
        string += &line;
    }
    string
}

#[test]
fn serialize_seq() {
    let seq = [39.39, 420.69];
    let mut serializer = Serializer::new();
    let ser = seq.serialize(&mut serializer);

    debug_tree(&serializer.tree);
    println!("{}", write_tree(&serializer.tree));
    panic!()
}

#[test]
fn serialize_struct() {
    use serde_derive::*;
    #[derive(Serialize)]
    struct Test {
        foo: u32,
        bar: f32,
        baz_array: [u8; 4]
    }

    // #[derive(Serialize)]
    // struct BazArray {
    //     baz: Vec<u8>
    // }
    // let seq = Test { foo: 69, bar: 39.39, baz_array: BazArray{baz:vec![39, 39, 69, 0]} };
    let seq = Test { foo: 69, bar: 39.39, baz_array: [39, 39, 69, 0] };
    // let seq = Test { foo: 69, bar: 39.39 };
    let mut serializer = Serializer::new();
    let ser = seq.serialize(&mut serializer);

    debug_tree(&serializer.tree);
    println!("{}", write_tree(&serializer.tree));
    panic!()
}

#[test]
fn serialize_struct2() {
    use serde_derive::*;
    #[derive(Serialize)]
    struct A3daFile {
        #[serde(rename="_")]
        metadata: A3daMetadata,
    }

    #[derive(Serialize)]
    struct A3daMetadata {
        converter: Converter,
        file_name: String,
        property: Property,
    }

    #[derive(Serialize)]
    struct Converter {
        version: usize
    }

    #[derive(Serialize)]
    struct Property {
        version: usize
    }

    let converter = Converter { version: 20050823 };
    let property = Property { version: 20050706 };
    let metadata = A3daMetadata { file_name: "CAMPV001_BASE.a3da".to_string(), converter, property };
    let a3da = A3daFile { metadata };

    let result = to_string(&a3da).unwrap();
    let output = "_.converter.version=20050823
_.file_name=CAMPV001_BASE.a3da
_.property.version=20050706
";
    assert_eq!(result, output);
}

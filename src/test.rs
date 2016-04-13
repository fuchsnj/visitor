use super::*;

struct Data{
	a: u32,
	b: u32
}
impl Visit<u32> for Data{
	fn visit<V: Visitor<u32>>(&self, v: &mut V) -> Result<(),V::Error>{
		try!(v.visit(self.a));
		try!(v.visit(self.b));
		Ok(())
	}
}

struct AddVisitor{
	value: u32
}
impl Visitor<u32> for AddVisitor{
	type Error = ();
	fn visit(&mut self, data: u32) -> Result<(), Self::Error>{
		self.value += data;
		Ok(())
	}
}

#[test]
fn it_works() {
	let data = Data{
		a: 3,
		b: 4
	};
	let mut adder = AddVisitor{
		value: 0
	};
	
	data.visit(&mut adder).unwrap();
	
	assert_eq!(adder.value, 7);
}
#[cfg(test)]
mod test;

pub trait Visitor{
	type Error;
	fn visit(&mut self, data: u32) -> Result<(), Self::Error>;
}

pub trait Visit{
	fn visit<V: Visitor>(&self, f: &mut V) -> Result<(),V::Error>;
}
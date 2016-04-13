#[cfg(test)]
mod test;

pub trait Visitor<T>{
	type Error;
	fn visit(&mut self, data: T) -> Result<(), Self::Error>;
}

pub trait Visit<T>{
	fn visit<V: Visitor<T>>(&self, f: &mut V) -> Result<(),V::Error>;
}
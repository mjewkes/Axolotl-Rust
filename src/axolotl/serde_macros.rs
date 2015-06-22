
macro_rules! next_field {

	($field:expr ; { $($from:ident,)+ } { $($to:ident,)+ }) => {
		match $field {
			$(Field::$from => Field::$to,)+
			other => other,
		}
	};

	($field:expr ; $fst:ident, $($rest:ident,)*) => {
		next_field!($field ; { $fst, $($rest,)* } { $($rest,)* End,});
	};
}

macro_rules! first_field {
	($fst:ident, $($rest:ident,)*) => {
		Field::$fst
	}
}

macro_rules! impl_axolotl_serde {
	($($module:ident { $Type:ident { $($field:ident),+ } })+) => { $(
		mod $module {
			mod ser {
				use axolotl::Axolotl;
				use super::super::$Type;
				use serde::ser::{Serialize,Serializer,MapVisitor};
			
				struct StructVisitor<'a, T> where T:Axolotl {
				    value : &'a $Type<T>,
				    state : Field,
				}

				#[derive(Copy,Clone)]
				#[allow(non_camel_case_types)]
				enum Field {
					$($field,)+
					End
				}

				impl<'a, T:Axolotl> MapVisitor for StructVisitor<'a,T> {
				    #[inline]
				    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: Serializer {
				        let state = self.state;
				        self.state = next_field!(self.state ; $($field,)+);
				        match state {
				            $(Field::$field => serializer.visit_map_elt(stringify!($field), &self.value.$field).map(|_|Some(())),)+
				            _ => Ok(None),
				        }
				    }
				
				    #[inline]
				    fn len(&self) -> Option<usize> {
				        Some(Field::End as usize)
				    }
				}
				
				impl<T:Axolotl> Serialize for $Type<T> {
				    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S:Serializer{
				        serializer.visit_map(StructVisitor{value:self, state:first_field!($($field,)+) })
				    }
				}
			}
			mod de {
				use std::marker::PhantomData;
				use axolotl::Axolotl;
				use super::super::$Type;
				use serde;
				use serde::de::{Deserialize,Deserializer,Visitor,MapVisitor};
	
				struct StructVisitor<T> where T:Axolotl {
					marker : PhantomData<T>,
				}
	
				#[allow(non_camel_case_types)]
				enum Field { $($field),+ }
	
				struct FieldVisitor;
	
				impl<T:Axolotl> Deserialize for $Type<T> {
					fn deserialize<D>(deserializer: &mut D) -> Result<$Type<T>, D::Error> where D: serde::Deserializer {
						deserializer.visit_map(StructVisitor { marker : PhantomData })
					}
				}
	
				impl Deserialize for Field {
					fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error> where D: serde::Deserializer {
						deserializer.visit(FieldVisitor)
					}
				}
				
			    impl Visitor for FieldVisitor {
			        type Value = Field;
				
			        fn visit_str<E>(&mut self, value: &str) -> Result<Field, E> where E: serde::de::Error {
			            match value { 
			            	$(stringify!($field) => Ok(Field::$field),)+
			                _ => Err(serde::de::Error::syntax_error()),
			            }
			        }
			    }
	
			    impl<T:Axolotl> Visitor for StructVisitor<T> {
					type Value = $Type<T>;
					fn visit_map<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error> where V: MapVisitor {
						$(let mut $field = None;)+
			
						loop {
							match try!(visitor.visit_key()) {
								$(Some(Field::$field) => { $field = Some(try!(visitor.visit_value())); },)+
								None => { break; },
							}
						}
			
						$(let $field = match $field {
							Some($field) => $field,
							None => try!(visitor.missing_field(stringify!($field))),
						};)+
			
						Ok($Type { $($field:$field),+ })
					}
				}
			}
		}
	)+}
}
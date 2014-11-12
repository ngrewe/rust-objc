use runtime::Class;
use foundation::INSObject;

pub struct ClassName<T>(pub &'static str);

pub fn class<T: INSObject>() -> &'static Class {
	let ClassName(name): ClassName<T> = INSObject::class_name();
	match Class::get(name) {
		Some(cls) => cls,
		None => panic!("Class {} not found", name),
	}
}

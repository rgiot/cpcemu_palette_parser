use std::ffi::CString;
use crate::*;

#[derive(Debug)]
pub enum Action {
	Memory(MemoryAction),
	BreakPoint(BreakPointAction)
}


impl Action {
	pub fn execute(&self) {
		match self {
    		Action::Memory(MemoryAction::Read { address, amount }) => {
				let address = address.clone().to_c();
				// amount is not taken into account ATM

				unsafe{cpcemu_mem_read_at(
					address.value_ptr(), 
					address.label_ptr()
				)};
							
			},
			Action::Memory(MemoryAction::Write { address, value }) => {
				let address = address.clone().to_c();
				let value = value.clone().to_c();
				unsafe{
					cpcemu_mem_write_at(
						address.value_ptr(),
						address.label_ptr(),
						value.value_ptr(),
						value.label_ptr()
					);
				}

			},
			Action::BreakPoint(BreakPointAction::AddAt{ dest, cond }) => {
				let dest = dest.clone().to_c();
				unsafe{
					cpcemu_brk_add_z80_brk_at(
						dest.value_ptr(),
						dest.label_ptr(),
						std::ptr::null()
					)
				};
			},

		}
	}
}

#[derive(Debug, Clone)]
pub enum Value<T> {
	Label(String),
	Value(T)
}

#[derive(Debug)]
pub enum CValue<T> {
	Label(CString),
	Value(T)
}


impl<T: Copy> Into<Option<T>> for &Value<T> {
    fn into(self) -> Option<T> {
        match self {
            Value::Label(_) => None,
            Value::Value(v) => Some(*v),
        }
    }
}

impl<T> From<T> for Value<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}




impl<T> Value<T> {
	pub fn from_label(label: String) -> Self {
		Value::Label(label)
	}
	pub fn to_c(self) -> CValue<T> {
		match self {
			Self::Label(l) => CValue::Label(CString::new(l).unwrap()),
			Self::Value(v) => CValue::Value(v)
		}
	}
}


impl<T> CValue<T> {
	fn value_ptr(&self) -> *const T {
		match self {
			CValue::Label(_) => std::ptr::null(),
			CValue::Value(v) => v,
		}
	}

	fn label_ptr(&self) -> *const i8 {
		match self {
			CValue::Label(l) => l.as_ptr(),
			CValue::Value(_) => std::ptr::null(),
		}
	}
}


#[derive(Debug)]
pub enum MemoryAction {
	Read{address: Value<u16>, amount: Value<u16>},
	Write{address: Value<u16>, value: Value<u8>},
}

#[derive(Debug)]
pub enum BreakPointAction {
	AddAt{dest: Value<u16>, cond: Option<BreakPointCondition>}
}

#[derive(Debug)]
pub enum BreakPointCondition {

}


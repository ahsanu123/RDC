use crate::error::Max31865Err;

pub trait ToDebugTrait {
    fn to_debug(&self);
}

impl ToDebugTrait for Result<(), Max31865Err> {
    fn to_debug(&self) {
        match self {
            Ok(_) => {}
            Err(err) => {
                panic!("{err:?}")
            }
        }
    }
}

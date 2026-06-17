

#[derive(Clone, Copy)]
pub enum TraversalReturn {
    Continue,
    DontContinue,
    Undefined,
}

impl From<u8> for TraversalReturn {
    fn from(value: u8) -> Self {
        match value {
            10 => TraversalReturn::Continue,
            11 => TraversalReturn::DontContinue,
            _ => TraversalReturn::Undefined,
        }
    }
}

impl From<TraversalReturn> for u8 {
    fn from(value: TraversalReturn) -> Self {
        match value {
            TraversalReturn::Continue => 10,
            TraversalReturn::DontContinue => 11,
            TraversalReturn::Undefined => 0,
        }
    }
}

impl PartialEq for TraversalReturn {
    fn eq(&self, other: &Self) -> bool {
        return u8::from(*self) == u8::from(*other);
    }
}
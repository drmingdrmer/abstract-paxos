use std::collections::HashMap;

use crate::system::System;
use crate::Types;

pub type MonoHistories<S> =
    HashMap<<<S as System>::Types as Types>::Time, <S as System>::MonoHistory>;

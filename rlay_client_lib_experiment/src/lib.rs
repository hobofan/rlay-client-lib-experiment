pub use rlay_client_lib_experiment_macros::schema_module;

pub mod prelude {
    pub use crate::individual_with_children;
    pub use crate::individual_with_children_internal;
    pub use crate::schema_module;

    pub use crate::AddToIndividual;
    pub use crate::CborBytesNoPrefix;
    pub use crate::CidBytesPb;
    pub use crate::InherentAssertion;
    pub use crate::InherentAssertionProperty;
}

use cid_fork_rlay::ToCid;
use rlay_ontology::ontology::*;

pub trait InherentAssertion {
    fn inherent(property: &[u8], target: &[u8]) -> Self;
}

pub trait InherentAssertionProperty {
    type PositiveAssertion;

    fn inherent_assertion(&self, target: &[u8]) -> Self::PositiveAssertion;
}

impl InherentAssertion for DataPropertyAssertion {
    fn inherent(property: &[u8], target: &[u8]) -> Self {
        DataPropertyAssertion {
            property: Some(property.to_owned()),
            target: Some(target.to_owned()),
            ..DataPropertyAssertion::default()
        }
    }
}

impl InherentAssertionProperty for DataProperty {
    type PositiveAssertion = DataPropertyAssertion;

    fn inherent_assertion(&self, target: &[u8]) -> Self::PositiveAssertion {
        DataPropertyAssertion::inherent(&self.cid_bytes_pb(), target)
    }
}

pub trait CidBytesPb {
    fn cid_bytes_pb(&self) -> Vec<u8>;
}

impl<T: ToCid> CidBytesPb for T {
    fn cid_bytes_pb(&self) -> Vec<u8> {
        self.to_cid().unwrap().to_bytes()
    }
}

pub trait CborBytesNoPrefix {
    fn cbor_bytes_no_prefix(&self) -> Vec<u8>;
}

impl<T: serde::Serialize> CborBytesNoPrefix for T {
    fn cbor_bytes_no_prefix(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).unwrap()
    }
}

pub trait AddToIndividual<T> {
    fn add_to_individual(&mut self, inherent_assertion: &T);
}

impl AddToIndividual<DataPropertyAssertion> for Individual {
    fn add_to_individual(&mut self, inherent_assertion: &DataPropertyAssertion) {
        self.data_property_assertions
            .push(inherent_assertion.cid_bytes_pb());
    }
}

#[macro_export]
macro_rules! individual_with_children {
    ($($map:tt)+) => {
        individual_with_children_internal!($($map)+)
    };
}

#[macro_export]
// Based on json_internal! macro
// https://github.com/serde-rs/json/blob/a03d5ffaa2016da97dd3b1f3a697198210042e53/src/macros.rs
macro_rules! individual_with_children_internal {
    // Done
    (@object $ind:ident $children:ident () () ()) => {};

    // Add the current entry followed by trailing comma.
    (@object $ind:ident $children:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let assertion = ($($key)+).inherent_assertion($value);
        $ind.add_to_individual(&assertion);
        $children.push(assertion.into());

        individual_with_children_internal!(@object $ind $children () ($($rest)*) ($($rest)*));
    };

    (@object $ind:ident $children:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        individual_with_children_internal!(@object $ind $children ($key) (: $($rest)*) (: $($rest)*));
    };

    (@object $ind:ident $children:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        individual_with_children_internal!(@object $ind $children [$($key)+] ($value) , $($rest)*);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $ind:ident $children:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        individual_with_children_internal!(@object $ind $children ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@object $ind:ident $children:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        individual_with_children_internal!(@object $ind $children ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    // Full token tree; Starting point
    ({ $($tt:tt)+ }) => {{
        let mut ind = Individual::default();
        let mut children = Vec::new();

        individual_with_children_internal!(@object ind children () ($($tt)+) ($($tt)+));

        (ind, children)
    }}
}

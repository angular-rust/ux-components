use super::Id;

#[derive(Default, Debug, Copy, Clone, PartialEq, Hash)]
pub struct Parent(pub Id);

#[derive(Default, Debug, Copy, Clone)]
pub struct Visibility(pub bool);

#[derive(Default, Debug, Copy, Clone)]
pub struct Position(pub f32, pub f32);

#[derive(Default, Debug, Copy, Clone)]
pub struct Size(pub f32, pub f32);

/// Immutable layout constraints.
/// Similar as Flutter implementation
///
#[derive(Default, Debug, Copy, Clone)]
pub struct BoxConstraints {
    pub min: Option<Size>,
    pub max: Option<Size>,
}

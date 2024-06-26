#![allow(non_snake_case, non_upper_case_globals)]

#![doc = include_str!("../README.md")]

pub use dioxus_class;

pub mod macros;

pub mod modifier;
pub mod responsive;
pub mod dark_mode;
pub mod layout;
pub mod flex;
pub mod spacing;
pub mod sizing;
pub mod typography;
pub mod backgrounds;
pub mod borders;
pub mod effects;
pub mod filters;
pub mod tables;
pub mod transitions;
pub mod transforms;
pub mod interactivity;
pub mod svg;
pub mod accessibility;

pub mod build;

pub mod prelude {
    pub use dioxus_class::prelude::*;

    // macros
    pub use crate::colors;

    pub use crate::modifier::*;
    pub use crate::responsive::*;
    pub use crate::dark_mode::*;
    pub use crate::layout::*;
    pub use crate::flex::*;
    pub use crate::spacing::*;
    pub use crate::sizing::*;
    pub use crate::typography::*;
    pub use crate::backgrounds::*;
    pub use crate::borders::*;
    pub use crate::effects::*;
    pub use crate::filters::*;
    pub use crate::tables::*;
    pub use crate::transitions::*;
    pub use crate::transforms::*;
    pub use crate::interactivity::*;
    pub use crate::svg::*;
    pub use crate::accessibility::*;
}

pub mod ext {
    pub use dioxus_class::ext::*;

    pub use crate::special_ending_internal;
    pub use crate::special_ending;
    pub use crate::any;
    pub use crate::minus;
    pub use crate::colors;
    pub use crate::blend;
    pub use crate::_1_to_6;
    pub use crate::_1_to_12;
    pub use crate::_1_to_13;
    pub use crate::size_0_to_96;
    pub use crate::fraction_2_to_4;
    pub use crate::fraction_2_to_6;
    pub use crate::fraction_12;
    pub use crate::sm_to_2xl;
    pub use crate::xs_to_7xl;
    pub use crate::_3xs_to_7xl;
    pub use crate::opacities;
    pub use crate::scales;
    pub use crate::percents;
    pub use crate::border_radius;
    pub use crate::border_width;
}
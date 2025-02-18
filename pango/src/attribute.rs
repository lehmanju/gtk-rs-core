// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AttrClass;
use crate::AttrType;
use crate::Attribute;
use crate::FontDescription;
use crate::Gravity;
use crate::GravityHint;
#[cfg(any(feature = "v1_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
use crate::Overline;
use crate::Rectangle;
#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
use crate::ShowFlags;
use crate::Stretch;
use crate::Style;
use crate::Underline;
use crate::Variant;
use crate::Weight;
use glib::translate::*;

impl Attribute {
    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_allow_breaks_new")]
    pub fn new_allow_breaks(allow_breaks: bool) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_allow_breaks_new(allow_breaks.into_glib())) }
    }

    #[doc(alias = "pango_attr_background_alpha_new")]
    pub fn new_background_alpha(alpha: u16) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_background_alpha_new(alpha)) }
    }

    #[doc(alias = "pango_attr_background_new")]
    pub fn new_background(red: u16, green: u16, blue: u16) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_background_new(red, green, blue)) }
    }

    #[doc(alias = "pango_attr_fallback_new")]
    pub fn new_fallback(enable_fallback: bool) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_fallback_new(enable_fallback.into_glib())) }
    }

    #[doc(alias = "pango_attr_family_new")]
    pub fn new_family(family: &str) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_family_new(family.to_glib_none().0)) }
    }

    #[doc(alias = "pango_attr_font_desc_new")]
    pub fn new_font_desc(desc: &FontDescription) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_font_desc_new(desc.to_glib_none().0)) }
    }

    #[doc(alias = "pango_attr_font_features_new")]
    pub fn new_font_features(features: &str) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_font_features_new(features.to_glib_none().0)) }
    }

    #[doc(alias = "pango_attr_foreground_alpha_new")]
    pub fn new_foreground_alpha(alpha: u16) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_foreground_alpha_new(alpha)) }
    }

    #[doc(alias = "pango_attr_foreground_new")]
    pub fn new_foreground(red: u16, green: u16, blue: u16) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_foreground_new(red, green, blue)) }
    }

    #[doc(alias = "pango_attr_gravity_hint_new")]
    pub fn new_gravity_hint(hint: GravityHint) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_gravity_hint_new(hint.into_glib())) }
    }

    #[doc(alias = "pango_attr_gravity_new")]
    pub fn new_gravity(gravity: Gravity) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_gravity_new(gravity.into_glib())) }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_insert_hyphens_new")]
    pub fn new_insert_hyphens(insert_hyphens: bool) -> Self {
        unsafe {
            from_glib_full(ffi::pango_attr_insert_hyphens_new(
                insert_hyphens.into_glib(),
            ))
        }
    }

    #[doc(alias = "pango_attr_letter_spacing_new")]
    pub fn new_letter_spacing(letter_spacing: i32) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_letter_spacing_new(letter_spacing)) }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_attr_overline_color_new")]
    pub fn new_overline_color(red: u16, green: u16, blue: u16) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_overline_color_new(red, green, blue)) }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_attr_overline_new")]
    pub fn new_overline(overline: Overline) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_overline_new(overline.into_glib())) }
    }

    #[doc(alias = "pango_attr_rise_new")]
    pub fn new_rise(rise: i32) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_rise_new(rise)) }
    }

    #[doc(alias = "pango_attr_scale_new")]
    pub fn new_scale(scale_factor: f64) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_scale_new(scale_factor)) }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_show_new")]
    pub fn new_show(flags: ShowFlags) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_show_new(flags.into_glib())) }
    }

    #[doc(alias = "pango_attr_size_new")]
    pub fn new_size(size: i32) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_size_new(size)) }
    }

    #[doc(alias = "pango_attr_size_new_absolute")]
    pub fn new_size_absolute(size: i32) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_size_new_absolute(size)) }
    }

    #[doc(alias = "pango_attr_stretch_new")]
    pub fn new_stretch(stretch: Stretch) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_stretch_new(stretch.into_glib())) }
    }

    #[doc(alias = "pango_attr_strikethrough_color_new")]
    pub fn new_strikethrough_color(red: u16, green: u16, blue: u16) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_strikethrough_color_new(red, green, blue)) }
    }

    #[doc(alias = "pango_attr_strikethrough_new")]
    pub fn new_strikethrough(strikethrough: bool) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_strikethrough_new(strikethrough.into_glib())) }
    }

    #[doc(alias = "pango_attr_style_new")]
    pub fn new_style(style: Style) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_style_new(style.into_glib())) }
    }

    #[doc(alias = "pango_attr_underline_color_new")]
    pub fn new_underline_color(red: u16, green: u16, blue: u16) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_underline_color_new(red, green, blue)) }
    }

    #[doc(alias = "pango_attr_underline_new")]
    pub fn new_underline(underline: Underline) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_underline_new(underline.into_glib())) }
    }

    #[doc(alias = "pango_attr_variant_new")]
    pub fn new_variant(variant: Variant) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_variant_new(variant.into_glib())) }
    }

    #[doc(alias = "pango_attr_weight_new")]
    pub fn new_weight(weight: Weight) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_weight_new(weight.into_glib())) }
    }

    #[doc(alias = "pango_attr_shape_new")]
    pub fn new_shape(ink_rect: &Rectangle, logical_rect: &Rectangle) -> Self {
        unsafe {
            from_glib_full(ffi::pango_attr_shape_new(
                ink_rect.to_glib_none().0,
                logical_rect.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "get_attr_class")]
    pub fn attr_class(&self) -> AttrClass {
        unsafe { from_glib_full((*self.to_glib_none().0).klass) }
    }

    pub fn type_(&self) -> AttrType {
        unsafe { from_glib((*(*self.to_glib_none().0).klass).type_) }
    }

    #[doc(alias = "get_start_index")]
    pub fn start_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).start_index
        }
    }

    #[doc(alias = "get_end_index")]
    pub fn end_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).end_index
        }
    }

    pub fn set_start_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).start_index = index;
        }
    }

    pub fn set_end_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).end_index = index;
        }
    }
}

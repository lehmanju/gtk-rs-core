// This file was generated by gir (3c73dd9) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_10")]
use Adjustment;
use Container;
#[cfg(feature = "v3_10")]
use ListBoxRow;
use MovementStep;
use SelectionMode;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ListBox(Object<ffi::GtkListBox>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_list_box_get_type(),
    }
}

impl ListBox {
    #[cfg(feature = "v3_10")]
    pub fn new() -> ListBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_box_new()).downcast_unchecked()
        }
    }
}

pub trait ListBoxExt {
    //#[cfg(feature = "v3_16")]
    //fn bind_model<'a, 'b, P: IsA</*Ignored*/gio::ListModel> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b /*Unimplemented*/ListBoxCreateWidgetFunc>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, model: Q, create_widget_func: R, user_data: S, user_data_free_func: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    #[cfg(feature = "v3_10")]
    fn drag_highlight_row(&self, row: &ListBoxRow);

    #[cfg(feature = "v3_10")]
    fn drag_unhighlight_row(&self);

    #[cfg(feature = "v3_10")]
    fn get_activate_on_single_click(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_adjustment(&self) -> Option<Adjustment>;

    #[cfg(feature = "v3_10")]
    fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow>;

    #[cfg(feature = "v3_10")]
    fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow>;

    #[cfg(feature = "v3_10")]
    fn get_selected_row(&self) -> Option<ListBoxRow>;

    #[cfg(feature = "v3_14")]
    fn get_selected_rows(&self) -> Vec<ListBoxRow>;

    #[cfg(feature = "v3_10")]
    fn get_selection_mode(&self) -> SelectionMode;

    #[cfg(feature = "v3_10")]
    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32);

    #[cfg(feature = "v3_10")]
    fn invalidate_filter(&self);

    #[cfg(feature = "v3_10")]
    fn invalidate_headers(&self);

    #[cfg(feature = "v3_10")]
    fn invalidate_sort(&self);

    #[cfg(feature = "v3_10")]
    fn prepend<P: IsA<Widget>>(&self, child: &P);

    #[cfg(feature = "v3_14")]
    fn select_all(&self);

    #[cfg(feature = "v3_10")]
    fn select_row<'a, P: Into<Option<&'a ListBoxRow>>>(&self, row: P);

    //#[cfg(feature = "v3_14")]
    //fn selected_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/ListBoxForeachFunc, data: P);

    #[cfg(feature = "v3_10")]
    fn set_activate_on_single_click(&self, single: bool);

    #[cfg(feature = "v3_10")]
    fn set_adjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, adjustment: P);

    //#[cfg(feature = "v3_10")]
    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, filter_func: P, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //#[cfg(feature = "v3_10")]
    //fn set_header_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxUpdateHeaderFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, update_header: P, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    #[cfg(feature = "v3_10")]
    fn set_placeholder<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, placeholder: Q);

    #[cfg(feature = "v3_10")]
    fn set_selection_mode(&self, mode: SelectionMode);

    //#[cfg(feature = "v3_10")]
    //fn set_sort_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxSortFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, sort_func: P, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    #[cfg(feature = "v3_14")]
    fn unselect_all(&self);

    #[cfg(feature = "v3_14")]
    fn unselect_row(&self, row: &ListBoxRow);

    fn get_property_activate_on_single_click(&self) -> bool;

    fn set_property_activate_on_single_click(&self, activate_on_single_click: bool);

    fn get_property_selection_mode(&self) -> SelectionMode;

    fn set_property_selection_mode(&self, selection_mode: SelectionMode);

    fn connect_activate_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_10")]
    fn connect_row_activated<F: Fn(&Self, &ListBoxRow) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_10")]
    fn connect_row_selected<F: Fn(&Self, &Option<ListBoxRow>) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_selected_rows_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_toggle_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ListBox> + IsA<glib::object::Object>> ListBoxExt for O {
    //#[cfg(feature = "v3_16")]
    //fn bind_model<'a, 'b, P: IsA</*Ignored*/gio::ListModel> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b /*Unimplemented*/ListBoxCreateWidgetFunc>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, model: Q, create_widget_func: R, user_data: S, user_data_free_func: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_bind_model() }
    //}

    #[cfg(feature = "v3_10")]
    fn drag_highlight_row(&self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_drag_highlight_row(self.to_glib_none().0, row.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn drag_unhighlight_row(&self) {
        unsafe {
            ffi::gtk_list_box_drag_unhighlight_row(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_get_activate_on_single_click(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_adjustment(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_index(self.to_glib_none().0, index_))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_y(self.to_glib_none().0, y))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_selected_row(&self) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_selected_row(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_selected_rows(&self) -> Vec<ListBoxRow> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_list_box_get_selected_rows(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_list_box_get_selection_mode(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::gtk_list_box_insert(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v3_10")]
    fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_filter(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn invalidate_headers(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_headers(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_sort(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn prepend<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_list_box_prepend(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    fn select_all(&self) {
        unsafe {
            ffi::gtk_list_box_select_all(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn select_row<'a, P: Into<Option<&'a ListBoxRow>>>(&self, row: P) {
        let row = row.into();
        let row = row.to_glib_none();
        unsafe {
            ffi::gtk_list_box_select_row(self.to_glib_none().0, row.0);
        }
    }

    //#[cfg(feature = "v3_14")]
    //fn selected_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/ListBoxForeachFunc, data: P) {
    //    unsafe { TODO: call ffi::gtk_list_box_selected_foreach() }
    //}

    #[cfg(feature = "v3_10")]
    fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_list_box_set_activate_on_single_click(self.to_glib_none().0, single.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_adjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, adjustment: P) {
        let adjustment = adjustment.into();
        let adjustment = adjustment.to_glib_none();
        unsafe {
            ffi::gtk_list_box_set_adjustment(self.to_glib_none().0, adjustment.0);
        }
    }

    //#[cfg(feature = "v3_10")]
    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, filter_func: P, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_set_filter_func() }
    //}

    //#[cfg(feature = "v3_10")]
    //fn set_header_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxUpdateHeaderFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, update_header: P, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_set_header_func() }
    //}

    #[cfg(feature = "v3_10")]
    fn set_placeholder<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, placeholder: Q) {
        let placeholder = placeholder.into();
        let placeholder = placeholder.to_glib_none();
        unsafe {
            ffi::gtk_list_box_set_placeholder(self.to_glib_none().0, placeholder.0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_list_box_set_selection_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    //#[cfg(feature = "v3_10")]
    //fn set_sort_func<'a, P: Into<Option<&'a /*Unimplemented*/ListBoxSortFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, sort_func: P, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_list_box_set_sort_func() }
    //}

    #[cfg(feature = "v3_14")]
    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_list_box_unselect_all(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    fn unselect_row(&self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_unselect_row(self.to_glib_none().0, row.to_glib_none().0);
        }
    }

    fn get_property_activate_on_single_click(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "activate-on-single-click".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_activate_on_single_click(&self, activate_on_single_click: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "activate-on-single-click".to_glib_none().0, Value::from(&activate_on_single_click).to_glib_none().0);
        }
    }

    fn get_property_selection_mode(&self) -> SelectionMode {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "selection-mode".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_selection_mode(&self, selection_mode: SelectionMode) {
        let selection_mode = selection_mode.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "selection-mode".to_glib_none().0, Value::from(&selection_mode).to_glib_none().0);
        }
    }

    fn connect_activate_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-cursor-row",
                transmute(activate_cursor_row_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, MovementStep, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-cursor",
                transmute(move_cursor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_10")]
    fn connect_row_activated<F: Fn(&Self, &ListBoxRow) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &ListBoxRow) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-activated",
                transmute(row_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_10")]
    fn connect_row_selected<F: Fn(&Self, &Option<ListBoxRow>) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Option<ListBoxRow>) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-selected",
                transmute(row_selected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-all",
                transmute(select_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_selected_rows_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selected-rows-changed",
                transmute(selected_rows_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_toggle_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-cursor-row",
                transmute(toggle_cursor_row_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "unselect-all",
                transmute(unselect_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_cursor_row_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ListBox::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn move_cursor_trampoline<P>(this: *mut ffi::GtkListBox, object: ffi::GtkMovementStep, p0: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    callback_guard!();
    let f: &Box_<Fn(&P, MovementStep, i32) + 'static> = transmute(f);
    f(&ListBox::from_glib_none(this).downcast_unchecked(), from_glib(object), p0)
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn row_activated_trampoline<P>(this: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    callback_guard!();
    let f: &Box_<Fn(&P, &ListBoxRow) + 'static> = transmute(f);
    f(&ListBox::from_glib_none(this).downcast_unchecked(), &from_glib_none(row))
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn row_selected_trampoline<P>(this: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Option<ListBoxRow>) + 'static> = transmute(f);
    f(&ListBox::from_glib_none(this).downcast_unchecked(), &from_glib_none(row))
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn select_all_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ListBox::from_glib_none(this).downcast_unchecked())
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn selected_rows_changed_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ListBox::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn toggle_cursor_row_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ListBox::from_glib_none(this).downcast_unchecked())
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn unselect_all_trampoline<P>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ListBox::from_glib_none(this).downcast_unchecked())
}

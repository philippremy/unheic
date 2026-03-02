use std::cell::RefCell;

thread_local! {
    static UNHEIC_THREAD_NAME: RefCell<Option<String>> = const { RefCell::new(None) };
}

#[inline]
pub(crate) fn set_thread_name(name: impl Into<String>) {
    UNHEIC_THREAD_NAME.set(Some(name.into()));
}

#[inline]
pub(crate) fn get_thread_name() -> Option<String> {
    let name = UNHEIC_THREAD_NAME.with(|value| return value.clone());
    name.borrow().clone()
}

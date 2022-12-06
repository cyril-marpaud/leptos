#[cfg(all(target_arch = "wasm32", feature = "web"))]
use crate::Mountable;
use crate::{Comment, CoreComponent, IntoView, View};
use wasm_bindgen::JsCast;

/// The internal representation of the [`Unit`] core-component.
#[derive(Debug)]
pub struct UnitRepr {
  comment: Comment,
}

impl Default for UnitRepr {
  fn default() -> Self {
    Self {
      comment: Comment::new("<() />"),
    }
  }
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
impl Mountable for UnitRepr {
  fn get_mountable_node(&self) -> web_sys::Node {
    self.comment.node.clone().unchecked_into()
  }

  fn get_opening_node(&self) -> web_sys::Node {
    self.comment.node.clone().unchecked_into()
  }
}

/// The unit `()` leptos counterpart.
#[derive(Clone, Copy, Debug)]
pub struct Unit;

impl IntoView for Unit {
  #[cfg_attr(
    debug_assertions,
    instrument(level = "trace", name = "<() />", skip_all)
  )]
  fn into_view(self, _: leptos_reactive::Scope) -> crate::View {
    let component = UnitRepr::default();

    View::CoreComponent(CoreComponent::Unit(component))
  }
}
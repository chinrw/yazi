use std::time::Duration;

use mlua::{FromLua, Lua, MultiValue, Table, Value};

use super::Rect;
use crate::RtRef;

static WARNED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[inline]
fn warn_deprecated(id: Option<&str>) {
	if !WARNED.swap(true, std::sync::atomic::Ordering::Relaxed) {
		let id = match id {
			Some(id) => format!("`{id}.yazi` plugin"),
			None => "`init.lua` config".to_owned(),
		};
		let s = "The `ui.Paragraph` and `ui.ListItem` elements have been deprecated in Yazi v0.4.

Please use the new `ui.Text` instead, in your {id}. See https://github.com/sxyazi/yazi/pull/1776 for details.";
		yazi_proxy::AppProxy::notify(yazi_proxy::options::NotifyOpt {
			title:   "Deprecated API".to_owned(),
			content: s.replace("{id}", &id),
			level:   yazi_proxy::options::NotifyLevel::Warn,
			timeout: Duration::from_secs(20),
		});
	}
}

// TODO: remove this after v0.4 release
#[derive(Clone, Default, FromLua)]
pub struct Paragraph;

impl Paragraph {
	pub fn install(lua: &Lua, ui: &Table) -> mlua::Result<()> {
		let mt = lua.create_table_from([
			(
				"__call",
				lua.create_function(|lua, (_, area, lines): (Table, Rect, Value)| {
					warn_deprecated(lua.named_registry_value::<RtRef>("rt")?.current());
					lua
						.load(mlua::chunk! {
							return ui.Text($lines):area($area)
						})
						.call::<_, MultiValue>(())
				})?,
			),
			(
				"__index",
				lua.create_function(|lua, (_, key): (Table, mlua::String)| {
					warn_deprecated(lua.named_registry_value::<RtRef>("rt")?.current());
					lua
						.load(mlua::chunk! {
							return ui.Text[$key]
						})
						.call::<_, MultiValue>(())
				})?,
			),
		])?;

		let paragraph = lua.create_table()?;
		paragraph.set_metatable(Some(mt));

		ui.raw_set("Paragraph", paragraph)
	}
}

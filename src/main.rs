/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod toast_config;
mod level_select;
mod position_select;

use leptos::*;
use leptoaster::*;
use crate::toast_config::ToastConfig;

fn main() {
	mount_to_body(|| view! {
		<App />
	});
}

#[component]
fn App() -> impl IntoView {
	console_error_panic_hook::set_once();

	provide_toaster();

	view! {
		<Toaster />
		<ToastConfig />
	}
}

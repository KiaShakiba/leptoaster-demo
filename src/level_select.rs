/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::*;
use leptoaster::*;

#[component]
pub fn LevelSelect(
	level: ReadSignal<ToastLevel>,
	set_level: WriteSignal<ToastLevel>,
) -> impl IntoView {
	view! {
		<select on:change=move |ev| {
			let new_value = match event_target_value(&ev).as_str() {
				"info" => ToastLevel::Info,
				"success" => ToastLevel::Success,
				"warn" => ToastLevel::Warn,
				"error" => ToastLevel::Error,
				_ => unreachable!(),
			};

			set_level(new_value);
		}>
			<option
				value="info"
				selected=move || level() == ToastLevel::Info
			>
				"Info"
			</option>

			<option
				value="success"
				selected=move || level() == ToastLevel::Success
			>
				"Success"
			</option>

			<option
				value="warn"
				selected=move || level() == ToastLevel::Warn
			>
				"Warn"
			</option>

			<option
				value="error"
				selected=move || level() == ToastLevel::Error
			>
				"Error"
			</option>
		</select>
	}
}

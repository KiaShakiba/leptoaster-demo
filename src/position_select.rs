/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::prelude::*;
use leptoaster::*;

#[component]
pub fn PositionSelect(
	position: ReadSignal<ToastPosition>,
	set_position: WriteSignal<ToastPosition>,
) -> impl IntoView {
	view! {
		<select on:change=move |ev| {
			let new_value = match event_target_value(&ev).as_str() {
				"top_left" => ToastPosition::TopLeft,
				"top_right" => ToastPosition::TopRight,
				"bottom_right" => ToastPosition::BottomRight,
				"bottom_left" => ToastPosition::BottomLeft,
				_ => unreachable!(),
			};

			set_position.set(new_value);
		}>
			<option
				value="top_left"
				prop:selected=move || position.get() == ToastPosition::TopLeft
			>
				"Top left"
			</option>

			<option
				value="top_right"
				prop:selected=move || position.get() == ToastPosition::TopRight
			>
				"Top right"
			</option>

			<option
				value="bottom_right"
				prop:selected=move || position.get() == ToastPosition::BottomRight
			>
				"Bottom right"
			</option>

			<option
				value="bottom_left"
				prop:selected=move || position.get() == ToastPosition::BottomLeft
			>
				"Bottom left"
			</option>
		</select>
	}
}

/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::*;
use leptoaster::*;
use crate::level_select::LevelSelect;
use crate::position_select::PositionSelect;

#[component]
pub fn ToastConfig() -> impl IntoView {
	let toaster = expect_toaster();

	let (message, set_message) = create_signal(String::from("Toast message"));
	let (expiry, set_expiry) = create_signal(2500);
	let (dismissable, set_dismissable) = create_signal(true);
	let (expiry_enabled, set_expiry_enabled) = create_signal(true);
	let (progress_enabled, set_progress_enabled) = create_signal(true);
	let (level, set_level) = create_signal(ToastLevel::Success);
	let (position, set_position) = create_signal(ToastPosition::BottomLeft);

	let show_toast = move |()| {
		let message = match message().as_str() {
			"" => String::from("Toast message"),
			message => message.to_owned(),
		};

		let expiry = match expiry_enabled() {
			true => Some(expiry()),
			false => None,
		};

		toaster.toast(
			ToastBuilder::new(&message)
				.with_level(level())
				.with_dismissable(dismissable())
				.with_expiry(expiry)
				.with_progress(progress_enabled())
				.with_position(position())
		);
	};

	view! {
		<div class="container">
			<h1>"Leptoaster"</h1>
			<h2>"v0.1.5"</h2>

			<input type="text"
				on:change=move |ev| {
					set_message(event_target_value(&ev));
				}
				prop:value=message
				prop:placeholder="Toast message"
			/>

			<input type="number"
				on:change=move |ev| {
					if let Ok(value) = event_target_value(&ev).parse::<u32>() {
						set_expiry(value);
					}
				}
				prop:value=expiry
				prop:placeholder="Expiry (ms)"
				prop:disabled=move || !expiry_enabled()
			/>

			<label for="expiry-enabled">"Expiry"</label>

			<input type="checkbox"
				on:change=move |ev| {
					set_expiry_enabled(event_target_checked(&ev));
				}
				prop:id="expiry-enabled"
				prop:checked=expiry_enabled
			/>

			<label for="progress-enabled">"Progress"</label>

			<input type="checkbox"
				on:change=move |ev| {
					set_progress_enabled(event_target_checked(&ev));
				}
				prop:id="progress-enabled"
				prop:checked=progress_enabled
				prop:disabled=move || !expiry_enabled()
			/>

			<label for="dismissable">"Dismissable"</label>

			<input type="checkbox"
				on:change=move |ev| {
					set_dismissable(event_target_checked(&ev));
				}
				prop:id="dismissable"
				prop:checked=dismissable
			/>

			<LevelSelect
				level=level
				set_level=set_level
			/>

			<PositionSelect
				position=position
				set_position=set_position
			/>

			<button
				on:click=move |_| show_toast(())
				prop:disabled=move || message().is_empty()
			>
				"Toast"
			</button>
		</div>
	}
}

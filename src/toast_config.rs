/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::prelude::*;
use leptoaster::*;
use crate::level_select::LevelSelect;
use crate::position_select::PositionSelect;

#[component]
pub fn ToastConfig() -> impl IntoView {
	let (message, set_message) = signal(String::from("Toast message"));
	let (expiry, set_expiry) = signal(2500);
	let (dismissable, set_dismissable) = signal(true);
	let (expiry_enabled, set_expiry_enabled) = signal(true);
	let (progress_enabled, set_progress_enabled) = signal(true);
	let (level, set_level) = signal(ToastLevel::Success);
	let (position, set_position) = signal(ToastPosition::BottomLeft);
	let (stacked, set_stacked) = signal(false);

	let toaster = expect_toaster();
	let show_toast = move |_| {
		let message = match message.get().as_str() {
			"" => String::from("Toast message"),
			message => message.to_owned(),
		};

		let expiry = match expiry_enabled.get() {
			true => Some(expiry.get()),
			false => None,
		};

		toaster.toast(
			ToastBuilder::new(message)
				.with_level(level.get())
				.with_dismissable(dismissable.get())
				.with_expiry(expiry)
				.with_progress(progress_enabled.get())
				.with_position(position.get())
		);
	};

	let reset = move |_| {
		set_message.set(String::from("Toast message"));
		set_expiry.set(2500);
		set_dismissable.set(true);
		set_expiry_enabled.set(true);
		set_progress_enabled.set(true);
		set_level.set(ToastLevel::Success);
		set_position.set(ToastPosition::BottomLeft);
		set_stacked.set(false);
	};

	let toaster = expect_toaster();
	let clear = move |_| {
		toaster.clear();
	};

	view! {
		<Toaster stacked />

		<div class="container">
			<h1>"Leptoaster"</h1>
			<h2>"v0.2.2"</h2>

			<input type="text"
				on:change=move |ev| {
					set_message.set(event_target_value(&ev));
				}
				prop:value=message
				prop:placeholder="Toast message"
			/>

			<input type="number"
				on:change=move |ev| {
					if let Ok(value) = event_target_value(&ev).parse::<u32>() {
						set_expiry.set(value);
					}
				}
				prop:value=expiry
				prop:placeholder="Expiry (ms)"
				prop:disabled=move || !expiry_enabled.get()
			/>

			<div class="checkboxes">
				<label>
					<input type="checkbox"
						on:change=move |ev| {
							set_dismissable.set(event_target_checked(&ev));
						}
						prop:checked=dismissable
					/>

					"Dismissable"
				</label>

				<label>
					<input type="checkbox"
						on:change=move |ev| {
							set_progress_enabled.set(event_target_checked(&ev));
						}
						prop:checked=progress_enabled
						prop:disabled=move || !expiry_enabled.get()
					/>

					"Progress"
				</label>

				<label>
					<input type="checkbox"
						on:change=move |ev| {
							set_expiry_enabled.set(event_target_checked(&ev));
						}
						prop:checked=expiry_enabled
					/>

					"Expiry"
				</label>
			</div>

			<LevelSelect
				level=level
				set_level=set_level
			/>

			<PositionSelect
				position=position
				set_position=set_position
			/>

			<div class="checkboxes">
				<label>
					<input type="checkbox"
						on:change=move |ev| {
							set_stacked.set(event_target_checked(&ev));
						}
						prop:checked=stacked
					/>

					"Stacked"
				</label>
			</div>

			<div class="buttons">
				<button
					class="clear"
					on:click=clear
					prop:disabled=move || message.get().is_empty()
				>
					"Clear"
				</button>

				<button
					class="reset"
					on:click=reset
					prop:disabled=move || message.get().is_empty()
				>
					"Reset"
				</button>

				<button
					class="submit"
					on:click=show_toast
					prop:disabled=move || message.get().is_empty()
				>
					"Toast"
				</button>
			</div>
		</div>
	}
}

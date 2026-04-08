// Asus Hub - Unofficial Control Center for Asus Laptops
// Copyright (C) 2026 Guido Philipp
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see https://www.gnu.org/licenses/.

mod app;
mod autostart;
mod components;
mod search;
mod services;
mod tray;

use gtk4::gdk;

rust_i18n::i18n!("locales", fallback = "en");

const STYLE_CSS: &str = include_str!("../assets/style.css");

fn load_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(STYLE_CSS);
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    let config = services::config::AppConfig::load();
    rust_i18n::set_locale(&config.language);
    let a = relm4::RelmApp::new("de.guido.asus-hub");
    load_css();
    relm4::adw::StyleManager::default().set_color_scheme(relm4::adw::ColorScheme::PreferDark);
    let start_hidden = std::env::args().any(|arg| arg == "--hidden");
    a.run::<app::AppModel>(start_hidden);
}

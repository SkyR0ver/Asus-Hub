mod app;
mod components;
mod services;
mod tray;

rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    let config = services::config::AppConfig::load();
    rust_i18n::set_locale(&config.language);
    let a = relm4::RelmApp::new("de.guido.zenbook-control");
    relm4::adw::StyleManager::default().set_color_scheme(relm4::adw::ColorScheme::PreferDark);
    a.run::<app::AppModel>(());
}

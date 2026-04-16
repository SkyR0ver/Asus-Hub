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

use gtk4 as gtk;
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::prelude::*;
use rust_i18n::t;

use crate::services::commands::pkexec_read;

pub struct HomeModel {
    product_name_label: gtk::Label,
    board_row: adw::ActionRow,
    bios_row: adw::ActionRow,
    kernel_row: adw::ActionRow,
    serial_row: adw::ActionRow,
    reveal_button: gtk::Button,
    battery_text: String,
    cpu_text: String,
    ram_text: String,
    disk_text: String,
}

#[derive(Debug)]
pub enum HomeMsg {
    RevealSerial,
    RefreshMetrics,
}

#[derive(Debug)]
pub enum HomeCommandOutput {
    DataLoaded {
        product_name: String,
        board_name: String,
        bios_version: String,
        bios_date: String,
        kernel: String,
    },
    SerialRevealed(Result<String, String>),
    MetricsRefreshed {
        battery: String,
        cpu: String,
        ram: String,
        disk: String,
    },
}

#[relm4::component(pub)]
impl Component for HomeModel {
    type Init = ();
    type Input = HomeMsg;
    type Output = String;
    type CommandOutput = HomeCommandOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 24,
            set_margin_top: 24,
            set_margin_bottom: 32,
            set_margin_start: 32,
            set_margin_end: 32,

            append = &adw::PreferencesGroup {
                add = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 24,

                    append = &model.product_name_label.clone(),

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 32,

                        append = &gtk::Image {
                            set_icon_name: Some("computer-symbolic"),
                            set_pixel_size: 192,
                            set_valign: gtk::Align::Center,
                        },

                        append = &adw::PreferencesGroup {
                            set_valign: gtk::Align::Center,
                            set_hexpand: true,

                            add = &model.board_row.clone(),
                            add = &model.bios_row.clone(),
                            add = &model.kernel_row.clone(),
                            add = &model.serial_row.clone(),
                        },
                    },
                },
            },

            // System metrics dashboard
            append = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 16,
                set_homogeneous: true,

                // Battery card
                append = &gtk::Box {
                    add_css_class: "card",
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Start,

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        set_margin_top: 16,
                        set_margin_bottom: 16,
                        set_margin_start: 16,
                        set_margin_end: 16,

                        append = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,

                            append = &gtk::Image {
                                set_icon_name: Some("battery-symbolic"),
                                set_pixel_size: 16,
                            },
                            append = &gtk::Label {
                                set_label: "Battery",
                            },
                        },
                        append = &gtk::Label {
                            #[watch]
                            set_label: &model.battery_text,
                            add_css_class: "title-2",
                            add_css_class: "dim-label",
                            set_halign: gtk::Align::Start,
                        },
                    },
                },

                // CPU card
                append = &gtk::Box {
                    add_css_class: "card",
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Start,

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        set_margin_top: 16,
                        set_margin_bottom: 16,
                        set_margin_start: 16,
                        set_margin_end: 16,

                        append = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,

                            append = &gtk::Image {
                                set_icon_name: Some("system-run-symbolic"),
                                set_pixel_size: 16,
                            },
                            append = &gtk::Label {
                                set_label: "CPU",
                            },
                        },
                        append = &gtk::Label {
                            #[watch]
                            set_label: &model.cpu_text,
                            add_css_class: "title-2",
                            add_css_class: "dim-label",
                            set_halign: gtk::Align::Start,
                        },
                    },
                },

                // RAM card
                append = &gtk::Box {
                    add_css_class: "card",
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Start,

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        set_margin_top: 16,
                        set_margin_bottom: 16,
                        set_margin_start: 16,
                        set_margin_end: 16,

                        append = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,

                            append = &gtk::Image {
                                set_icon_name: Some("media-flash-symbolic"),
                                set_pixel_size: 16,
                            },
                            append = &gtk::Label {
                                set_label: "Memory",
                            },
                        },
                        append = &gtk::Label {
                            #[watch]
                            set_label: &model.ram_text,
                            add_css_class: "title-2",
                            add_css_class: "dim-label",
                            set_halign: gtk::Align::Start,
                        },
                    },
                },

                // Disk card
                append = &gtk::Box {
                    add_css_class: "card",
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Start,

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        set_margin_top: 16,
                        set_margin_bottom: 16,
                        set_margin_start: 16,
                        set_margin_end: 16,

                        append = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 8,

                            append = &gtk::Image {
                                set_icon_name: Some("drive-harddisk-symbolic"),
                                set_pixel_size: 16,
                            },
                            append = &gtk::Label {
                                set_label: "Disk",
                            },
                        },
                        append = &gtk::Label {
                            #[watch]
                            set_label: &model.disk_text,
                            add_css_class: "title-2",
                            add_css_class: "dim-label",
                            set_halign: gtk::Align::Start,
                        },
                    },
                },
            },

            // Profiles placeholder
            append = &adw::PreferencesGroup {
                set_title: &t!("home_profiles_title"),

                add = &gtk::Label {
                    set_label: &t!("home_profiles_placeholder"),
                    set_margin_top: 12,
                    set_margin_bottom: 12,
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let product_name_label = gtk::Label::new(Some(&t!("home_loading")));
        product_name_label.add_css_class("title-1");
        product_name_label.set_halign(gtk::Align::Start);

        let board_row = adw::ActionRow::new();
        board_row.set_title(&t!("home_board_title"));
        board_row.set_selectable(false);

        let bios_row = adw::ActionRow::new();
        bios_row.set_title(&t!("home_bios_title"));
        bios_row.set_selectable(false);

        let kernel_row = adw::ActionRow::new();
        kernel_row.set_title(&t!("home_kernel_title"));
        kernel_row.set_selectable(false);

        let serial_row = adw::ActionRow::new();
        serial_row.set_title(&t!("home_serial_title"));
        serial_row.set_subtitle(&t!("home_serial_hidden"));
        serial_row.set_selectable(false);

        let reveal_button = gtk::Button::with_label(&t!("home_serial_reveal"));
        reveal_button.set_valign(gtk::Align::Center);
        reveal_button.add_css_class("flat");
        {
            let sender = sender.clone();
            reveal_button.connect_clicked(move |_| {
                sender.input(HomeMsg::RevealSerial);
            });
        }
        serial_row.add_suffix(&reveal_button);

        let model = HomeModel {
            product_name_label,
            board_row,
            bios_row,
            kernel_row,
            serial_row,
            reveal_button,
            battery_text: "…".to_string(),
            cpu_text: "…".to_string(),
            ram_text: "…".to_string(),
            disk_text: "…".to_string(),
        };

        let widgets = view_output!();

        // Initial load of device info
        sender.command(move |out, shutdown| {
            shutdown
                .register(async move {
                    let product_name = tokio::fs::read_to_string("/sys/class/dmi/id/product_name")
                        .await
                        .map(|s| s.trim().to_string())
                        .unwrap_or_default();

                    let board_name = tokio::fs::read_to_string("/sys/class/dmi/id/board_name")
                        .await
                        .map(|s| s.trim().to_string())
                        .unwrap_or_default();

                    let bios_version = tokio::fs::read_to_string("/sys/class/dmi/id/bios_version")
                        .await
                        .map(|s| s.trim().to_string())
                        .unwrap_or_default();

                    let bios_date = tokio::fs::read_to_string("/sys/class/dmi/id/bios_date")
                        .await
                        .map(|s| s.trim().to_string())
                        .unwrap_or_default();

                    let kernel = tokio::process::Command::new("uname")
                        .arg("-r")
                        .output()
                        .await
                        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                        .unwrap_or_default();

                    out.emit(HomeCommandOutput::DataLoaded {
                        product_name,
                        board_name,
                        bios_version,
                        bios_date,
                        kernel,
                    });
                })
                .drop_on_shutdown()
        });

        // Fetch metrics immediately, then every 5 seconds
        sender.input(HomeMsg::RefreshMetrics);
        {
            let sender = sender.clone();
            gtk::glib::timeout_add_local(std::time::Duration::from_secs(5), move || {
                sender.input(HomeMsg::RefreshMetrics);
                gtk::glib::ControlFlow::Continue
            });
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: HomeMsg, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            HomeMsg::RevealSerial => {
                self.reveal_button.set_sensitive(false);
                sender.command(move |out, shutdown| {
                    shutdown
                        .register(async move {
                            let result = pkexec_read("cat /sys/class/dmi/id/product_serial").await;
                            out.emit(HomeCommandOutput::SerialRevealed(result));
                        })
                        .drop_on_shutdown()
                });
            }
            HomeMsg::RefreshMetrics => {
                sender.command(move |out, shutdown| {
                    shutdown
                        .register(async move {
                            let battery = {
                                let b0 = tokio::fs::read_to_string(
                                    "/sys/class/power_supply/BAT0/capacity",
                                )
                                .await;
                                let b1 = tokio::fs::read_to_string(
                                    "/sys/class/power_supply/BAT1/capacity",
                                )
                                .await;
                                b0.or(b1)
                                    .map(|s| format!("{}%", s.trim()))
                                    .unwrap_or_else(|_| "N/A".to_string())
                            };

                            let cpu = {
                                let load = tokio::fs::read_to_string("/proc/loadavg")
                                    .await
                                    .map(|s| {
                                        s.split_whitespace()
                                            .next()
                                            .unwrap_or("?")
                                            .to_string()
                                    })
                                    .unwrap_or_else(|_| "?".to_string());

                                let temp = tokio::fs::read_to_string(
                                    "/sys/class/thermal/thermal_zone0/temp",
                                )
                                .await
                                .map(|s| {
                                    let millideg: i32 = s.trim().parse().unwrap_or(0);
                                    format!("{}°C", millideg / 1000)
                                })
                                .unwrap_or_else(|_| "?°C".to_string());

                                format!("{}% | {}", load, temp)
                            };

                            let ram = tokio::fs::read_to_string("/proc/meminfo")
                                .await
                                .map(|s| {
                                    let mut total: u64 = 0;
                                    let mut available: u64 = 0;
                                    for line in s.lines() {
                                        if line.starts_with("MemTotal:") {
                                            total = line
                                                .split_whitespace()
                                                .nth(1)
                                                .and_then(|v| v.parse().ok())
                                                .unwrap_or(0);
                                        } else if line.starts_with("MemAvailable:") {
                                            available = line
                                                .split_whitespace()
                                                .nth(1)
                                                .and_then(|v| v.parse().ok())
                                                .unwrap_or(0);
                                        }
                                    }
                                    if total > 0 {
                                        format!("{}%", 100 * (total - available) / total)
                                    } else {
                                        "N/A".to_string()
                                    }
                                })
                                .unwrap_or_else(|_| "N/A".to_string());

                            let disk = tokio::process::Command::new("df")
                                .args(["-h", "/"])
                                .output()
                                .await
                                .map(|o| {
                                    let stdout = String::from_utf8_lossy(&o.stdout);
                                    stdout
                                        .lines()
                                        .nth(1)
                                        .and_then(|line| line.split_whitespace().nth(4))
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| "N/A".to_string())
                                })
                                .unwrap_or_else(|_| "N/A".to_string());

                            out.emit(HomeCommandOutput::MetricsRefreshed {
                                battery,
                                cpu,
                                ram,
                                disk,
                            });
                        })
                        .drop_on_shutdown()
                });
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: HomeCommandOutput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            HomeCommandOutput::DataLoaded {
                product_name,
                board_name,
                bios_version,
                bios_date,
                kernel,
            } => {
                self.product_name_label.set_label(&product_name);
                self.board_row.set_subtitle(&board_name);
                self.bios_row
                    .set_subtitle(&format!("{bios_version} / {bios_date}"));
                self.kernel_row.set_subtitle(&kernel);
            }
            HomeCommandOutput::SerialRevealed(Ok(serial)) => {
                self.serial_row.set_subtitle(&serial);
            }
            HomeCommandOutput::SerialRevealed(Err(e)) => {
                self.reveal_button.set_sensitive(true);
                let _ = sender.output(e);
            }
            HomeCommandOutput::MetricsRefreshed {
                battery,
                cpu,
                ram,
                disk,
            } => {
                self.battery_text = battery;
                self.cpu_text = cpu;
                self.ram_text = ram;
                self.disk_text = disk;
            }
        }
    }
}

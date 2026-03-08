//! ADAT-RS GUI: ayarlar paneli ve komut sekmeleri.

use crate::commands::{
    substitute, CommandCategory, CommandContext, external_categories, internal_categories,
    null_mode_categories,
};
use egui::{Color32, RichText, ScrollArea, TextEdit};

const DEFAULT_USERLIST: &str = "/usr/share/seclists/Usernames/Names/names.txt";
const DEFAULT_USERLIST_XATO: &str = "/usr/share/seclists/Usernames/xato-net-10-million-usernames.txt";

#[derive(Default)]
pub struct AdatApp {
    target_ip: String,
    domain: String,
    username: String,
    password: String,
    ldap_base: String,
    local_ip: String,
    local_port: String,
    userlist: String,
    userlist_xato: String,

    selected_tab: usize,
    copy_feedback: Option<String>,
    copy_feedback_time: Option<f64>,
}

impl AdatApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            local_ip: "10.10.14.10".to_string(),
            local_port: "8080".to_string(),
            userlist: DEFAULT_USERLIST.to_string(),
            userlist_xato: DEFAULT_USERLIST_XATO.to_string(),
            ..Default::default()
        }
    }

    fn ctx(&self) -> CommandContext {
        CommandContext {
            userlist: self.userlist.clone(),
            userlist_xato: self.userlist_xato.clone(),
            local_ip: self.local_ip.clone(),
            local_port: self.local_port.clone(),
            ldap_base: self.ldap_base.clone(),
            ..Default::default()
        }
        .build(
            self.target_ip.trim(),
            self.domain.trim(),
            self.username.trim(),
            self.password.trim(),
        )
    }

    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hedef ve kimlik bilgileri");
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Hedef IP:");
            ui.add(
                TextEdit::singleline(&mut self.target_ip)
                    .hint_text("10.10.10.100")
                    .desired_width(140.0),
            );
            ui.label("Domain:");
            ui.add(
                TextEdit::singleline(&mut self.domain)
                    .hint_text("Security.local")
                    .desired_width(140.0),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Kullanıcı:");
            ui.add(
                TextEdit::singleline(&mut self.username)
                    .hint_text("(Null modda boş bırak)")
                    .desired_width(140.0),
            );
            ui.label("Parola:");
            ui.add(
                TextEdit::singleline(&mut self.password)
                    .password(true)
                    .desired_width(140.0),
            );
        });

        ui.horizontal(|ui| {
            ui.label("LDAP base:");
            ui.add(
                TextEdit::singleline(&mut self.ldap_base)
                    .hint_text("DC=Security,DC=Local")
                    .desired_width(280.0),
            );
        });

        ui.collapsing("Yerel repo (internet yok)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Local IP:");
                ui.add(TextEdit::singleline(&mut self.local_ip).desired_width(100.0));
                ui.label("Port:");
                ui.add(TextEdit::singleline(&mut self.local_port).desired_width(60.0));
            });
        });

        ui.collapsing("Wordlist'ler", |ui| {
            ui.horizontal(|ui| {
                ui.label("Userlist:");
                ui.add(
                    TextEdit::singleline(&mut self.userlist)
                        .desired_width(320.0),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Userlist (xato):");
                ui.add(
                    TextEdit::singleline(&mut self.userlist_xato)
                        .desired_width(320.0),
                );
            });
        });
    }

    fn copy_to_clipboard(&mut self, text: &str, ctx_time: f64) {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            if clipboard.set_text(text).is_ok() {
                self.copy_feedback = Some("Panoya kopyalandı.".to_string());
                self.copy_feedback_time = Some(ctx_time);
            }
        }
    }

    fn render_command_list(
        &mut self,
        ui: &mut egui::Ui,
        categories: &[CommandCategory],
        ctx: &CommandContext,
        ctx_time: f64,
    ) {
        for cat in categories {
            ui.add_space(6.0);
            ui.heading(RichText::new(&cat.name).color(Color32::from_rgb(100, 180, 100)));
            for item in &cat.items {
                let cmd = substitute(ctx, &item.command);
                ui.horizontal(|ui| {
                    if ui.button("Kopyala").clicked() {
                        self.copy_to_clipboard(&cmd, ctx_time);
                    }
                    if !item.comment.is_empty() {
                        ui.label(RichText::new(format!("— {}", item.comment)).color(Color32::GRAY));
                    }
                    ui.monospace(&cmd);
                });
            }
        }
    }

    fn render_tab_commands(
        &mut self,
        ui: &mut egui::Ui,
        categories: Vec<CommandCategory>,
        tab_name: &str,
        ctx_time: f64,
    ) {
        let ctx = self.ctx();
        let all: String = categories
            .iter()
            .flat_map(|c| {
                c.items.iter().map(|i| substitute(&ctx, &i.command))
            })
            .collect::<Vec<_>>()
            .join("\n");

        ui.horizontal(|ui| {
            if ui.button(format!("Tümünü kopyala ({})", tab_name)).clicked() {
                self.copy_to_clipboard(&all, ctx_time);
            }
        });
        ui.add_space(4.0);

        ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
            self.render_command_list(ui, &categories, &ctx, ctx_time);
        });
    }
}

impl eframe::App for AdatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let t = ctx.input(|i| i.time);
        if let Some(tt) = self.copy_feedback_time {
            if self.copy_feedback.is_some() && t - tt > 2.0 {
                self.copy_feedback = None;
                self.copy_feedback_time = None;
            }
        }

        egui::TopBottomPanel::top("settings").show(ctx, |ui| {
            self.render_settings(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref msg) = self.copy_feedback {
                ui.colored_label(Color32::GREEN, msg);
            }

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, 0, "Null mod");
                ui.selectable_value(&mut self.selected_tab, 1, "Dış komutlar");
                ui.selectable_value(&mut self.selected_tab, 2, "İç komutlar");
            });
            ui.add_space(8.0);

            match self.selected_tab {
                0 => {
                    self.render_tab_commands(
                        ui,
                        null_mode_categories(),
                        "Null",
                        t,
                    );
                }
                1 => {
                    self.render_tab_commands(
                        ui,
                        external_categories(),
                        "Dış",
                        t,
                    );
                }
                2 => {
                    self.render_tab_commands(
                        ui,
                        internal_categories(),
                        "İç",
                        t,
                    );
                }
                _ => {}
            }
        });
    }
}

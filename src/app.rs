// Copyright 2025 Titouan Real <titouan.real@gmail.com>
// SPDX-License-Identifier: GPL-3.0

use crate::{
    backend::{Backend, ContactBackend},
    config::Config,
    fl,
    utils::Contact,
};
use cosmic::{
    app::{context_drawer, Core, Task},
    cosmic_config::{self, CosmicConfigEntry},
    cosmic_theme::Spacing,
    iced::{
        alignment::{Horizontal, Vertical},
        Alignment, Length, Subscription,
    },
    iced_core::text::Wrapping,
    iced_futures::backend::default::time,
    iced_widget::scrollable,
    theme,
    widget::{
        self, about::About, button, horizontal_space, icon, list_column, menu, settings, text,
    },
    Application, ApplicationExt, Apply, Element,
};
use std::{collections::HashMap, time::Duration};

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub enum ContactList {
    Loading,
    Loaded(Vec<(u64, Contact)>),
}

pub struct AppModel {
    core: Core,
    about: About,
    context_page: ContextPage,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    config: Config,
    contact_list: ContactList,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    None,
    Reload,
    ToggleContextPage(ContextPage),
    ToggleContextDrawer,
    UpdateConfig(Config),
    LaunchUrl(String),
    LoadPage(Vec<(u64, Contact)>),
}

/// Create a COSMIC application from the app model
impl Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.github.TitouanReal.Contacts";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let about = About::default()
            .name(fl!("app-title"))
            // TODO: Add icon
            // .icon(Self::APP_ID)
            .author("Titouan Real")
            .version(env!("CARGO_PKG_VERSION"))
            .license("GPL-3.0")
            .links([
                (fl!("support"), format!("{REPOSITORY}/issues").as_str()),
                (fl!("repository"), REPOSITORY.into()),
            ])
            .developers([("Titouan Real", "titouan.real@gmail.com")]);

        let mut app = AppModel {
            core,
            about,
            context_page: ContextPage::default(),
            key_binds: HashMap::new(),
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => config,
                })
                .unwrap_or_default(),
            contact_list: ContactList::Loading,
        };

        let command = Task::batch([app.update_title(), Task::future(Self::reload())]);

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), None, MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page.clone() {
            ContextPage::About => context_drawer::about(
                &self.about,
                Message::LaunchUrl,
                Message::ToggleContextDrawer,
            )
            .title(fl!("about")),
            ContextPage::ContactDetail((id, contact)) => context_drawer::context_drawer(
                Self::contact_detail(id, contact.clone()),
                Message::ToggleContextPage(ContextPage::ContactDetail((id, contact.clone()))),
            )
            .title(fl!("contact-details")),
        })
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.contact_list {
            ContactList::Loading => widget::text::title1("Loading contacts...")
                .apply(widget::container)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into(),
            ContactList::Loaded(contacts) => {
                let mut list = list_column();
                for (id, contact) in contacts {
                    list = list.add(
                        settings::item_row(vec![
                            text::body(contact.name.clone())
                                .wrapping(Wrapping::Word)
                                .into(),
                            horizontal_space().into(),
                            icon::from_name("go-next-symbolic").size(16).icon().into(),
                        ])
                        .apply(widget::container)
                        .class(cosmic::theme::Container::List)
                        .apply(button::custom)
                        .class(theme::Button::Transparent)
                        .on_press(Message::ToggleContextPage(ContextPage::ContactDetail((
                            *id,
                            contact.clone(),
                        )))),
                    );
                }
                list.apply(scrollable).into()
            }
        }
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            time::every(Duration::from_secs(2)).map(|_| Message::Reload),
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::None => {}

            Message::Reload => return Task::future(Self::reload()),

            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }
            }

            Message::ToggleContextDrawer => {
                self.core.window.show_context = !self.core.window.show_context;
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("failed to open {url:?}: {err}");
                }
            },

            Message::LoadPage(contacts) => self.contact_list = ContactList::Loaded(contacts),
        }
        Task::none()
    }
}

impl AppModel {
    pub fn contact_detail(_id: u64, contact: Contact) -> Element<'static, Message> {
        let Spacing { space_xxs, .. } = cosmic::theme::active().cosmic().spacing;

        let name = widget::text::title3(contact.name);

        // let id = widget::text(format!("id: {}", id));

        let mut mails = widget::settings::section().title(fl!("emails"));

        for mail in contact.mails {
            mails = mails.add(widget::text(mail.address))
        }

        let mut phones = widget::settings::section().title(fl!("phones"));

        for phone in contact.phones {
            phones = phones.add(widget::text(phone.number))
        }

        widget::column()
            .push(name)
            // .push(id)
            .push(mails)
            .push(phones)
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<Message> {
        let window_title = fl!("app-title");

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }

    pub async fn reload() -> cosmic::app::Message<Message> {
        cosmic::app::Message::App(match Backend::get_contacts().await {
            Ok(contacts) => Message::LoadPage(contacts),
            Err(()) => Message::None,
        })
    }
}

/// The context page to display in the context drawer.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
    ContactDetail((u64, Contact)),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}

use std::{
    env,
    io::{self, BufRead},
};

use xcb::{x, Xid};
use xkbcommon::xkb;

mod config;
mod draw;
mod parts;

struct Rmenu {
    connection: xcb::Connection,
    window: parts::Window,
    draw: draw::Draw,
    prompt: parts::Prompt,
    text_field: parts::TextField,
    search_field: parts::SearchField,
    state: xkb::State,
}

impl Rmenu {
    fn new(
        connection: xcb::Connection,
        window: parts::Window,
        draw: draw::Draw,
        prompt: parts::Prompt,
        text_field: parts::TextField,
        search_field: parts::SearchField,
        state: xkb::State,
    ) -> Self {
        Self {
            connection,
            window,
            draw,
            prompt,
            text_field,
            search_field,
            state,
        }
    }

    fn run(&mut self) {
        self.draw_prompt();
        self.draw_text_field();
        self.draw_items();
        self.connection.flush().unwrap();
        self.grab_keyboard();
        self.event_loop();
    }

    fn draw_prompt(&self) {
        let draw = &self.draw;
        draw.rectangle(0, self.prompt.width, config::HL_COLOR);
        draw.text(0, &self.prompt.text, config::TEXT_HL_COLOR);
    }

    fn draw_text_field(&mut self) {
        let draw = &self.draw;
        let text = &format!("{}|", self.text_field.text);

        if draw.text_size(text).0 as u16 + config::MARGIN * 2 > self.text_field.width {
            self.text_field.text.pop();
            return;
        }

        draw.rectangle(self.prompt.width, self.text_field.width, config::BG_COLOR);
        draw.text(self.prompt.width, text, config::TEXT_COLOR);
    }

    fn draw_items(&mut self) {
        let draw = &self.draw;
        self.search_field.cursor = 0;

        let mut items: Vec<parts::Item> = self
            .search_field
            .all_items
            .iter()
            .filter(|item| item.text.contains(&self.text_field.text))
            .cloned()
            .collect();
        items.sort_by(|x, y| ord(x, y, &self.text_field.text));

        self.search_field.items = items.clone();

        let mut position = self.prompt.width + self.text_field.width;
        draw.rectangle(position, self.search_field.width, config::BG_COLOR);

        let mut visited = false;
        for item in items {
            if position + item.width > self.window.width {
                break;
            }

            let box_color;
            let text_color;

            if !visited {
                visited = true;
                box_color = config::HL_COLOR;
                text_color = config::TEXT_HL_COLOR;
            } else {
                box_color = config::BG_COLOR;
                text_color = config::TEXT_COLOR;
            }

            draw.rectangle(position, item.width, box_color);
            draw.text(position, &item.text, text_color);

            position += item.width;
        }
    }

    fn grab_keyboard(&self) {
        while self
            .connection
            .wait_for_reply(self.connection.send_request(&x::GrabKeyboard {
                owner_events: true,
                grab_window: self.window.id,
                time: x::CURRENT_TIME,
                pointer_mode: x::GrabMode::Async,
                keyboard_mode: x::GrabMode::Async,
            }))
            .unwrap()
            .status()
            != x::GrabStatus::Success
        {}
    }

    fn event_loop(&mut self) {
        loop {
            match self.connection.wait_for_event().unwrap() {
                xcb::Event::X(x::Event::KeyPress(key_press)) => {
                    let detail = key_press.detail();

                    let keysym = self.state.key_get_one_sym(detail.into());

                    match keysym {
                        xkb::Keysym::Escape => return,
                        xkb::Keysym::Return => {
                            print!("{}", self.search_field.get_selection());
                            return;
                        }
                        xkb::Keysym::Left => {
                            self.move_cursor_left();
                            self.connection.flush().unwrap();
                            continue;
                        }
                        xkb::Keysym::Right => {
                            self.move_cursor_right();
                            self.connection.flush().unwrap();
                            continue;
                        }
                        xkb::Keysym::BackSpace => {
                            self.text_field.text.pop();
                        }
                        k => {
                            let Some(c) = k.key_char() else {
                                continue;
                            };

                            self.text_field.text.push(c);
                        }
                    }

                    self.draw_text_field();
                    self.draw_items();
                }
                xcb::Event::Xkb(xcb::xkb::Event::StateNotify(state_notify)) => {
                    self.state.update_mask(
                        state_notify.base_mods().bits() as xkb::ModMask,
                        state_notify.latched_mods().bits() as xkb::ModMask,
                        state_notify.locked_mods().bits() as xkb::ModMask,
                        state_notify.base_group() as xkb::LayoutIndex,
                        state_notify.latched_group() as xkb::LayoutIndex,
                        state_notify.locked_group() as xkb::LayoutIndex,
                    );
                }
                _ => {}
            }

            self.connection.flush().unwrap();
        }
    }

    fn move_cursor_left(&mut self) {
        let draw = &self.draw;
        let cursor = self.search_field.cursor;
        let indent = self.prompt.width + self.text_field.width;

        if self.search_field.items.is_empty() || cursor == 0 {
            return;
        }

        let old_item = self.search_field.items.get(cursor).unwrap();
        let new_item = self.search_field.items.get(cursor - 1).unwrap();

        let old_position = self
            .search_field
            .items
            .iter()
            .take(cursor)
            .map(|item| item.width)
            .sum::<u16>()
            + indent;

        let new_position = old_position - new_item.width;

        draw.rectangle(old_position, old_item.width, config::BG_COLOR);
        draw.rectangle(new_position, new_item.width, config::HL_COLOR);

        draw.text(old_position, &old_item.text, config::TEXT_COLOR);
        draw.text(new_position, &new_item.text, config::TEXT_HL_COLOR);

        self.search_field.cursor -= 1;
    }

    fn move_cursor_right(&mut self) {
        let draw = &self.draw;
        let cursor = self.search_field.cursor;
        let indent = self.prompt.width + self.text_field.width;

        if self.search_field.items.is_empty() || cursor == self.search_field.items.len() - 1 {
            return;
        }

        let old_item = self.search_field.items.get(cursor).unwrap();
        let new_item = self.search_field.items.get(cursor + 1).unwrap();

        let old_position = self
            .search_field
            .items
            .iter()
            .take(cursor)
            .map(|item| item.width)
            .sum::<u16>()
            + indent;

        let new_position = old_position + old_item.width;

        if new_position + new_item.width > self.window.width {
            return;
        }

        draw.rectangle(old_position, old_item.width, config::BG_COLOR);
        draw.rectangle(new_position, new_item.width, config::HL_COLOR);

        draw.text(old_position, &old_item.text, config::TEXT_COLOR);
        draw.text(new_position, &new_item.text, config::TEXT_HL_COLOR);

        self.search_field.cursor += 1;
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    let items = io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let (connection, screen_num) =
        xcb::Connection::connect_with_extensions(None, &[xcb::Extension::Xkb], &[]).unwrap();

    {
        let xkbver = connection
            .wait_for_reply(connection.send_request(&xcb::xkb::UseExtension {
                wanted_major: xkb::x11::MIN_MAJOR_XKB_VERSION,
                wanted_minor: xkb::x11::MIN_MINOR_XKB_VERSION,
            }))
            .unwrap();

        assert!(
            xkbver.supported(),
            "required xcb-xkb-{}-{} is not supported",
            xkb::x11::MIN_MAJOR_XKB_VERSION,
            xkb::x11::MIN_MINOR_XKB_VERSION
        );
    }

    let events = xcb::xkb::EventType::STATE_NOTIFY;
    let map_parts = xcb::xkb::MapPart::empty();

    connection
        .check_request(connection.send_request_checked(&xcb::xkb::SelectEvents {
            device_spec: xcb::xkb::Id::UseCoreKbd as xcb::xkb::DeviceSpec,
            affect_which: events,
            clear: xcb::xkb::EventType::empty(),
            select_all: events,
            affect_map: map_parts,
            map: map_parts,
            details: &[],
        }))
        .unwrap();

    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let device_id = xkb::x11::get_core_keyboard_device_id(&connection);
    let keymap = xkb::x11::keymap_new_from_device(
        &ctx,
        &connection,
        device_id,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    );
    let state = xkb::x11::state_new_from_device(&keymap, &connection, device_id);

    let setup = connection.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let width = screen.width_in_pixels();

    let window = connection.generate_id();
    connection.send_request(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width,
        height: config::HEIGHT,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        value_list: &[
            x::Cw::OverrideRedirect(true),
            x::Cw::EventMask(x::EventMask::KEY_PRESS),
        ],
    });
    connection
        .send_and_check_request(&x::MapWindow { window })
        .unwrap();

    let cairo_conn = cairo::XCBConnection(
        std::ptr::NonNull::new(connection.get_raw_conn() as *mut cairo::ffi::xcb_connection_t)
            .unwrap(),
    );
    let cairo_window = cairo::XCBDrawable(window.resource_id());
    let visual_type = screen
        .allowed_depths()
        .flat_map(|depth| depth.visuals())
        .find(|visual| screen.root_visual() == visual.visual_id())
        .unwrap();

    #[repr(C)]
    struct XcbVisualtypeT {
        visual_id: u32,
        _class: u8,
        bits_per_rgb_value: u8,
        colormap_entries: u16,
        red_mask: u32,
        green_mask: u32,
        blue_mask: u32,
        pad0: u8,
    }

    let mut visual_ptr = XcbVisualtypeT {
        visual_id: visual_type.visual_id(),
        _class: visual_type.class() as u8,
        bits_per_rgb_value: visual_type.bits_per_rgb_value(),
        colormap_entries: visual_type.colormap_entries(),
        red_mask: visual_type.red_mask(),
        green_mask: visual_type.green_mask(),
        blue_mask: visual_type.blue_mask(),
        pad0: 0,
    };

    let visual_ptr = &mut visual_ptr as *mut _ as *mut cairo::ffi::xcb_visualtype_t;

    let cairo_visual = cairo::XCBVisualType(std::ptr::NonNull::new(visual_ptr).unwrap());
    let surface = cairo::XCBSurface::create(
        &cairo_conn,
        &cairo_window,
        &cairo_visual,
        screen.width_in_pixels() as i32,
        config::HEIGHT as i32,
    )
    .unwrap();
    let context = cairo::Context::new(&surface).unwrap();

    let layout = pangocairo::functions::create_layout(&context);
    layout.set_font_description(Some(&pangocairo::pango::FontDescription::from_string(
        config::FONT,
    )));

    let draw = draw::Draw::new(context, layout);

    let prompt = parts::Prompt::new(
        draw.text_size(config::PROMPT).0 as u16 + 2 * config::MARGIN,
        config::PROMPT.to_string(),
    );

    let text_field = parts::TextField::new();

    let search_field_width = width - prompt.width - text_field.width;
    let items = items
        .iter()
        .map(|text| {
            parts::Item::new(
                draw.text_size(text).0 as u16 + config::MARGIN * 2,
                text.to_string(),
            )
        })
        .collect();
    let search_field = parts::SearchField::new(search_field_width, items);

    let window = parts::Window::new(width, window);

    let mut rmenu = Rmenu::new(
        connection,
        window,
        draw,
        prompt,
        text_field,
        search_field,
        state,
    );

    rmenu.run();
}

fn ord(x: &parts::Item, y: &parts::Item, search_text: &str) -> std::cmp::Ordering {
    if x.text == search_text {
        return std::cmp::Ordering::Less;
    }

    if y.text == search_text {
        return std::cmp::Ordering::Greater;
    }

    let starts_with_x = x.text.starts_with(search_text);
    let starts_with_y = y.text.starts_with(search_text);
    let priority_x = config::PRIORITY_ITEMS.contains(&x.text.as_str());
    let priority_y = config::PRIORITY_ITEMS.contains(&y.text.as_str());

    if starts_with_x && !starts_with_y {
        return std::cmp::Ordering::Less;
    }

    if !starts_with_x && starts_with_y {
        return std::cmp::Ordering::Greater;
    }

    if priority_x && !priority_y {
        std::cmp::Ordering::Less
    } else if !priority_x && priority_y {
        std::cmp::Ordering::Greater
    } else if !search_text.is_empty() {
        x.text.len().cmp(&y.text.len())
    } else {
        std::cmp::Ordering::Equal
    }
}

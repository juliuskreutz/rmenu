mod config;
mod draw;
mod keymap;
mod parts;

struct Rmenu {
    connection: xcb::Connection,
    window: parts::Window,
    draw: draw::Draw,
    prompt: parts::Prompt,
    text_field: parts::TextField,
    search_field: parts::SearchField,
    keymap: keymap::Keymap,
}

impl Rmenu {
    fn new(
        connection: xcb::Connection,
        window: parts::Window,
        draw: draw::Draw,
        prompt: parts::Prompt,
        text_field: parts::TextField,
        search_field: parts::SearchField,
        keymap: keymap::Keymap,
    ) -> Self {
        Self {
            connection,
            window,
            draw,
            prompt,
            text_field,
            search_field,
            keymap,
        }
    }

    fn run(&mut self) {
        self.draw_prompt();
        self.draw_text_field();
        self.draw_items();
        self.connection.flush();
        self.grab_keyboard();
        self.event_loop();
    }

    fn draw_prompt(&self) {
        let draw = &self.draw;
        draw.draw_rectangle(0, self.prompt.width, config::HL_COLOR);
        draw.draw_text(0, &self.prompt.text, config::TEXT_HL_COLOR);
    }

    fn draw_text_field(&mut self) {
        let draw = &self.draw;
        let text = &format!("{}|", self.text_field.text);

        if draw.text_size(text).0 as u16 + config::MARGIN * 2 > self.text_field.width {
            self.text_field.text.pop();
            return;
        }

        draw.draw_rectangle(self.prompt.width, self.text_field.width, config::BG_COLOR);
        draw.draw_text(self.prompt.width, text, config::TEXT_COLOR);
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
        draw.draw_rectangle(position, self.search_field.width, config::BG_COLOR);

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

            draw.draw_rectangle(position, item.width, box_color);
            draw.draw_text(position, &item.text, text_color);

            position += item.width;
        }
    }

    fn grab_keyboard(&self) {
        while xcb::grab_keyboard(
            &self.connection,
            true,
            self.window.id,
            xcb::CURRENT_TIME,
            xcb::GRAB_MODE_ASYNC as u8,
            xcb::GRAB_MODE_ASYNC as u8,
        )
        .get_reply()
        .unwrap()
        .status()
            != 0
        {}
    }

    fn event_loop(&mut self) {
        while let Some(event) = self.connection.wait_for_event() {
            if event.response_type() == xcb::KEY_PRESS {
                let key_press: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&event) };

                let keymap = self
                    .keymap
                    .get_keysym(key_press.detail(), key_press.state());

                if keymap::is_escape(keymap) {
                    return;
                }

                if keymap::is_return(keymap) {
                    print!("{}", self.search_field.get_selection());
                    return;
                }

                if keymap::is_left(keymap) {
                    self.move_cursor_left();
                    self.connection.flush();
                    continue;
                } else if keymap::is_right(keymap) {
                    self.move_cursor_right();
                    self.connection.flush();
                    continue;
                }

                if keymap::is_backspace(keymap) {
                    self.text_field.text.pop();
                } else {
                    self.text_field.text.push_str(self.keymap.get_key(keymap));
                }

                self.draw_text_field();
                self.draw_items();
                self.connection.flush();
            }
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

        draw.draw_rectangle(old_position, old_item.width, config::BG_COLOR);
        draw.draw_rectangle(new_position, new_item.width, config::HL_COLOR);

        draw.draw_text(old_position, &old_item.text, config::TEXT_COLOR);
        draw.draw_text(new_position, &new_item.text, config::TEXT_HL_COLOR);

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

        draw.draw_rectangle(old_position, old_item.width, config::BG_COLOR);
        draw.draw_rectangle(new_position, new_item.width, config::HL_COLOR);

        draw.draw_text(old_position, &old_item.text, config::TEXT_COLOR);
        draw.draw_text(new_position, &new_item.text, config::TEXT_HL_COLOR);

        self.search_field.cursor += 1;
    }
}

fn main() {
    let (connection, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = connection.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let width = screen.width_in_pixels();

    let window = connection.generate_id();
    xcb::create_window(
        &connection,
        xcb::COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0,
        0,
        width,
        config::HEIGHT,
        0,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &[
            (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_KEY_PRESS),
            (xcb::CW_OVERRIDE_REDIRECT, 1),
        ],
    );
    xcb::map_window(&connection, window);

    let mapping = xcb::get_keyboard_mapping(
        &connection,
        setup.min_keycode(),
        setup.max_keycode() - setup.min_keycode() + 1,
    )
    .get_reply()
    .unwrap();
    let keymap = keymap::Keymap::new(
        mapping.keysyms().to_owned(),
        mapping.keysyms_per_keycode(),
        setup.min_keycode(),
    );

    let cairo_conn = cairo::XCBConnection(
        std::ptr::NonNull::new(connection.get_raw_conn() as *mut cairo::ffi::xcb_connection_t)
            .unwrap(),
    );
    let cairo_window = cairo::XCBDrawable(window);
    let mut visual_type = screen
        .allowed_depths()
        .flat_map(|depth| depth.visuals())
        .find(|visual| screen.root_visual() == visual.visual_id())
        .unwrap();
    let visual_ptr: *mut cairo::ffi::xcb_visualtype_t =
        &mut visual_type.base as *mut _ as *mut cairo::ffi::xcb_visualtype_t;
    let cairo_visual = cairo::XCBVisualType(std::ptr::NonNull::new(visual_ptr).unwrap());
    let surface = cairo::XCBSurface::create(
        &cairo_conn,
        &cairo_window,
        &cairo_visual,
        screen.width_in_pixels() as i32,
        32,
    )
    .unwrap();
    let context = cairo::Context::new(&surface).unwrap();

    let layout = pangocairo::create_layout(&context).unwrap();
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
    let items = get_items()
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
        keymap,
    );

    rmenu.run();
}

fn get_items() -> std::collections::HashSet<String> {
    std::env::var("PATH")
        .unwrap()
        .split(':')
        .flat_map(std::fs::read_dir)
        .flat_map(|read| {
            read.flatten()
                .map(|entry| entry.file_name().into_string().unwrap())
        })
        .collect::<std::collections::HashSet<String>>()
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

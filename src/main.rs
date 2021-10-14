use std::env;
use std::fs;
use std::rc::Rc;
use nwg::Button;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Settings {
    browsers: Vec<Browser>
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Browser {
    name: String,
    command_line: String,
    icon: String,
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut window = Default::default();
    let mut url_edit = Default::default();
    let layout = Default::default();

    let settings_file = fs::read_to_string("settings.json").expect("Unable to read settings.json");
    let settings: Settings = serde_json::from_str(&settings_file).expect("Error reading settings.json");

    let mut buttons: Vec<Button> = Vec::new(); 

    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    nwg::Window::builder()
        .size((600, 250))
        // .position((300, 300))
        .title("CrossRoad")
        .build(&mut window)
        .unwrap();

    nwg::TextInput::builder()
        .text(url)
        .focus(true)
        .parent(&window)
        .build(&mut url_edit)
        .unwrap();

    let mut buttons_layout = nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        // .child(0, 0, &url_edit);
        .child_item(nwg::GridLayoutItem::new(&url_edit, 0, 0, settings.browsers.len() as u32, 1));

    let mut button_count = 0;
    for browser in settings.browsers {
        println!("{:?}", browser);
        let mut button = Default::default();
        nwg::Button::builder()
        .text(&browser.name)
        .parent(&window)
        .build(&mut button)
        .unwrap();
        buttons_layout = buttons_layout.child_item(nwg::GridLayoutItem::new(&button, button_count, 1, 1, 1));
        button_count += 1;
        buttons.push(button);
    };
    
    buttons_layout.build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => 
                if &handle == &events_window as &nwg::Window {
                    // nwg::modal_info_message(&events_window.handle, "Goodbye", &format!("Goodbye {}", url_edit.text()));
                    nwg::stop_thread_dispatch();
                },
            E::OnButtonClick => 
                for button in &buttons {
                    if &handle == &button.handle {
                        println!("{:?}", button.text());
                        // for browser in settings.browsers {
                        //     if &browser.name == &button.text() {
                        //         println!("{} {}", browser.command_line, url_edit.text());
                        //     }
                        // }
                    }
                },
            // E::OnButtonClick => 
            //     if &handle == &hello_button {
            //         nwg::modal_info_message(&events_window.handle, "Hello", &format!("Hello {}", url_edit.text()));
            //     },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}

#![windows_subsystem = "windows"]

use nwg::Button;
use nwg::WindowFlags;
use std::process::Command;
use serde::Deserialize;
use std::env;
use std::fs;
use std::rc::Rc;
use std::sync::Arc;

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
    arguments: Vec<String>,
    icon: String,
}

fn run_browser(browser: &Browser, url: String) {
    let mut arguments: Vec<String> = Vec::new();
    for argument in &browser.arguments {
        arguments.push(argument.clone());
    }
    arguments.push(url);
    
    let _process = match Command::new(&browser.command_line)
        .args(arguments)
        .spawn() {
            Ok(process) => process,
            Err(err)    => panic!("Running process error: {}", err),
        };

    nwg::stop_thread_dispatch();
    std::process::exit(0);
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut window = Default::default();
    let mut url_edit = Default::default();
    let layout = Default::default();

    let appdata = env::var("APPDATA").unwrap();
    let settings_filepath = format!("{}/CrossRoad/settings.json", appdata);
    let settings_file = fs::read_to_string(settings_filepath).expect("Unable to read settings.json");
    let parsed_settings: Settings = serde_json::from_str(&settings_file).expect("Error reading settings.json");
    let settings = Arc::new(parsed_settings);
    
    let mut buttons: Vec<Button> = Vec::new(); 

    let args: Vec<String> = env::args().collect();
    let mut url="";
    if args.len() > 1 {
        url = &args[1];
    }

    nwg::Window::builder()
        .center(true)
        .size((600, 250))
        .flags(WindowFlags::WINDOW | WindowFlags::POPUP)
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
        .child_item(nwg::GridLayoutItem::new(&url_edit, 0, 0, settings.browsers.len() as u32, 1));

    let mut button_count = 0;
    for browser in &settings.browsers {
        let mut button = Default::default();
        // let browser_icon = nwg::Icon::from_file(&browser.icon, true).unwrap();
        nwg::Button::builder()
        .text(&browser.name)
        // .icon(&browser_icon)
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
    window.set_visible(true);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => 
                if &handle == &events_window as &nwg::Window {
                    nwg::stop_thread_dispatch();
                },
            E::OnButtonClick => 
                for button in &buttons {
                    if &handle == &button.handle {
                        for browser in &settings.browsers {
                            if &browser.name == &button.text() {
                                run_browser(&browser, url_edit.text());
                            }
                        }
                    }
                },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}

use tray_item::{TrayItem, IconSource};

use crate::{ThreadState, VERSION};

pub fn start_process(tx: &std::sync::mpsc::Sender<ThreadState>)
{
    std::thread::spawn(|| run(tx));
}

fn run(sender: &std::sync::mpsc::Sender<ThreadState>)
{
    let mut tray = TrayItem::new("BUM", IconSource::Resource("tray-default")).unwrap();
    tray.add_label(format!("BUM v{}", VERSION).as_str()).unwrap();

    tray.add_menu_item("Item Tracker", || {
        println!("not dun yet");
    }).unwrap();

    tray.inner_mut().add_separator().unwrap();

    tray.add_menu_item("Quit", move || sender.send(ThreadState::Finished).unwrap()).unwrap();

    loop {}
}
use std::panic::PanicInfo;
use std::process::exit;
use native_dialog::{MessageDialog, MessageType};

pub fn panic_handler(panic_info: &PanicInfo) {
    let mut panic_message = "A critical system failure occurred and the program will shutdown immediately.\n\n".to_owned();

    panic_message += "Here is some information to help repair the problem:\n";

    let loc = panic_info.location().cloned().unwrap();
    panic_message += format!("Problem occurred in: {}, L{}, C{}\n", loc.file(), loc.line(), loc.column()).as_str();
    if let Some(&s) = panic_info.payload().downcast_ref::<&str>() {
        panic_message += s;
    } else {
        panic_message += "No further messages could we get."
    }

    let _ = MessageDialog::new()
        .set_title("Sorry!")
        .set_text(&panic_message)
        .set_type(MessageType::Error)
        .show_alert()
        .unwrap();

    exit(1);
}

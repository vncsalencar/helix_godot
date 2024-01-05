use std::fmt::Display;
use std::process::Command;
use std::thread;
use std::time::Duration;
use windows::core::{Result, HSTRING};
use windows::UI::Notifications::ToastNotification;
use windows::{Data::Xml::Dom::XmlDocument, UI::Notifications::ToastNotificationManager};

enum ToastDuration {
    Short,
    Long,
}

impl Display for ToastDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ToastDuration::Short => "short",
            ToastDuration::Long => "long",
        };

        f.write_str(s)
    }
}

struct ToastXmlData {
    duration: ToastDuration,
    title: String,
    text: String,
    silent: bool,
}

fn toast(data: ToastXmlData) {
    let app_id = "Microsoft.WindowsTerminal_8wekyb3d8bbwe!App";
    let toast_notifier =
        ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(app_id)).unwrap();

    let toast_xml = create_toast_xml(data).unwrap();
    let notification = ToastNotification::CreateToastNotification(&toast_xml).unwrap();

    toast_notifier.Show(&notification).unwrap();
    thread::sleep(Duration::from_secs(4));
    toast_notifier.Hide(&notification).unwrap();
}

fn create_toast_xml(data: ToastXmlData) -> Result<XmlDocument> {
    let toast_xml = XmlDocument::new()?;
    let silent = if data.silent { "true" } else { "false" };

    let xml_str = format!(
        "<toast duration=\"{}\">
            <visual>
                <binding template=\"ToastGeneric\">
                    <text hint-maxLines=\"1\">{}</text>
                    <text>{}</text>
                </binding>
            </visual>
            <audio silent=\"{}\"/>
        </toast>",
        data.silent, data.duration, data.title, data.text
    );

    toast_xml.LoadXml(&HSTRING::from(xml_str))?;
    Ok(toast_xml)
}

fn main() {
    let terminal = "alacritty.exe";
    let hx = "hx.exe";

    let mut args = std::env::args();
    _ = args.next().unwrap();
    let project_dir = args.next().unwrap();
    let file = args.next().unwrap();

    let output = Command::new(terminal)
        .arg("-e")
        .arg(hx)
        .arg("-w")
        .arg(project_dir)
        .arg(file)
        .output()
        .expect("Couldn't start helix.");

    let silent = output.status.success();
    let output = format!("{:?}", output);

    toast(ToastXmlData {
        duration: ToastDuration::Short,
        title: "HelixGodot".to_string(),
        text: output.clone(),
        silent,
    });

    println!("{:?}", output);
}

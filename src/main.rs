use rusb::Device;
use std::time::Duration;
extern crate udev;

fn main() {
    println!("USB CHECKER!");
    match rusb::devices() {
        Ok(devices) => {
            for device in devices.iter() {
                print_device(&device);
            }
        }
        Err(e) => {
            eprintln!("Failed to get device list: {}", e);
        }
    }
    monitor_devices();
}

// fn monitor_devices() {
//     let mut enumerator = udev::Enumerator::new().unwrap();
//     enumerator.match_subsystem("usb").unwrap();
//     let monitor = udev::Monitor::new().unwrap();
//     monitor
//         .match_subsystem_devtype("usb", "usb_device")
//         .unwrap();
//     let mut monitor_socket = monitor.listen().unwrap();

//     println!("Listening for events...");
//     loop {
//         match monitor_socket.receive_event() {
//             Ok(event) => {
//                 println!("EVENT:");
//                 println!("udev: {:?}", event.udev());
//                 println!("sequence_number: {:?}", event.sequence_number());
//                 println!("type: {:?}", event.event_type());
//                 println!("path: {:?}", event.syspath());
//                 println!("device: {:?}", event.device().unwrap().sysname());
//             }
//             Err(e) => println!("{:?}", e),
//         }
//     }
// }

fn print_device(device: &Device<rusb::GlobalContext>) {
    match device.device_descriptor() {
        Ok(descriptor) => {
            println!("----------------------------------------");
            println!(
                "Bus {:03} Device {:03} ID {:04x}:{:04x}",
                device.bus_number(),
                device.address(),
                descriptor.vendor_id(),
                descriptor.product_id()
            );

            let handle = device.open();
            match handle {
                Ok(handle) => {
                    if let Ok(langs) = handle.read_languages(Duration::from_secs(0)) {
                        if !langs.is_empty() {
                            let lang = langs[0];
                            println!(
                                "Manufacturer: {}",
                                handle
                                    .read_manufacturer_string(
                                        lang,
                                        &descriptor,
                                        Duration::from_secs(0)
                                    )
                                    .unwrap_or("Unknown".to_string())
                            );
                            println!(
                                "Product: {}",
                                handle
                                    .read_product_string(lang, &descriptor, Duration::from_secs(0))
                                    .unwrap_or("Unknown".to_string())
                            );
                            println!(
                                "Serial Number: {}",
                                handle
                                    .read_serial_number_string(
                                        lang,
                                        &descriptor,
                                        Duration::from_secs(0)
                                    )
                                    .unwrap_or("Unknown".to_string())
                            );
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Cannot open the device");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to get device descriptor: {}", e);
        }
    }
}

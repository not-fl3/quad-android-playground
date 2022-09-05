// copy-pasted from quad-bt/examples and slightly modified to fit into the apps ui

use macroquad::{
    prelude::*,
    ui::{root_ui, widgets},
};
use quad_bt::{self as bt, Message};
use std::collections::VecDeque;

enum State {
    BluetoothNotReady,
    Scan,
    Connected(bt::Connection),
}

pub struct Bluetooth {
    state: State,
    received_data: VecDeque<Vec<u8>>,
    adapter: bt::Adapter,
    characteristics: Vec<bt::Characteristic>,
}

impl Bluetooth {
    pub fn new() -> Bluetooth {
        let state = State::BluetoothNotReady;
        let received_data = VecDeque::new();
        let adapter = bt::Adapter::new().unwrap();
        let characteristics = vec![];

        Bluetooth {
            state,
            received_data,
            adapter,
            characteristics,
        }
    }

    pub fn ui(&mut self) {
        match self.state {
            State::BluetoothNotReady => {
                if self.adapter.is_ready() {
                    self.adapter.start_scan().unwrap();
                    self.state = State::Scan;
                }
            }
            State::Connected(ref mut connection) => {
                let mut done = false;
                while let Ok(Some(msg)) = connection.try_recv() {
                    match msg {
                        Message::CharacteristicDiscovered(characteristic) => {
                            self.characteristics.push(characteristic);
                        }
                        Message::Data(data) => {
                            self.received_data.push_front(data);
                            if self.received_data.len() > 20 {
                                self.received_data.pop_back();
                            }
                        }
                        Message::Disconnected => {
                            done = true;
                        }
                        _ => {}
                    }
                }
                if done {
                    self.received_data.clear();
                    self.characteristics.clear();
                    self.adapter.start_scan().unwrap();
                    self.state = State::Scan;
                }
            }
            _ => {}
        }

        clear_background(WHITE);

        match self.state {
            State::BluetoothNotReady => root_ui().label(None, "Bluetooth initializing"),
            State::Scan => {
                root_ui().label(None, "Devices:");

                let mut device_id = None;
                self.adapter
                    .walk_devices(|device| {
                        if widgets::Button::new(format!("{:?} {:?}", device.address, device.name))
                            .size(vec2(400., 50.))
                            .ui(&mut *root_ui())
                        {
                            device_id = Some(device.id());
                        }
                    })
                    .unwrap();

                if let Some(device_id) = device_id {
                    let connection = self.adapter.connect(device_id.clone()).unwrap();
                    self.state = State::Connected(connection);
                }
            }
            State::Connected(ref mut connection) => {
                for characteristic in &self.characteristics {
                    widgets::Label::new(format!("{:?}", &characteristic.id)).ui(&mut *root_ui());

                    widgets::Label::new(format!(
                        "write: {:?}, read: {:?}, notify: {:?}, indicate: {:?}",
                        characteristic.write,
                        characteristic.read,
                        characteristic.notify,
                        characteristic.indicate
                    ))
                    .ui(&mut *root_ui());

                    if widgets::Button::new("write 1")
                        .size(vec2(100., 50.))
                        .ui(&mut *root_ui())
                    {
                        characteristic.send_bytes(&[0x01], true).unwrap();
                    }
                    root_ui().same_line(110.);
                    if widgets::Button::new("write Uxx")
                        .size(vec2(100., 50.))
                        .ui(&mut *root_ui())
                    {
                        characteristic.send_string("Uxx").unwrap();
                    }
                    root_ui().same_line(220.);

                    if widgets::Button::new("notify")
                        .size(vec2(100., 50.))
                        .ui(&mut *root_ui())
                    {
                        characteristic.set_notification(true).unwrap();
                    }
                    root_ui().same_line(330.);
                    if widgets::Button::new("indicate")
                        .size(vec2(100., 50.))
                        .ui(&mut *root_ui())
                    {
                        characteristic.set_indication(true).unwrap();
                    }
                    root_ui().same_line(440.);
                    if widgets::Button::new("write 0x3")
                        .size(vec2(100., 50.))
                        .ui(&mut *root_ui())
                    {
                        characteristic.send_bytes(&[0x3], true).unwrap();
                    }
                    root_ui().same_line(440.);
                    if widgets::Button::new("write wtf")
                        .size(vec2(100., 50.))
                        .ui(&mut *root_ui())
                    {
                        characteristic.send_bytes(&[0x1, 0x2, 0x3], false).unwrap();
                    }
                }
                if widgets::Button::new("disconnect")
                    .position(vec2(screen_width() - 200., screen_height() - 50.))
                    .size(vec2(200., 50.))
                    .ui(&mut *root_ui())
                {
                    connection.disconnect().unwrap();
                }
            }
        }

        for (n, data) in self.received_data.iter().enumerate() {
            widgets::Label::new(format!("{:?}", data))
                .position(vec2(450., n as f32 * 20.))
                .ui(&mut *root_ui());
        }
    }
}

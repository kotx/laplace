use dialoguer::{console::Term, Select};
use lazy_regex::regex_captures;
use pad_motion::{
    protocol::{ConnectionType, ControllerData, ControllerInfo, DeviceType, SlotState},
    server::{DsServer, Server},
};
use serial2::SerialPort;
use std::{
    io::{BufRead, BufReader},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};

fn main() {
    let port = {
        let ports: Vec<String> = SerialPort::available_ports()
            .expect("unable to list serial ports")
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        println!("Select a serial port:");
        let addy_idx = Select::new()
            .items(&ports)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .expect("unable to prompt for input");

        if addy_idx.is_none() {
            std::process::exit(0);
        }

        let addr = &ports[addy_idx.unwrap()];
        println!("Connecting to {}", addr);
        SerialPort::open(addr, 115200).expect("Unable to open serial port")
    };

    port.set_dtr(true).unwrap();

    let mut port = BufReader::new(port);

    let running = Arc::new(AtomicBool::new(true));

    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    let server = Arc::new(Server::new(None, None).expect("unable to create pad-motion server"));
    let server_thread_join_handle = {
        let server = server.clone();
        println!("Server running at 127.0.0.1:26760");
        server.start(running.clone())
    };

    let controller_info = ControllerInfo {
        slot: 0,
        slot_state: SlotState::Connected,
        device_type: DeviceType::FullGyro,
        connection_type: ConnectionType::USB,
        ..Default::default()
    };
    server.update_controller_info(controller_info);

    let now = Instant::now();
    let mut accel: [f32; 3] = [0., 0., 0.];
    let mut gyro: [f32; 3] = [0., 0., 0.];

    while running.load(Ordering::SeqCst) {
        serial_read(&mut port, &mut accel, &mut gyro).expect("unable to read serial data");

        server.update_controller_data(
            0,
            ControllerData {
                connected: true,
                motion_data_timestamp: now.elapsed().as_micros() as u64,
                accelerometer_x: accel[0],
                accelerometer_y: accel[1],
                accelerometer_z: accel[2],
                gyroscope_pitch: gyro[0],
                gyroscope_yaw: gyro[1],
                gyroscope_roll: gyro[2],
                ..Default::default()
            },
        );
    }

    server_thread_join_handle.join().unwrap();
}

fn serial_read(
    port: &mut BufReader<SerialPort>,
    accel: &mut [f32; 3],
    gyro: &mut [f32; 3],
) -> anyhow::Result<()> {
    for _ in 0..2 {
        let mut line = String::new();
        port.read_line(&mut line)?;

        let matches = regex_captures!(
            r#"^(a|g): (\-?\d+(?:\.?\d+)?) (\-?\d+(?:\.?\d+)?) (\-?\d+(?:\.?\d+)?)"#,
            &line
        );

        if let Some((_, typ, x, y, z)) = matches {
            let tgt = match typ {
                "a" => &mut *accel,
                "g" => gyro,
                _ => unreachable!(),
            };

            tgt[0] = x.parse()?;
            tgt[1] = y.parse()?;
            tgt[2] = z.parse()?;
        } else {
            println!("no match: {}", line);
        }
    }

    Ok(())
}

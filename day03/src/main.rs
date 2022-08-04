mod program;

use self::program::{initialize_program, Event};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut program = initialize_program();

    // Rain
    program.register(Event {
        func: |&mut _state| {
            println!("HAHAHAHAHA HI!");
        },
        interval: Some(Duration::from_secs_f32(0.5)),
        ..Default::default()
    });

    // let mut stdout = cursor::HideCursor::from(stdout().lock().into_raw_mode()?);

    program.run();

    Ok(())
}

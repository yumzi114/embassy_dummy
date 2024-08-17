#![no_std]
#![no_main]
// #![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, embassy::{self}, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*, timer::TimerGroup
};
// use embedded_hal::i2c::{I2c, Error};
use lcd1602_driver::{
    command::{DataWidth, MoveDirection, State},
    lcd::{self, Anim, Basic, Ext, FlipStyle, Lcd, MoveStyle},
    sender::I2cSender,
    utils::BitOps,
};



const HEART: [u8; 8] = [
    0b00000, 0b00000, 0b01010, 0b11111, 0b01110, 0b00100, 0b00000, 0b00000,
];

#[embassy_executor::task]
async fn run() {
    loop {
        log::info!("Hello from an embassy thread");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}
#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);
    let mut i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio3,
        io.pins.gpio2,
        100.kHz(),
        &clocks,
        None,
    );
    let mut sender = I2cSender::new(&mut i2c, 0x27);
    let lcd_config = lcd::Config::default().set_data_width(DataWidth::Bit4);
    let mut lcd= Lcd::new(&mut sender, &mut delay, lcd_config, 10);
    lcd.write_graph_to_cgram(1, &HEART);

    let mut graph_data = lcd.read_graph_from_cgram(1);
    graph_data[1].set_bit(2);
    graph_data[2].set_bit(2);
    lcd.write_graph_to_cgram(2, &graph_data);

    lcd.set_cursor_blink_state(State::On);
    lcd.set_cursor_pos((1, 0));

    // type writer effect
    lcd.typewriter_write("hello,", 250_000);

    // relative cursor move
    lcd.offset_cursor_pos((1, 0));

    // to test write string to cur pos
    lcd.write_str_to_cur("world!");

    // manually delay
    lcd.delay_ms(250);

    let line_capacity = lcd.get_line_capacity();

    // to test write character to specified position
    // since tilde chracter (~) is not in CGROM of LCD1602A
    // it should be displayed as a full rectangle
    lcd.write_char_to_pos('~', (15, 0));

    // manually delay
    lcd.delay_ms(250);

    // to test whether line break works well
    // set cursor to the end of first line, and write a vertical line
    lcd.set_cursor_pos((line_capacity - 1, 0));
    lcd.write_char_to_cur('|');

    // turn off cursor blinking, so that cursor will only be a underline
    lcd.set_cursor_blink_state(State::Off);

    lcd.typewriter_write("Hello, ", 250_000);

    // to test right to left write in
    // move cursor to left end of display window, then type string in reverse order
    lcd.set_direction(MoveDirection::RightToLeft);
    lcd.set_cursor_pos((15, 1));
    lcd.typewriter_write("~!", 250_000);
    // and the 2 type of split flap display effect
    lcd.split_flap_write("2061", FlipStyle::Simultaneous, None, 150_000, None);
    lcd.split_flap_write(
        "DCL",
        FlipStyle::Sequential,
        Some(10),
        150_000,
        Some(250_000),
    );

    lcd.set_cursor_state(State::Off);

    // replace 2 rectangle with custom heart shape and diamond shape
    lcd.delay_ms(1_000);
    lcd.write_graph_to_pos(1, (15, 0));
    lcd.delay_ms(1_000);
    lcd.write_graph_to_pos(2, (15, 1));

    // to test read from DDRAM
    // read from first line end, and write same character to the second line end
    let char_at_end = lcd.read_byte_from_pos((39, 0));
    lcd.write_byte_to_pos(char_at_end, (39, 1));

    // shift display window
    lcd.delay_ms(1_000);
    lcd.shift_display_to_pos(2, MoveStyle::Shortest, State::On, 250_000);
    lcd.delay_ms(1_000);
    lcd.shift_display_to_pos(40 - 2, MoveStyle::Shortest, State::On, 250_000);
    lcd.delay_ms(1_000);
    lcd.shift_display_to_pos(0, MoveStyle::Shortest, State::On, 250_000);

    // and blinking display 3 times
    lcd.delay_ms(1_000);
    lcd.full_display_blink(3, 500_000);

    // and blinking backlight 3 times
    for _ in 0..3 {
        lcd.delay_ms(500);
        lcd.set_backlight(State::Off);
        lcd.delay_ms(500);
        lcd.set_backlight(State::On);
    }
    // esp_println::logger::init_logger_from_env();
    spawner.spawn(run()).ok();
    // for addr in 1..=127 {
    //     log::info!("Scanning Address {}",addr as u8);

    //     // Scan Address
        
    //     // let res = i2c.read(addr as u8, &mut [0]);
    //     i2c.write(1,"1".as_bytes());
    //     // Check and Print Result
    //     // match res {
    //     //     Ok(_) => 
    //     //     log::info!("Device Found at Address {}",addr as u8),
    //     //     Err(_) => 
    //     //     log::info!("No Device Found",),
    //     // }
    // }
    loop {
        log::info!("Hello from Main");
        // i2c.write();
        Timer::after(Duration::from_millis(1_000)).await;
    }
}


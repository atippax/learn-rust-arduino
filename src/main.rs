#![no_std]
#![no_main]
use arduino_hal::{hal::port::Pin, port::{PinOps, mode::Output}};
use panic_halt as _;
const IS_PRESSED:bool = true;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led_red = pins.d13.into_output();
    let mut led_yellow = pins.d8.into_output();
    let mut led_green = pins.d5.into_output();
    let button = pins.d3.into_pull_up_input();
    let mut led_for_button = pins.d2.into_output();

    led_green.set_low();
    led_yellow.set_low();
    led_red.set_low();

    let mut blinking_active = false;
    loop{
        let current_button_state = button.is_low();
        if check_button_is_presses(current_button_state) {
            show_led_and_stop(&mut led_for_button,None);
            blinking_active = true;
        }
        if blinking_active {
            loop_swith_led_3_times(&mut led_yellow,&mut led_red);
            blinking_active = false;
        }
        else {
            show_led_and_stop(&mut led_green,Some(10_u32));
            arduino_hal::delay_ms(10);
        }
    }
}

fn loop_swith_led_3_times<P1, P2>(led1: &mut Pin<Output, P1>, led2: &mut Pin<Output, P2>)
where
    P1: PinOps,
    P2: PinOps,
{
    let mut loop_time = 0;
    loop {
        swith_led(led1,led2);
        loop_time += 1;
        if loop_time == 3 {
            break;
        }
    }
}
fn check_button_is_presses(current_button_state:bool)-> bool {
    current_button_state == IS_PRESSED
}
fn show_led<P:PinOps>(led: &mut Pin<Output, P>){
    led.set_high();
}
fn stop_led<P:PinOps>(led: &mut Pin<Output, P>){
    led.set_low();
}
fn show_led_and_stop<P:PinOps>(led: &mut Pin<Output, P>,time:Option<u32>){
    show_led(led);
    match time {
        Some(time)=>arduino_hal::delay_ms(time),
        None=>arduino_hal::delay_ms(500)
    }
    stop_led(led);
}

fn swith_led<P1, P2>(led1: &mut Pin<Output, P1>, led2: &mut Pin<Output, P2>)
where
    P1: PinOps,
    P2: PinOps,
{
    show_led_and_stop(led1,None);
    show_led_and_stop(led2,None);
}

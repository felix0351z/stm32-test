cargo build --release
openocd -f nucleo.cfg -c"program target/thumbv7em-none-eabi/release/stm32-test verify reset exit"
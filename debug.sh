cargo build
openocd -f nucleo.cfg -c"program target/thumbv7em-none-eabi/debug/stm32-test verify reset exit"
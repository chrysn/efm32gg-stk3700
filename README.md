`efm32gg-stk3700`
-----------------

This is a board support crate for the [EFM32GG-STK3700] Starter Kit.

Right now, it is very minimal; LEDs and buttons are exposed in a board struct,
but that's about it. It will grow as the underlying [HAL implementation] does.

Thus, the larger value is in providing runnable examples. With an STK connected
via USB and an [OpenOCD] running (typically ``openocd -f board/efm32.cfg``), a
blinking example can be run using ``cargo +nightly run --example blink``.

[EFM32GG-STK3700]: https://www.silabs.com/products/development-tools/mcu/32-bit/efm32-giant-gecko-starter-kit
[HAL implementation]: https://github.com/chrysn/efm32gg-hal
[OpenOCD]: http://openocd.org/

### License

This is licensed under the [Apache License] or the [MIT License] at the your
option. By contributing to this project, you license your contribution under
the same dual-licensed terms unless the contribution itself says otherwise.

[Apache License]: http://www.apache.org/licenses/LICENSE-2.0
[MIT License]: http://opensource.org/licenses/MIT

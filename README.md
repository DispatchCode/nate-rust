*Noob rustacean* version of [Nate](https://github.com/DispatchCode/NaTE).

## NaTE - Not a True Emulator *(Rust version)*

A codegolf challenge captured my attention, [Emulate an Intel 8086 CPU](https://codegolf.stackexchange.com/questions/4732/emulate-an-intel-8086-cpu).

Here is the result:

![rust_nate](https://user-images.githubusercontent.com/4256708/85185145-1033fa80-b293-11ea-97f8-bc035c3444a1.png)

## Project Files

`mca8086.rs` is the core that fulfill the "fetch and decode" step (inspired by NaTE). 

`cpu.rs` offers a basic CPU initialization.

`cpu_exec.rs` emulates an instruction previously disassembled by mca8086.

Please note that the video memory is not properly emulated; features such as segments and interrupts are not supported.

#### LICENSE
The program is licensed under GPL v.3.

# Toy Synth
<!-- TODO: Rename to something cooler -->

> **Note**: This project is still in its early stages and is not ready for use.

This project is composed of multiple packages that can be used to create a toy synthesizer. The idea is to have a Raspberry Pi Pico that sends MIDI messages over USB to a Raspberry Pi Zero W 2 that runs a Rust application that generates sounds and has a nice visual interface.

This is the proposed hardware and software setup:

```mermaid

%%{ init : {"flowchart" : { "curve" : "stepAfter" }}}%%
flowchart LR
  subgraph X["Firmware"]
  direction LR
    A[Raspberry Pi Pico]
    C[Slide potentiometer] --> A
    D[Rotary potentiometer] --> A
    E[Rotary encoders x4] --> A
    F[Buttons x24 *] --> A
  end

  subgraph Y["App"]
  direction RL
    B[Raspberry Pi Zero W 2]
    G[Accelerometer *] --> B
    H[Radio FM Module *] --> B
    I[Audio DAC *] --> B
    J[OLED Screen *] --> B
  end

  X<-->Y
```
> **Note**: Components marked with * have not been bought or implemented yet.


* `firmware`: Firmware for a Raspberry Pi Pico that sends MIDI messages over USB
<!-- TODO: Rename app to something more descriptive -->
* `app`: Application that runs on a Raspberry Pi Zero W 2 that receives MIDI messages over USB and generates sounds

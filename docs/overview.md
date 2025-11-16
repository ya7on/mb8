# Overview

MB8 is an 8-bit virtual machine and playground for writing games and bots on a tiny assembler. Think CHIP-8, but built for multiplayer simulations: everything runs in one VM, sharing state through registers and mailboxes. It feels like smart contracts for games—the logic and the bots all live in the same minimal ISA.

Terminology:
- **Judge** — the host program that defines the rules, renders graphics, and schedules bots.
- **Bots** — player/agent programs that follow the judge’s rules. They run in separate contexts and talk via shared mailboxes.

Check the following chapters for the machine layout, instruction set, and examples.

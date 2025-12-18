# ExtendCalc DataStructures
 <h1 align="center">Data Structures</h1>

## RON Structures

All the Keypad and Key templates are stored in [RON][ron].

The structure looks like this:

...text
└── [Keypads](./src/data_handling/models/keypads.rs)
    └── [KeypadRef](./src/data_handling/models/keypad_ref.rs)
        └── [KeypadDef](./src/data_handling/models/keypad_def.rs)
            └── [KeyRef](./src/data_handling/models/key_ref.rs)
                    └── [KeyDef](./src/data_handling/models/keydef.rs)
...

This allows exensibility of the collection of Keypads and Keys.

**NNOTE:** the RON structure is only parsed at first startup and then stored in
the UI structures. This are then saved as config data so that at next startup
there is no need to parse the RON files. If the RON structures have been updated
then at the next startup the RON structure will be parsed again and the UI 
structure re-built and stored in the app config data.

## Implementation

Programmatically, the data structures are defined as follows:

...rust
>pub type CalcKeypad = KeypadDef;
>pub type CalcKeypads = Keypads;
>pub type CalcKey = KeyDef;
>pub type CalcKeys = Keys;
...

All the implementation code is the above modules.
Which then allows us tto build the final UI stuctures.

The program structures mimic the RON structure with a few differences.

...text
└── AssembledKeypads
    ├── CalcKeypads
    └── Vec ── AssembledKeypad
               ├── CalcKeypad
               └── CalcKeyGrid
                   └── HashMap
                       ├── GridPosition
                       │   ├── Row
                       │   └── Column
                       └── CalcKey
...
                              
**NOTE:** We have introduced a Hashmap with a tuple as a key into it.
Turns the HashMap into a "grid" of Keys. It's not laided out like a
traditional grid, but the tuple access key allows us to iterate over it
as if it was a grid. Which then allows us to layout the keys in the UI
in a grid order.

## UI Structures

Once we have all the RON stuff into some structures we can actually do stuff on,
we need to build some structures that make sense for actually using the Keypad in
the UI.

Namely

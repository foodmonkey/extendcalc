# ExtendCalc DataStructures
 <h1 align="center">Data Structures</h1>

## RON Structures

All the Keypad and Key templates are stored in [RON][ron].

The RON structure looks like this:

...text

    └── [CalcKeypadRef](./src/data_handling/models/keypad_ref.rs)
        └── [CalcKeypadDef](./src/data_handling/models/keypad_def.rs)
            └── [CalcKeyRef](./src/data_handling/models/key_ref.rs)
                    └── [CalcKeyDef](./src/data_handling/models/keydef.rs)
...
And the struture for lists fo Keypads that allows us to build the
assembled keypads that the UI uses to convert into a grid of buttons
that can be pressed.

    └── AssembledKeypads (a collection of AssembledKeypad that the user can switch between)
        └── AssmbledKeypad
            └── CalcKeypad data.... (rows, columns, label etc)
            └── CalcKeyGrid (HashMap)
                └── GridPosition (the key to HashMap row, column)
                └── CalcKey              
  
**NNOTE:** the RON structure is only parsed at first startup and then stored in
the UI structures. This are then saved as config data so that at next startup
there is no need to parse the RON files. If the RON structures have been updated
then at the next startup the RON structure will be parsed again and the UI 
structure re-built and stored in the app config data.

## Implementation

The program structures mimic the RON structure with a few differences.
Programmatically, the data structures are defined as follows:

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

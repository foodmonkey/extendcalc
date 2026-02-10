Location of files

Since you're building a PathBuf based on specific directories, are you using
std::env::current_dir or are you targeting the standard Linux XDG directories
(like ~/.config/extendcalc) for your RON files?

Thinking
That is the classic "Last Mile" problem in app development. When you move from
cargo run to a packaged app, relative paths like ./data will break because the
app's working directory changes.
For a COSMIC or Linux-first app, you should use the XDG Base Directory Specification.

1. The Pro Way: Use the directories crate
Don't write path logic yourself. Add the directories crate to your Cargo.toml.
It handles Linux, macOS, and Windows automatically.

```Rust
let project_dirs = ProjectDirs::from("com", "github", "foodmonkey").unwrap();
let config_dir = project_dirs.config_dir(); // ~/.config/foodmonkey
```

2.The "COSMIC" Way: cosmic-config
Since you are already in the libcosmic ecosystem, the toolkit actually has its
own cosmic-config system designed specifically for this. It handles the pathing
and the RON parsing for you.

3.Quick Abstracted Path Builder
If you want to keep your current logic, use a conditional to switch between
Debug (local folder) and Release (system folder):

```Rust
fn get_data_path() -> PathBuf {
    if cfg!(debug_assertions) {
        // While developing in Zed
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data")
    } else {
        // When packaged
        directories::ProjectDirs::from("com", "foodmonkey", "extendcalc")
            .unwrap()
            .data_dir()
            .to_path_buf()
    }
}
```

4.Handling Assets in the Binary
If those RON files are static templates (meaning the user doesn't edit them),
you can bypass the file system entirely using include_str!:

```Rust
const DEFAULT_KEYPAD: &str = include_str!("../data/default.ron");
// Now you can parse this string directly in your Task
```

Where to put the files on Linux:

User Config: ~/.config/extendcalc/ (User's custom keypads)
System Data: /usr/share/extendcalc/ (Default keypads)

Do you want these keypad files to be editable by the user, or are they strictly
internal resources for the app?
Proactive Follow-up: I can show you how to set up a build script (build.rs) if
you need to copy these files to the correct system location automatically
during installation

the RON libraries need to be in their own little ecosystem because users can
add keys and keypads to the application - so either external importing or
through the editor to create custom user keypads using keys from any other
keypad

Thinking
That changes everything—you aren't just building a static app; you're building a
customisable platform. Since users can "remix" keys into new keypads, your path
logic needs to be rock-solid to prevent "missing file" errors.
1.The Directory Strategy
Since users are creating content, you should use the user-local data directory.
This ensures the app has permission to write files without needing sudo.
I recommend the directories crate for this. It handles the cross-platform
headache for you.

```Rust
// Use this in your path_builder
let proj_dirs = ProjectDirs::from("com", "foodmonkey", "extendcalc")
    .expect("Could not find home directory");

let user_keypads_dir = proj_dirs.data_dir().join("keypads");
let user_keys_dir = proj_dirs.data_dir().join("keys");
```

2.Cross-Referencing Logic (The "Ecosystem")
Since a Keypad references Keys from other files, your KeypadModel RON should
probably store IDs (filenames) rather than full paths.

Internal Keys: Shipped with the app in /usr/share/extendcalc/keys.
User Keys: Saved in ~/.local/share/extendcalc/keys.

Your load_and_parse helper should look in the User folder first (allowing users
to "override" default keys) and fall back to the System folder.
3. Handling the "Import"
When a user "adds" a key via your editor, your Task should:

Copy the external .ron file into your user_keys_dir.
Verify the parse is successful using your load_and_parse helper.
Update the AppModel via a Message so the new key appears in the UI immediately.

4.Zed Productivity Tip
Since you'll be juggling many RON files, you can add RON support to Zed by
ensuring the extension is installed. This gives you syntax highlighting for
your data files right next to your Rust code.

Thinking
That image perfectly illustrates the file structure we were just discussing.
You have all your core data definitions stored neatly in development folders
(data, keypad_definitions, library, ui). To move this from your development
environment (where you currently use relative paths) to a production app that
uses the XDG Base Directory system we discussed, you will need a reliable way
to copy these default files during installation.
The "Build Script" Solution
The most robust way to ensure these default files exist in the right user
location is to run a setup or copy script after the application is installed.
When you package your app (e.g., as a .deb or Flatpak file for Linux), the
packaging system will place the default read-only files in a system-wide
location like:

/usr/share/extendcalc/data/...
/usr/share/extendcalc/keypad_definitions/...

Then, when your application first runs, your init function needs to check if the
user's writable directory (e.g., ~/.local/share/extendcalc/) exists.
If it doesn't, your app creates it and copies the system defaults over.
This gives you your desired ecosystem:

System Defaults (Read-Only): Safely stored away from user edits.
User Data (Read/Write): Where users can add their custom keys and keypads.

Would you like a template for a simple build.rs script that can automatically
copy those files to a target directory during your project build?

Here is a template for a build.rs script that copies your data files to the
output directory every time you build your project. This ensures the files
are always available alongside your executable, whether you're running in debug
or release mode.
1.Add Build Dependencies
First, you'll need the fs_extra crate for easy recursive directory copying.
Update your Cargo.toml with a new section:

```toml
[build-dependencies]
fs_extra = "1.3.0"
```

2.Create the build.rs File
Create a file named build.rs in the root of your project directory
(the same place as Cargo.toml).
Cargo automatically detects and runs this file before building your
main app.

Paste the following code into your new build.rs:

```Rust
use std::env;
use std::path::PathBuf;
use fs_extra::dir::{copy, create_dir_all, CopyOptions};

fn main() -> std::io::Result<()> {
    // Tell Cargo that if any file in the 'data' 
    // directory changes, rerun this build script.
    println!("cargo:rerun-if-changed=data");
    println!("cargo:rerun-if-changed=keypad_definitions");
    println!("cargo:rerun-if-changed=library");
    println!("cargo:rerun-if-changed=ui");

    // Get the base directory of your project.
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let out_path = PathBuf::from(out_dir);

    // Define source directories based on your project structure:
    let data_src = PathBuf::from(&manifest_dir).join("data");
    let keypads_src = PathBuf::from(&manifest_dir).join("keypad_definitions");
    let library_src = PathBuf::from(&manifest_dir).join("library");
    let ui_src = PathBuf::from(&manifest_dir).join("ui");
    
    // Define destination directories inside the output path:
    let data_dst = out_path.join("data");
    let keypads_dst = out_path.join("keypad_definitions");
    let library_dst = out_path.join("library");
    let ui_dst = out_path.join("ui");

    // Copy options for recursive copying.
    let options = CopyOptions::new();
    
    // Create destination directories if they don't exist
    create_dir_all(&data_dst, options)?;
    create_dir_all(&keypads_dst, options)?;
    create_dir_all(&library_dst, options)?;
    create_dir_all(&ui_dst, options)?;

    // Copy the directories recursively
    copy(data_src, data_dst, options)?;
    copy(keypads_src, keypads_dst, options)?;
    copy(library_src, library_dst, options)?;
    copy(ui_src, ui_dst, options)?;

    Ok(())
}
```

3.Update Your App's Path Logic
Now, in your main application code (in the Task that runs your init logic),
when you use your path_builder helper, you must reference the files using the
CARGO_MANIFEST_DIR environment variable during development, or the correct
system path when deployed.
The build.rs script has made all these files available relative to the
executable's location within the target/debug or target/release folders.

```Rust
// In your main app logic, use std::env for path building:
use std::env;

fn get_path_to_ron_file(filename: &str) -> PathBuf {
    // This points to the root of where cargo places your app's built assets/executable.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // Example: building path to a 'business.ron' file
    PathBuf::from(manifest_dir).join("ui").join(filename)
}
```

This setup ensures that your application finds the files reliably when you
run cargo run in Zed.

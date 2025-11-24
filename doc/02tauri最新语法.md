

---

## 📄 文件: autostart.mdx

---

```mdx
---
title: Autostart
description: Automatically launch your app at system startup.
plugin: autostart
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Automatically launch your application at system startup.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the autostart plugin to get started.

<Tabs>
  <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    {' '}

    <CommandTabs
      npm="npm run tauri add autostart"
      yarn="yarn run tauri add autostart"
      pnpm="pnpm tauri add autostart"
      deno="deno task tauri add autostart"
      bun="bun tauri add autostart"
      cargo="cargo tauri add autostart"
    />

  </TabItem>
    <TabItem label="Manual">
      <Steps>

        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-autostart --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={5-6}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .setup(|app| {
                        #[cfg(desktop)]
                        app.handle().plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"]) /* arbitrary number of args to pass to your app */));
                        Ok(())
                    })
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

        3.  You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

            <CommandTabs
                npm="npm install @tauri-apps/plugin-autostart"
                yarn="yarn add @tauri-apps/plugin-autostart"
                pnpm="pnpm add @tauri-apps/plugin-autostart"
                deno="deno add npm:@tauri-apps/plugin-autostart"
                bun="bun add @tauri-apps/plugin-autostart"
            />

      </Steps>
    </TabItem>

</Tabs>

## Usage

The autostart plugin is available in both JavaScript and Rust.

<Tabs syncKey="lang">
  <TabItem label="JavaScript">

```javascript
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
// when using `"withGlobalTauri": true`, you may use
// const { enable, isEnabled, disable } = window.__TAURI__.autostart;

// Enable autostart
await enable();
// Check enable state
console.log(`registered for autostart? ${await isEnabled()}`);
// Disable autostart
disable();
```

  </TabItem>
  <TabItem label="Rust">

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    Some(vec!["--flag1", "--flag2"]),
                ));

                // Get the autostart manager
                let autostart_manager = app.autolaunch();
                // Enable autostart
                let _ = autostart_manager.enable();
                // Check enable state
                println!("registered for autostart? {}", autostart_manager.is_enabled().unwrap());
                // Disable autostart
                let _ = autostart_manager.disable();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

  </TabItem>
</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json"
{
  "permissions": [
    ...,
    "autostart:allow-enable",
    "autostart:allow-disable",
    "autostart:allow-is-enabled"
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: barcode-scanner.mdx

---

```mdx
---
title: Barcode Scanner
description: Allows your mobile application to use the camera to scan QR codes, EAN-13 and other types of barcodes.
plugin: barcode-scanner
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Allows your mobile application to use the camera to scan QR codes, EAN-13 and other kinds of barcodes.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the barcode-scanner plugin to get started.

<Tabs>
  <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    {' '}

    <CommandTabs
      npm="npm run tauri add barcode-scanner"
      yarn="yarn run tauri add barcode-scanner"
      pnpm="pnpm tauri add barcode-scanner"
      deno="deno task tauri add barcode-scanner"
      bun="bun tauri add barcode-scanner"
      cargo="cargo tauri add barcode-scanner"
    />

  </TabItem>
  <TabItem label="Manual">
    <Steps>

      1.  Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

          ```sh frame=none
          cargo add tauri-plugin-barcode-scanner --target 'cfg(any(target_os = "android", target_os = "ios"))'
          ```

      2.  Modify `lib.rs` to initialize the plugin:

          ```rust title="src-tauri/src/lib.rs" ins={5-6}
          #[cfg_attr(mobile, tauri::mobile_entry_point)]
          pub fn run() {
              tauri::Builder::default()
                  .setup(|app| {
                      #[cfg(mobile)]
                      app.handle().plugin(tauri_plugin_barcode_scanner::init());
                      Ok(())
                  })
                  .run(tauri::generate_context!())
                  .expect("error while running tauri application");
          }
          ```

      3.  Install the JavaScript Guest bindings using your preferred JavaScript package manager:

          <CommandTabs
              npm="npm install @tauri-apps/plugin-barcode-scanner"
              yarn="yarn add @tauri-apps/plugin-barcode-scanner"
              pnpm="pnpm add @tauri-apps/plugin-barcode-scanner"
              deno="deno add npm:@tauri-apps/plugin-barcode-scanner"
              bun="bun add @tauri-apps/plugin-barcode-scanner"
          />

    </Steps>

  </TabItem>
</Tabs>

## Configuration

On iOS the barcode scanner plugin requires the `NSCameraUsageDescription` information property list value, which should describe why your app needs to use the camera.

In the `src-tauri/Info.ios.plist` file, add the following snippet:

```xml title=src-tauri/Info.ios.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
	<dict>
		<key>NSCameraUsageDescription</key>
		<string>Read QR codes</string>
	</dict>
</plist>
```

## Usage

The barcode scanner plugin is available in JavaScript.

```javascript
import { scan, Format } from '@tauri-apps/plugin-barcode-scanner';
// when using `"withGlobalTauri": true`, you may use
// const { scan, Format } = window.__TAURI__.barcodeScanner;

// `windowed: true` actually sets the webview to transparent
// instead of opening a separate view for the camera
// make sure your user interface is ready to show what is underneath with a transparent element
scan({ windowed: true, formats: [Format.QRCode] });
```

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/mobile.json"
{
  "$schema": "../gen/schemas/mobile-schema.json",
  "identifier": "mobile-capability",
  "windows": ["main"],
  "platforms": ["iOS", "android"],
  "permissions": ["barcode-scanner:allow-scan", "barcode-scanner:allow-cancel"]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: biometric.mdx

---

```mdx
---
title: Biometric
description: Prompt the user for biometric authentication on Android and iOS.
plugin: biometric
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Prompt the user for biometric authentication on Android and iOS.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the biometric plugin to get started.

<Tabs>
    <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    <CommandTabs npm="npm run tauri add biometric"
    yarn="yarn run tauri add biometric"
    pnpm="pnpm tauri add biometric"
    deno="deno task tauri add biometric"
    bun="bun tauri add biometric"
    cargo="cargo tauri add biometric" />

    </TabItem>
    <TabItem label="Manual">
        <Steps>
        1.  Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-biometric --target 'cfg(any(target_os = "android", target_os = "ios"))'
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={5-6}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .setup(|app| {
                        #[cfg(mobile)]
                        app.handle().plugin(tauri_plugin_biometric::Builder::new().build());
                        Ok(())
                    })
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

        3.  Install the JavaScript Guest bindings using your preferred JavaScript package manager:

            <CommandTabs
            npm = "npm install @tauri-apps/plugin-biometric"
            yarn = "yarn add @tauri-apps/plugin-biometric"
            pnpm = "pnpm add @tauri-apps/plugin-biometric"
            deno = "deno add npm:@tauri-apps/plugin-biometric"
            bun = "bun add @tauri-apps/plugin-biometric"
            />
        </Steps>
    </TabItem>

</Tabs>

## Configuration

On iOS the biometric plugin requires the `NSFaceIDUsageDescription` information property list value, which should describe why your app needs to use biometric authentication.

In the `src-tauri/Info.ios.plist` file, add the following snippet:

```xml title=src-tauri/Info.ios.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
	<dict>
		<key>NSFaceIDUsageDescription</key>
		<string>Authenticate with biometric</string>
	</dict>
</plist>
```

## Usage

This plugin enables you to verify the availability of Biometric Authentication on a device, prompt the user for biometric authentication, and check the result to determine if the authentication was successful or not.

### Check Status

You can check the status of Biometric Authentication, including its availability and the types of biometric authentication methods supported.

<Tabs syncKey="lang">
<TabItem label="JavaScript">

```javascript
import { checkStatus } from '@tauri-apps/plugin-biometric';

const status = await checkStatus();
if (status.isAvailable) {
  console.log('Yes! Biometric Authentication is available');
} else {
  console.log(
    'No! Biometric Authentication is not available due to ' + status.error
  );
}
```

</TabItem>
<TabItem label="Rust">

```rust
use tauri_plugin_biometric::BiometricExt;

fn check_biometric(app_handle: tauri::AppHandle) {
    let status = app_handle.biometric().status().unwrap();
    if status.is_available {
        println!("Yes! Biometric Authentication is available");
    } else {
        println!("No! Biometric Authentication is not available due to: {}", status.error.unwrap());
    }
}
```

</TabItem>
</Tabs>

### Authenticate

To prompt the user for Biometric Authentication, utilize the `authenticate()` method.

<Tabs syncKey="lang">

<TabItem label="JavaScript">

```javascript ins={18}
import { authenticate } from '@tauri-apps/plugin-biometric';

const options = {
  // Set true if you want the user to be able to authenticate using phone password
  allowDeviceCredential: false,
  cancelTitle: "Feature won't work if Canceled",

  // iOS only feature
  fallbackTitle: 'Sorry, authentication failed',

  // Android only features
  title: 'Tauri feature',
  subtitle: 'Authenticate to access the locked Tauri function',
  confirmationRequired: true,
};

try {
  await authenticate('This feature is locked', options);
  console.log(
    'Hooray! Successfully Authenticated! We can now perform the locked Tauri function!'
  );
} catch (err) {
  console.log('Oh no! Authentication failed because ' + err.message);
}
```

</TabItem>

<TabItem label="Rust">

```rust ins={21}
use tauri_plugin_biometric::{BiometricExt, AuthOptions};

fn bio_auth(app_handle: tauri::AppHandle) {

    let options = AuthOptions {
        // Set True if you want the user to be able to authenticate using phone password
        allow_device_credential:false,
        cancel_title: Some("Feature won't work if Canceled".to_string()),

        // iOS only feature
        fallback_title: Some("Sorry, authentication failed".to_string()),

        // Android only features
        title: Some("Tauri feature".to_string()),
        subtitle: Some("Authenticate to access the locked Tauri function".to_string()),
        confirmation_required: Some(true),
    };

    // if the authentication was successful, the function returns Result::Ok()
    // otherwise returns Result::Error()
    match app_handle.biometric().authenticate("This feature is locked".to_string(), options) {
        Ok(_) => {
            println!("Hooray! Successfully Authenticated! We can now perform the locked Tauri function!");
        }
        Err(e) => {
            println!("Oh no! Authentication failed because : {e}");
        }
    }
}
```

</TabItem>
</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={6}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["biometric:default"]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: cli.mdx

---

```mdx
---
title: Command Line Interface (CLI)
description: Parse arguments from the command line interface.
plugin: cli
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Tauri enables your app to have a CLI through [clap](https://github.com/clap-rs/clap), a robust command line argument parser. With a simple CLI definition in your `tauri.conf.json` file, you can define your interface and read its argument matches map on JavaScript and/or Rust.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

- Windows
  - Due to an OS limitation, production apps are not able to write text back to the calling console by default. Please check out [tauri#8305](https://github.com/tauri-apps/tauri/issues/8305#issuecomment-1826871949) for a workaround.{/* TODO: Inline the instructions into this guide */}

## Setup

Install the CLI plugin to get started.

<Tabs>
	<TabItem label="Automatic">

    	Use your project's package manager to add the dependency:

    	<CommandTabs
            npm="npm run tauri add cli"
            yarn="yarn run tauri add cli"
            pnpm="pnpm tauri add cli"
            deno="deno task tauri add cli"
            bun="bun tauri add cli"
            cargo="cargo tauri add cli"
    	/>

  	</TabItem>
    <TabItem label="Manual">
    	<Steps>

        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-cli --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
            ```

    		2.	Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={5-6}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .setup(|app| {
                        #[cfg(desktop)]
                        app.handle().plugin(tauri_plugin_cli::init());
                        Ok(())
                    })
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

    		3.	Install the JavaScript Guest bindings using your preferred JavaScript package manager:

            <CommandTabs
                npm="npm install @tauri-apps/plugin-cli"
                yarn="yarn add @tauri-apps/plugin-cli"
                pnpm="pnpm add @tauri-apps/plugin-cli"
                deno="deno add npm:@tauri-apps/plugin-cli"
                bun="bun add @tauri-apps/plugin-cli"
            />

    	</Steps>
    </TabItem>

</Tabs>

## Base Configuration

Under `tauri.conf.json`, you have the following structure to configure the interface:

```json title="src-tauri/tauri.conf.json"
{
  "plugins": {
    "cli": {
      "description": "Tauri CLI Plugin Example",
      "args": [
        {
          "short": "v",
          "name": "verbose",
          "description": "Verbosity level"
        }
      ],
      "subcommands": {
        "run": {
          "description": "Run the application",
          "args": [
            {
              "name": "debug",
              "description": "Run application in debug mode"
            },
            {
              "name": "release",
              "description": "Run application in release mode"
            }
          ]
        }
      }
    }
  }
}
```

:::note

All JSON configurations here are just samples, many other fields have been omitted for the sake of clarity.

:::

## Adding Arguments

The `args` array represents the list of arguments accepted by its command or subcommand.

{/* TODO: List available configuration */}

### Positional Arguments

A positional argument is identified by its position in the list of arguments. With the following configuration:

```json title="src-tauri/tauri.conf.json"
{
  "args": [
    {
      "name": "source",
      "index": 1,
      "takesValue": true
    },
    {
      "name": "destination",
      "index": 2,
      "takesValue": true
    }
  ]
}
```

Users can run your app as `./app tauri.txt dest.txt` and the arg matches map will define `source` as `"tauri.txt"` and `destination` as `"dest.txt"`.

### Named Arguments

A named argument is a (key, value) pair where the key identifies the value. With the following configuration:

```json title="tauri-src/tauri.conf.json"
{
  "args": [
    {
      "name": "type",
      "short": "t",
      "takesValue": true,
      "multiple": true,
      "possibleValues": ["foo", "bar"]
    }
  ]
}
```

Users can run your app as `./app --type foo bar`, `./app -t foo -t bar` or `./app --type=foo,bar` and the arg matches map will define `type` as `["foo", "bar"]`.

### Flag Arguments

A flag argument is a standalone key whose presence or absence provides information to your application. With the following configuration:

```json title="tauri-src/tauri.conf.json"
{
  "args": [
    {
      "name": "verbose",
      "short": "v"
    }
  ]
}
```

Users can run your app as `./app -v -v -v`, `./app --verbose --verbose --verbose` or `./app -vvv` and the arg matches map will define `verbose` as `true`, with `occurrences = 3`.

## Subcommands

Some CLI applications have additional interfaces as subcommands. For instance, the `git` CLI has `git branch`, `git commit` and `git push`. You can define additional nested interfaces with the `subcommands` array:

```json title="tauri-src/tauri.conf.json"
{
  "cli": {
    ...
    "subcommands": {
      "branch": {
        "args": []
      },
      "push": {
        "args": []
      }
    }
  }
}
```

Its configuration is the same as the root application configuration, with the `description`, `longDescription`, `args`, etc.

## Usage

The CLI plugin is available in both JavaScript and Rust.

<Tabs syncKey="lang">
  <TabItem label="JavaScript">

```javascript
import { getMatches } from '@tauri-apps/plugin-cli';
// when using `"withGlobalTauri": true`, you may use
// const { getMatches } = window.__TAURI__.cli;

const matches = await getMatches();
if (matches.subcommand?.name === 'run') {
  // `./your-app run $ARGS` was executed
  const args = matches.subcommand.matches.args;
  if (args.debug?.value === true) {
    // `./your-app run --debug` was executed
  }
  if (args.release?.value === true) {
    // `./your-app run --release` was executed
  }
}
```

  </TabItem>
  <TabItem label="Rust">

```rust title="src-tauri/src/lib.rs" {1, 6-18}
use tauri_plugin_cli::CliExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
   tauri::Builder::default()
       .plugin(tauri_plugin_cli::init())
       .setup(|app| {
           match app.cli().matches() {
               // `matches` here is a Struct with { args, subcommand }.
               // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
               // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
               Ok(matches) => {
                   println!("{:?}", matches)
               }
               Err(_) => {}
           }
           Ok(())
       })
       .run(tauri::generate_context!())
       .expect("error while running tauri application");
}
```

  </TabItem>
</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={6}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["cli:default"]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: clipboard.mdx

---

```mdx
---
title: Clipboard
description: Read and write to the system clipboard.
plugin: clipboard-manager
i18nReady: true
---

import Stub from '@components/Stub.astro';
import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Read and write to the system clipboard using the clipboard plugin.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the clipboard plugin to get started.

<Tabs>
    <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    <CommandTabs npm="npm run tauri add clipboard-manager"
    yarn="yarn run tauri add clipboard-manager"
    pnpm="pnpm tauri add clipboard-manager"
    bun="bun tauri add clipboard-manager"
    deno="deno task tauri add clipboard-manager"
    cargo="cargo tauri add clipboard-manager" />

    </TabItem>
    <TabItem label="Manual">
        <Steps>

        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-clipboard-manager
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={4}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .plugin(tauri_plugin_clipboard_manager::init())
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

        3.  If you'd like to manage the clipboard in JavaScript then install the npm package as well:

            <CommandTabs
                npm="npm install @tauri-apps/plugin-clipboard-manager"
                yarn="yarn add @tauri-apps/plugin-clipboard-manager"
                pnpm="pnpm add @tauri-apps/plugin-clipboard-manager"
                deno="deno add npm:@tauri-apps/plugin-clipboard-manager"
                bun="bun add @tauri-apps/plugin-clipboard-manager"
            />

        </Steps>
    </TabItem>

</Tabs>

## Usage

{/* TODO: Link to which language to use, frontend vs. backend guide when it's made */}

The clipboard plugin is available in both JavaScript and Rust.

<Tabs syncKey="lang">
<TabItem label="JavaScript">

```javascript
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';
// when using `"withGlobalTauri": true`, you may use
// const { writeText, readText } = window.__TAURI__.clipboardManager;

// Write content to clipboard
await writeText('Tauri is awesome!');

// Read content from clipboard
const content = await readText();
console.log(content);
// Prints "Tauri is awesome!" to the console
```

</TabItem>
<TabItem label="Rust">

```rust
use tauri_plugin_clipboard_manager::ClipboardExt;

app.clipboard().write_text("Tauri is awesome!".to_string()).unwrap();

// Read content from clipboard
let content = app.clipboard().read_text();
println!("{:?}", content.unwrap());
// Prints "Tauri is awesome!" to the terminal


```

</TabItem>
</Tabs>

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: deep-linking.mdx

---

```mdx
---
title: Deep Linking
description: Set your Tauri application as the default handler for an URL.
plugin: deep-link
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Set your Tauri application as the default handler for an URL.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the deep-link plugin to get started.

<Tabs>
  <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    {' '}

    <CommandTabs
      npm="npm run tauri add deep-link"
      yarn="yarn run tauri add deep-link"
      pnpm="pnpm tauri add deep-link"
      deno="deno task tauri add deep-link"
      bun="bun tauri add deep-link"
      cargo="cargo tauri add deep-link"
    />

  </TabItem>
  <TabItem label="Manual">
    <Steps>

      1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

          ```sh frame=none
          cargo add tauri-plugin-deep-link@2.0.0
          ```

      2.  Modify `lib.rs` to initialize the plugin:

          ```rust title="src-tauri/src/lib.rs" ins={4}
          #[cfg_attr(mobile, tauri::mobile_entry_point)]
          pub fn run() {
              tauri::Builder::default()
                  .plugin(tauri_plugin_deep_link::init())
                  .run(tauri::generate_context!())
                  .expect("error while running tauri application");
          }
          ```

      3.  Install the JavaScript Guest bindings using your preferred JavaScript package manager:

          <CommandTabs
              npm="npm install @tauri-apps/plugin-deep-link"
              yarn="yarn add @tauri-apps/plugin-deep-link"
              pnpm="pnpm add @tauri-apps/plugin-deep-link"
              deno="deno add npm:@tauri-apps/plugin-deep-link"
              bun="bun add @tauri-apps/plugin-deep-link"
          />

    </Steps>

  </TabItem>
</Tabs>

## Setting up

### Android

There are two ways to open your app from links on Android:

1. **App Links (http/https + host, verified)**
   For [app links](https://developer.android.com/training/app-links#android-app-links), you need a server with a
   `.well-known/assetlinks.json` endpoint that must return a text response in the given format:

```json title=".well-known/assetlinks.json"
[
  {
    "relation": ["delegate_permission/common.handle_all_urls"],
    "target": {
      "namespace": "android_app",
      "package_name": "$APP_BUNDLE_ID",
      "sha256_cert_fingerprints": [
        $CERT_FINGERPRINT
      ]
    }
  }
]
```

Where `$APP_BUNDLE_ID` is the value defined on [`tauri.conf.json > identifier`] with `-` replaced with `_` and
`$CERT_FINGERPRINT` is a list of SHA256 fingerprints of your app's signing certificates,
see [verify Android applinks] for more information.

2. **Custom URI schemes (no host required, no verification)**
   For URIs like `myapp://...`, you can declare a custom scheme without hosting any files. Use the `scheme` field in the mobile configuration and omit the `host`.

### iOS

There are two ways to open your app from links on iOS:

1. **Universal Links (https + host, verified)**
   For [universal links], you need a server with a `.well-known/apple-app-site-association` endpoint that must return a JSON response
   in the given format:

```json title=".well-known/apple-app-site-association"
{
  "applinks": {
    "details": [
      {
        "appIDs": ["$DEVELOPMENT_TEAM_ID.$APP_BUNDLE_ID"],
        "components": [
          {
            "/": "/open/*",
            "comment": "Matches any URL whose path starts with /open/"
          }
        ]
      }
    ]
  }
}
```

:::note
The response `Content-Type` header must be `application/json`.

The `.well-known/apple-app-site-association` endpoint must be served over HTTPS.
To test localhost you can either use a self-signed TLS certificate and install it on the iOS simulator or use services like [ngrok].
:::

Where `$DEVELOPMENT_TEAM_ID` is the value defined on `tauri.conf.json > tauri > bundle > iOS > developmentTeam` or the
`TAURI_APPLE_DEVELOPMENT_TEAM` environment variable and `$APP_BUNDLE_ID` is the value defined on [`tauri.conf.json > identifier`].

To verify if your domain has been properly configured to expose the app associations, you can run the following command,
replacing `<host>` with your actual host:

```sh
curl -v https://app-site-association.cdn-apple.com/a/v1/<host>
```

See [applinks.details](https://developer.apple.com/documentation/bundleresources/applinks/details-swift.dictionary) for more information.

2. **Custom URI schemes (no host, no verification)**
   For URIs like `myapp://...`, you can declare a custom scheme under mobile configuration with `"appLink": false` (or omit it). The plugin generates the appropriate `CFBundleURLTypes` entries in your app's Info.plist. No `.well-known` files or HTTPS host are needed.

### Desktop

On Linux and Windows deep links are delivered as a command line argument to a new app process.
The deep link plugin has integration with the [single instance] plugin if you prefer having a unique app instance receiving the events.

- First you must add the `deep-link` feature to the single instance plugin:

```toml title="src-tauri/Cargo.toml"
[target."cfg(any(target_os = \"macos\", windows, target_os = \"linux\"))".dependencies]
tauri-plugin-single-instance = { version = "2.0.0", features = ["deep-link"] }
```

- Then configure the single instance plugin which should always be the first plugin you register:

```rust title="src-tauri/lib.rs"
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
          println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");
          // when defining deep link schemes at runtime, you must also check `argv` here
        }));
    }

    builder = builder.plugin(tauri_plugin_deep_link::init());
}
```

:::warn
The user could trigger a fake deep link manually by including the URL as argument.
Tauri matches the command line argument against the configured schemes to mitigate this,
but you should still check if the URL matches the format you expect.

This means Tauri only handles deep links for schemes that were statically configured,
and schemes registered at runtime must be manually checked using [`Env::args_os`].
:::

## Configuration

Under `tauri.conf.json > plugins > deep-link`, configure mobile domains/schemes and desktop schemes you want to associate with your application.

### Examples

**Custom scheme on mobile (no server required):**

```json title="tauri.conf.json"
{
  "plugins": {
    "deep-link": {
      "mobile": [
        {
          "scheme": ["ovi"],
          "appLink": false
        }
      ]
    }
  }
}
```

This registers the `ovi://*` scheme on Android and iOS.

**App Link / Universal Link (verified https + host):**

```json
{
  "plugins": {
    "deep-link": {
      "mobile": [
        {
          "scheme": ["https"],
          "host": "your.website.com",
          "pathPrefix": ["/open"],
          "appLink": true
        }
      ]
    }
  }
}
```

This registers `https://your.website.com/open/*` as an app/universal link.

**Desktop custom schemes:**

```json
{
  "plugins": {
    "deep-link": {
      "desktop": {
        "schemes": ["something", "my-tauri-app"]
      }
    }
  }
}
```

## Usage

The deep-link plugin is available in both JavaScript and Rust.

### Listening to Deep Links

<Tabs syncKey="lang">
  <TabItem label="JavaScript">

When a deep link triggers your app while it's running, the `onOpenUrl` callback is called. To detect whether your app was opened via a deep link, use `getCurrent` on app start.

```javascript
import { getCurrent, onOpenUrl } from '@tauri-apps/plugin-deep-link';
// when using `"withGlobalTauri": true`, you may use
// const { getCurrent, onOpenUrl } = window.__TAURI__.deepLink;

const startUrls = await getCurrent();
if (startUrls) {
  // App was likely started via a deep link
  // Note that getCurrent's return value will also get updated every time onOpenUrl gets triggered.
}

await onOpenUrl((urls) => {
  console.log('deep link:', urls);
});
```

  </TabItem>
  <TabItem label="Rust">

When a deep link triggers your app while it's running, the plugin's `on_open_url` closure is called. To detect whether your app was opened via a deep link, use `get_current` on app start.

```rust title="src-tauri/src/lib.rs"
use tauri_plugin_deep_link::DeepLinkExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            // Note that get_current's return value will also get updated every time on_open_url gets triggered.
            let start_urls = app.deep_link().get_current()?;
            if let Some(urls) = start_urls {
                // app was likely started by a deep link
                println!("deep link URLs: {:?}", urls);
            }

            app.deep_link().on_open_url(|event| {
                println!("deep link URLs: {:?}", event.urls());
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

  </TabItem>
</Tabs>

:::note
The open URL event is triggered with a list of URLs that were requested to be compatible with the macOS API for deep links,
but in most cases your app will only receive a single URL.
:::

### Registering Desktop Deep Links at Runtime

The [configuration](#configuration) section describes how to define static deep link schemes for your application.

On Linux and Windows it is possible to also associate schemes with your application at runtime via the `register` Rust function.

In the following snippet, we will register the `my-app` scheme at runtime. After executing the app for the first time,
the operating system will open `my-app://*` URLs with our application:

```rust title="src-tauri/src/lib.rs"
use tauri_plugin_deep_link::DeepLinkExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            #[cfg(desktop)]
            app.deep_link().register("my-app")?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

:::note
Registering the deep links at runtime can be useful for developing on Linux and Windows
as by default the deep link is only registered when your app is installed.

Installing an AppImage can be complicated as it requires an AppImage launcher.

Registering the deep links at runtime might be preferred, so Tauri also includes a
helper function to force register all statically configured deep links at runtime.
Calling this function also ensures the deep links is registered for development mode:

```rust
#[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
{
  use tauri_plugin_deep_link::DeepLinkExt;
  app.deep_link().register_all()?;
}
```

:::

## Testing

There are some caveats to test deep links for your application.

### Desktop

Deep links are only triggered for installed applications on desktop.
On Linux and Windows you can circumvent this using the [`register_all`] Rust function,
which registers all configured schemes to trigger the current executable:

```rust title="src-tauri/src/lib.rs"
use tauri_plugin_deep_link::DeepLinkExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            #[cfg(any(windows, target_os = "linux"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                app.deep_link().register_all()?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

:::note
Installing an AppImage that supports deep links on Linux requires an AppImage launcher to integrate the AppImage with the operating system.
Using the `register_all` function you can support deep links out of the box, without requiring your users to use external tools.

When the AppImage is moved to a different location in the file system, the deep link is invalidated since it leverages an absolute path
to the executable, which makes registering the schemes at runtime even more important.

See the [Registering Desktop Deep Links at Runtime](#registering-desktop-deep-links-at-runtime) section for more information.
:::

:::caution
Registering deep links at runtime is not possible on macOS, so deep links can only be tested on the bundled application,
which must be installed in the `/Applications` directory.
:::

#### Windows

To trigger a deep link on Windows you can either open `<scheme>://url` in the browser or run the following command in the terminal:

```sh
start <scheme>://url
```

#### Linux

To trigger a deep link on Linux you can either open `<scheme>://url` in the browser or run `xdg-open` in the terminal:

```sh
xdg-open <scheme>://url
```

### iOS

To trigger an app link on iOS you can open the `https://<host>/path` URL in the browser. For simulators you can leverage the `simctl` CLI to directly open a link from the terminal:

```sh
xcrun simctl openurl booted https://<host>/path
```

### Android

To trigger an app link on Android you can open the `https://<host>/path` URL in the browser. For emulators you can leverage the `adb` CLI to directly open a link from the terminal:

```sh
adb shell am start -a android.intent.action.VIEW -d https://<host>/path <bundle-identifier>
```

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={9}
{
  "$schema": "../gen/schemas/mobile-schema.json",
  "identifier": "mobile-capability",
  "windows": ["main"],
  "platforms": ["iOS", "android"],
  "permissions": [
    // Usually you will need core:event:default to listen to the deep-link event
    "core:event:default",
    "deep-link:default"
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />

[`tauri.conf.json > identifier`]: /reference/config/#identifier
[verify Android applinks]: https://developer.android.com/training/app-links/verify-android-applinks#web-assoc
[universal links]: https://developer.apple.com/documentation/xcode/allowing-apps-and-websites-to-link-to-your-content?language=objc
[single instance]: /plugin/single-instance/
[`register_all`]: https://docs.rs/tauri-plugin-deep-link/2.0.0/tauri_plugin_deep_link/struct.DeepLink.html#method.register_all
[ngrok]: https://ngrok.com/
[`Env::args_os`]: https://docs.rs/tauri/2.0.0/tauri/struct.Env.html#structfield.args_os
```

---

## 📄 文件: dialog.mdx

---

```mdx
---
title: Dialog
description: Native system dialogs for opening and saving files along with message dialogs.
i18nReady: true
tableOfContents:
  maxHeadingLevel: 4
plugin: dialog
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Native system dialogs for opening and saving files along with message dialogs.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the dialog plugin to get started.

<Tabs>
    <TabItem label="Automatic">

        Use your project's package manager to add the dependency:

        <CommandTabs npm="npm run tauri add dialog"
                    yarn="yarn run tauri add dialog"
                    pnpm="pnpm tauri add dialog"
                    deno="deno task tauri add dialog"
                    bun="bun tauri add dialog"
                    cargo="cargo tauri add dialog" />

    </TabItem>
    <TabItem label="Manual">
        <Steps>

            1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

                ```sh frame=none
                cargo add tauri-plugin-dialog
                ```

            2.  Modify `lib.rs` to initialize the plugin:

                ```rust title="src-tauri/src/lib.rs" ins={4}
                #[cfg_attr(mobile, tauri::mobile_entry_point)]
                pub fn run() {
                    tauri::Builder::default()
                        .plugin(tauri_plugin_dialog::init())
                        .run(tauri::generate_context!())
                        .expect("error while running tauri application");
                }
                ```

            3.  If you'd like create dialogs in JavaScript, install the npm package as well:

                <CommandTabs
                    npm="npm install @tauri-apps/plugin-dialog"
                    yarn="yarn add @tauri-apps/plugin-dialog"
                    pnpm="pnpm add @tauri-apps/plugin-dialog"
                    deno="deno add npm:@tauri-apps/plugin-dialog"
                    bun="bun add @tauri-apps/plugin-dialog"
                />

        </Steps>
    </TabItem>

</Tabs>

## Usage

The dialog plugin is available in both JavaScript and Rust. Here's how you can use it:

in JavaScript:

- [Create Yes/No Dialog](#create-yesno-dialog)
- [Create Ok/Cancel Dialog](#create-okcancel-dialog)
- [Create Message Dialog](#create-message-dialog)
- [Open a File Selector Dialog](#open-a-file-selector-dialog)
- [Save to File Dialog](#save-to-file-dialog)

in Rust:

- [Build an Ask Dialog](#build-an-ask-dialog)
- [Build a Message Dialog](#build-a-message-dialog)
- [Build a File Selector Dialog](#build-a-file-selector-dialog)

:::note
The file dialog APIs returns file system paths on Linux, Windows and macOS.

On iOS, a `file://<path>` URIs are returned.

On Android, [content URIs] are returned.

The [filesystem plugin] works with any path format out of the box.
:::

### JavaScript

See all [Dialog Options](/reference/javascript/dialog/) at the JavaScript API reference.

#### Create Yes/No Dialog

Shows a question dialog with `Yes` and `No` buttons.

```javascript
import { ask } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { ask } = window.__TAURI__.dialog;

// Create a Yes/No dialog
const answer = await ask('This action cannot be reverted. Are you sure?', {
  title: 'Tauri',
  kind: 'warning',
});

console.log(answer);
// Prints boolean to the console
```

#### Create Ok/Cancel Dialog

Shows a question dialog with `Ok` and `Cancel` buttons.

```javascript
import { confirm } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { confirm } = window.__TAURI__.dialog;

// Creates a confirmation Ok/Cancel dialog
const confirmation = await confirm(
  'This action cannot be reverted. Are you sure?',
  { title: 'Tauri', kind: 'warning' }
);

console.log(confirmation);
// Prints boolean to the console
```

#### Create Message Dialog

Shows a message dialog with an `Ok` button. Keep in mind that if the user closes the dialog it will return `false`.

```javascript
import { message } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { message } = window.__TAURI__.dialog;

// Shows message
await message('File not found', { title: 'Tauri', kind: 'error' });
```

#### Open a File Selector Dialog

Open a file/directory selection dialog.

The `multiple` option controls whether the dialog allows multiple selection or not, while the `directory`, whether is a directory selection or not.

```javascript
import { open } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { open } = window.__TAURI__.dialog;

// Open a dialog
const file = await open({
  multiple: false,
  directory: false,
});
console.log(file);
// Prints file path or URI
```

#### Save to File Dialog

Open a file/directory save dialog.

```javascript
import { save } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
// const { save } = window.__TAURI__.dialog;

// Prompt to save a 'My Filter' with extension .png or .jpeg
const path = await save({
  filters: [
    {
      name: 'My Filter',
      extensions: ['png', 'jpeg'],
    },
  ],
});
console.log(path);
// Prints the chosen path
```

---

### Rust

Refer to the [Rust API reference](https://docs.rs/tauri-plugin-dialog/) to see all available options.

#### Build an Ask Dialog

Shows a question dialog with `Absolutely` and `Totally` buttons.

```rust
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

let answer = app.dialog()
        .message("Tauri is Awesome")
        .title("Tauri is Awesome")
        .buttons(MessageDialogButtons::OkCancelCustom("Absolutely", "Totally"))
        .blocking_show();
```

If you need a non blocking operation you can use `show()` instead:

```rust
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

app.dialog()
    .message("Tauri is Awesome")
    .title("Tauri is Awesome")
   .buttons(MessageDialogButtons::OkCancelCustom("Absolutely", "Totally"))
    .show(|result| match result {
        true => // do something,
        false =>// do something,
    });
```

#### Build a Message Dialog

Shows a message dialog with an `Ok` button. Keep in mind that if the user closes the dialog it will return `false`.

```rust
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

let ans = app.dialog()
    .message("File not found")
    .kind(MessageDialogKind::Error)
    .title("Warning")
    .blocking_show();
```

If you need a non blocking operation you can use `show()` instead:

```rust
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

app.dialog()
    .message("Tauri is Awesome")
    .kind(MessageDialogKind::Info)
    .title("Information")
    .buttons(MessageDialogButtons::OkCustom("Absolutely"))
    .show(|result| match result {
        true => // do something,
        false => // do something,
    });
```

#### Build a File Selector Dialog

#### Pick Files

```rust
use tauri_plugin_dialog::DialogExt;

let file_path = app.dialog().file().blocking_pick_file();
// return a file_path `Option`, or `None` if the user closes the dialog
```

If you need a non blocking operation you can use `pick_file()` instead:

```rust
use tauri_plugin_dialog::DialogExt;

app.dialog().file().pick_file(|file_path| {
    // return a file_path `Option`, or `None` if the user closes the dialog
    })
```

#### Save Files

```rust
use tauri_plugin_dialog::DialogExt;

let file_path = app
    .dialog()
    .file()
    .add_filter("My Filter", &["png", "jpeg"])
    .blocking_save_file();
    // do something with the optional file path here
    // the file path is `None` if the user closed the dialog
```

or, alternatively:

```rust
use tauri_plugin_dialog::DialogExt;

app.dialog()
    .file()
    .add_filter("My Filter", &["png", "jpeg"])
    .pick_file(|file_path| {
        // return a file_path `Option`, or `None` if the user closes the dialog
    });
```

<PluginPermissions plugin={frontmatter.plugin} />

[content URIs]: https://developer.android.com/guide/topics/providers/content-provider-basics
[filesystem plugin]: /plugin/file-system/
```

---

## 📄 文件: file-system.mdx

---

```mdx
---
title: File System
description: Access the file system.
plugin: fs
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Access the file system.

:::note[Use std::fs or tokio::fs on the Rust side]
If you want to manipulate files/directories through Rust, use traditional Rust's libs (std::fs, tokio::fs, etc).

:::

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the fs plugin to get started.

<Tabs>
	<TabItem label="Automatic" >

    	Use your project's package manager to add the dependency:

    	{ ' ' }

    	<CommandTabs
            npm="npm run tauri add fs"
            yarn="yarn run tauri add fs"
            pnpm="pnpm tauri add fs"
            deno="deno task tauri add fs"
            bun="bun tauri add fs"
            cargo="cargo tauri add fs"
    	/>

    </TabItem>
    <TabItem label = "Manual">
    	<Steps>

        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-fs
            ```

        2.	Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={4}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
              tauri::Builder::default()
                .plugin(tauri_plugin_fs::init())
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
            }
            ```

        3.	Install the JavaScript Guest bindings using your preferred JavaScript package manager:

            <CommandTabs
                npm = "npm install @tauri-apps/plugin-fs"
                yarn = "yarn add @tauri-apps/plugin-fs"
                pnpm = "pnpm add @tauri-apps/plugin-fs"
                deno = "deno add npm:@tauri-apps/plugin-fs"
                bun = "bun add @tauri-apps/plugin-fs"
            />

    	</Steps>
    </TabItem>

</Tabs>

## Configuration

### Android

When using the audio, cache, documents, downloads, picture, public or video directories your app must have access to the external storage.

Include the following permissions to the `manifest` tag in the `gen/android/app/src/main/AndroidManifest.xml` file:

```xml
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE"/>
<uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
```

### iOS

Apple requires app developers to specify approved reasons for API usage to enhance user privacy.

You must create a `PrivacyInfo.xcprivacy` file in the `src-tauri/gen/apple` folder
with the required [NSPrivacyAccessedAPICategoryFileTimestamp] key and the [C617.1] recommended reason.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>NSPrivacyAccessedAPITypes</key>
    <array>
      <dict>
        <key>NSPrivacyAccessedAPIType</key>
        <string>NSPrivacyAccessedAPICategoryFileTimestamp</string>
        <key>NSPrivacyAccessedAPITypeReasons</key>
        <array>
          <string>C617.1</string>
        </array>
      </dict>
    </array>
  </dict>
</plist>
```

## Usage

The fs plugin is available in both JavaScript and Rust.
:::caution[Different APIs]
Although this plugin has a file manipulation API on the frontend, in the backend it offers only the methods to change permission of some
resources (files, directories, etc).

In the Rust side you can use the traditional file manipulation libraries,
[std::fs](https://doc.rust-lang.org/std/fs/struct.File.html), [tokio::fs](https://docs.rs/tokio/latest/tokio/fs/index.html) or others.

:::

<Tabs syncKey="lang">
  <TabItem label="JavaScript" >

```javascript
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';
// when using `"withGlobalTauri": true`, you may use
// const { exists, BaseDirectory } = window.__TAURI__.fs;

// Check if the `$APPDATA/avatar.png` file exists
await exists('avatar.png', { baseDir: BaseDirectory.AppData });
```

  </TabItem>
  <TabItem label = "Rust" >

```rust title="src-tauri/src/lib.rs"
use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
      .plugin(tauri_plugin_fs::init())
      .setup(|app| {
          // allowed the given directory
          let scope = app.fs_scope();
        	scope.allow_directory("/path/to/directory", false);
          dbg!(scope.allowed());

          Ok(())
       })
       .run(tauri::generate_context!())
       .expect("error while running tauri application");
}
```

  </TabItem>
</Tabs>

## Security

This module prevents path traversal, not allowing parent directory accessors to be used
(i.e. "/usr/path/to/../file" or "../path/to/file" paths are not allowed).
Paths accessed with this API must be either relative to one of the [base directories][base directory]
or created with the [path API].

See [@tauri-apps/plugin-fs - Security] for more information.

## Paths

The file system plugin offers two ways of manipulating paths: the [base directory] and the [path API].

- base directory

  Every API has an options argument that lets you define a [baseDir][base directory] that acts as the working directory of the operation.

  ```js
  import { readFile } from '@tauri-apps/plugin-fs';
  const contents = await readFile('avatars/tauri.png', {
    baseDir: BaseDirectory.Home,
  });
  ```

  In the above example the ~/avatars/tauri.png file is read since we are using the **Home** base directory.

- path API

  Alternatively you can use the path APIs to perform path manipulations.

  ```js
  import { readFile } from '@tauri-apps/plugin-fs';
  import * as path from '@tauri-apps/api/path';
  const home = await path.homeDir();
  const contents = await readFile(await path.join(home, 'avatars/tauri.png'));
  ```

## Files

### Create

Creates a file and returns a handle to it. If the file already exists, it is truncated.

```js
import { create, BaseDirectory } from '@tauri-apps/plugin-fs';
const file = await create('foo/bar.txt', { baseDir: BaseDirectory.AppData });
await file.write(new TextEncoder().encode('Hello world'));
await file.close();
```

:::note
Always call `file.close()` when you are done manipulating the file.
:::

### Write

The plugin offers separate APIs for writing text and binary files for performance.

- text files

  ```js
  import { writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
  const contents = JSON.stringify({ notifications: true });
  await writeTextFile('config.json', contents, {
    baseDir: BaseDirectory.AppConfig,
  });
  ```

- binary files

  ```js
  import { writeFile, BaseDirectory } from '@tauri-apps/plugin-fs';
  const contents = new Uint8Array(); // fill a byte array
  await writeFile('config', contents, {
    baseDir: BaseDirectory.AppConfig,
  });
  ```

### Open

Opens a file and returns a handle to it.
With this API you have more control over how the file should be opened
(read-only mode, write-only mode, append instead of overwrite, only create if it does not exist, etc).

:::note
Always call `file.close()` when you are done manipulating the file.
:::

- read-only

  This is the default mode.

  ```js
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
  const file = await open('foo/bar.txt', {
    read: true,
    baseDir: BaseDirectory.AppData,
  });

  const stat = await file.stat();
  const buf = new Uint8Array(stat.size);
  await file.read(buf);
  const textContents = new TextDecoder().decode(buf);
  await file.close();
  ```

- write-only

  ```js
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
  const file = await open('foo/bar.txt', {
    write: true,
    baseDir: BaseDirectory.AppData,
  });
  await file.write(new TextEncoder().encode('Hello world'));
  await file.close();
  ```

  By default the file is truncated on any `file.write()` call.
  See the following example to learn how to append to the existing contents instead.

- append

  ```js
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
  const file = await open('foo/bar.txt', {
    append: true,
    baseDir: BaseDirectory.AppData,
  });
  await file.write(new TextEncoder().encode('world'));
  await file.close();
  ```

  Note that `{ append: true }` has the same effect as `{ write: true, append: true }`.

- truncate

  When the `truncate` option is set and the file already exists, it will be truncated to length 0.

  ```js
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
  const file = await open('foo/bar.txt', {
    write: true,
    truncate: true,
    baseDir: BaseDirectory.AppData,
  });
  await file.write(new TextEncoder().encode('world'));
  await file.close();
  ```

  This option requires `write` to be `true`.

  You can use it along the `append` option if you want to rewrite an existing file using multiple `file.write()` calls.

- create

  By default the `open` API only opens existing files. To create the file if it does not exist,
  opening it if it does, set `create` to `true`:

  ```js
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
  const file = await open('foo/bar.txt', {
    write: true,
    create: true,
    baseDir: BaseDirectory.AppData,
  });
  await file.write(new TextEncoder().encode('world'));
  await file.close();
  ```

  In order for the file to be created, `write` or `append` must also be set to `true`.

  To fail if the file already exists, see `createNew`.

- createNew

  `createNew` works similarly to `create`, but will fail if the file already exists.

  ```js
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';
  const file = await open('foo/bar.txt', {
    write: true,
    createNew: true,
    baseDir: BaseDirectory.AppData,
  });
  await file.write(new TextEncoder().encode('world'));
  await file.close();
  ```

  In order for the file to be created, `write` must also be set to `true`.

### Read

The plugin offers separate APIs for reading text and binary files for performance.

- text files

  ```js
  import { readTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
  const configToml = await readTextFile('config.toml', {
    baseDir: BaseDirectory.AppConfig,
  });
  ```

  If the file is large you can stream its lines with the `readTextFileLines` API:

  ```typescript
  import { readTextFileLines, BaseDirectory } from '@tauri-apps/plugin-fs';
  const lines = await readTextFileLines('app.logs', {
    baseDir: BaseDirectory.AppLog,
  });
  for await (const line of lines) {
    console.log(line);
  }
  ```

- binary files

  ```js
  import { readFile, BaseDirectory } from '@tauri-apps/plugin-fs';
  const icon = await readFile('icon.png', {
    baseDir: BaseDirectory.Resources,
  });
  ```

### Remove

Call `remove()` to delete a file. If the file does not exist, an error is returned.

```js
import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';
await remove('user.db', { baseDir: BaseDirectory.AppLocalData });
```

### Copy

The `copyFile` function takes the source and destination paths.
Note that you must configure each base directory separately.

```js
import { copyFile, BaseDirectory } from '@tauri-apps/plugin-fs';
await copyFile('user.db', 'user.db.bk', {
  fromPathBaseDir: BaseDirectory.AppLocalData,
  toPathBaseDir: BaseDirectory.Temp,
});
```

In the above example the \<app-local-data\>/user.db file is copied to $TMPDIR/user.db.bk.

### Exists

Use the `exists()` function to check if a file exists:

```js
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';
const tokenExists = await exists('token', {
  baseDir: BaseDirectory.AppLocalData,
});
```

### Metadata

File metadata can be retrieved with the `stat` and the `lstat` functions.
`stat` follows symlinks (and returns an error if the actual file it points to is not allowed by the scope)
and `lstat` does not follow symlinks, returning the information of the symlink itself.

```js
import { stat, BaseDirectory } from '@tauri-apps/plugin-fs';
const metadata = await stat('app.db', {
  baseDir: BaseDirectory.AppLocalData,
});
```

### Rename

The `rename` function takes the source and destination paths.
Note that you must configure each base directory separately.

```js
import { rename, BaseDirectory } from '@tauri-apps/plugin-fs';
await rename('user.db.bk', 'user.db', {
  fromPathBaseDir: BaseDirectory.AppLocalData,
  toPathBaseDir: BaseDirectory.Temp,
});
```

In the above example the \<app-local-data\>/user.db.bk file is renamed to $TMPDIR/user.db.

### Truncate

Truncates or extends the specified file to reach the specified file length (defaults to 0).

- truncate to 0 length

```typescript
import { truncate } from '@tauri-apps/plugin-fs';
await truncate('my_file.txt', 0, { baseDir: BaseDirectory.AppLocalData });
```

- truncate to a specific length

```typescript
import {
  truncate,
  readTextFile,
  writeTextFile,
  BaseDirectory,
} from '@tauri-apps/plugin-fs';

const filePath = 'file.txt';
await writeTextFile(filePath, 'Hello World', {
  baseDir: BaseDirectory.AppLocalData,
});
await truncate(filePath, 7, {
  baseDir: BaseDirectory.AppLocalData,
});
const data = await readTextFile(filePath, {
  baseDir: BaseDirectory.AppLocalData,
});
console.log(data); // "Hello W"
```

## Directories

### Create

To create a directory, call the `mkdir` function:

```js
import { mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';
await mkdir('images', {
  baseDir: BaseDirectory.AppLocalData,
});
```

### Read

The `readDir` function recursively lists the entries of a directory:

```typescript
import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs';
const entries = await readDir('users', { baseDir: BaseDirectory.AppLocalData });
```

### Remove

Call `remove()` to delete a directory. If the directory does not exist, an error is returned.

```js
import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';
await remove('images', { baseDir: BaseDirectory.AppLocalData });
```

If the directory is not empty, the `recursive` option must be set to `true`:

```js
import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';
await remove('images', {
  baseDir: BaseDirectory.AppLocalData,
  recursive: true,
});
```

### Exists

Use the `exists()` function to check if a directory exists:

```js
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';
const tokenExists = await exists('images', {
  baseDir: BaseDirectory.AppLocalData,
});
```

### Metadata

Directory metadata can be retrieved with the `stat` and the `lstat` functions.
`stat` follows symlinks (and returns an error if the actual file it points to is not allowed by the scope)
and `lstat` does not follow symlinks, returning the information of the symlink itself.

```js
import { stat, BaseDirectory } from '@tauri-apps/plugin-fs';
const metadata = await stat('databases', {
  baseDir: BaseDirectory.AppLocalData,
});
```

## Watching changes

To watch a directory or file for changes, use the `watch` or `watchImmediate` functions.

- watch

  `watch` is debounced so it only emits events after a certain delay:

  ```js
  import { watch, BaseDirectory } from '@tauri-apps/plugin-fs';
  await watch(
    'app.log',
    (event) => {
      console.log('app.log event', event);
    },
    {
      baseDir: BaseDirectory.AppLog,
      delayMs: 500,
    }
  );
  ```

- watchImmediate

  `watchImmediate` immediately notifies listeners of an event:

  ```js
  import { watchImmediate, BaseDirectory } from '@tauri-apps/plugin-fs';
  await watchImmediate(
    'logs',
    (event) => {
      console.log('logs directory event', event);
    },
    {
      baseDir: BaseDirectory.AppLog,
      recursive: true,
    }
  );
  ```

By default watch operations on a directory are not recursive.
Set the `recursive` option to `true` to recursively watch for changes on all sub-directories.

:::note
The watch functions require the `watch` feature flag:

```toml title="src-tauri/Cargo.toml"
[dependencies]
tauri-plugin-fs = { version = "2.0.0", features = ["watch"] }
```

:::

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={7-11}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "fs:default",
    {
      "identifier": "fs:allow-exists",
      "allow": [{ "path": "$APPDATA/*" }]
    }
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />

### Scopes

This plugin permissions includes scopes for defining which paths are allowed or explicitly rejected.
For more information on scopes, see the [Command Scopes].

Each `allow` or `deny` scope must include an array listing all paths that should be allowed or denied.
The scope entries are in the `{ path: string }` format.

:::note
`deny` take precedence over `allow` so if a path is denied by a scope, it will be blocked at runtime
even if it is allowed by another scope.
:::

Scope entries can use `$<path>` variables to reference common system paths such as the home directory,
the app resources directory and the config directory. The following table lists all common paths you can reference:

| Path                                                                                            | Variable      |
| ----------------------------------------------------------------------------------------------- | ------------- |
| [appConfigDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appconfigdir)       | $APPCONFIG    |
| [appDataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appdatadir)           | $APPDATA      |
| [appLocalDataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appLocaldatadir) | $APPLOCALDATA |
| [appcacheDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appcachedir)         | $APPCACHE     |
| [applogDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#applogdir)             | $APPLOG       |
| [audioDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#audiodir)               | $AUDIO        |
| [cacheDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#cachedir)               | $CACHE        |
| [configDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#configdir)             | $CONFIG       |
| [dataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#datadir)                 | $DATA         |
| [localDataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#localdatadir)       | $LOCALDATA    |
| [desktopDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#desktopdir)           | $DESKTOP      |
| [documentDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#documentdir)         | $DOCUMENT     |
| [downloadDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#downloaddir)         | $DOWNLOAD     |
| [executableDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#executabledir)     | $EXE          |
| [fontDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#fontdir)                 | $FONT         |
| [homeDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#homedir)                 | $HOME         |
| [pictureDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#picturedir)           | $PICTURE      |
| [publicDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#publicdir)             | $PUBLIC       |
| [runtimeDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#runtimedir)           | $RUNTIME      |
| [templateDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#templatedir)         | $TEMPLATE     |
| [videoDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#videodir)               | $VIDEO        |
| [resourceDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#resourcedir)         | $RESOURCE     |
| [tempDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#tempdir)                 | $TEMP         |

#### Examples

- global scope

To apply a scope to any `fs` command, use the `fs:scope` permission:

```json title="src-tauri/capabilities/default.json" {7-10}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "fs:scope",
      "allow": [{ "path": "$APPDATA" }, { "path": "$APPDATA/**/*" }]
    }
  ]
}
```

To apply a scope to a specific `fs` command,
use the the object form of permissions `{ "identifier": string, "allow"?: [], "deny"?: [] }`:

```json title="src-tauri/capabilities/default.json" {7-18}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "fs:allow-rename",
      "allow": [{ "path": "$HOME/**/*" }]
    },
    {
      "identifier": "fs:allow-rename",
      "deny": [{ "path": "$HOME/.config/**/*" }]
    },
    {
      "identifier": "fs:allow-exists",
      "allow": [{ "path": "$APPDATA/*" }]
    }
  ]
}
```

In the above example you can use the [`exists`](#exists) API using any `$APPDATA` sub path (does not include sub-directories)
and the [`rename`](#rename)

:::tip
If you are trying to access dotfiles (e.g. `.gitignore`) or dotfolders (e.g. `.ssh`) on Unix based systems,
then you need to specify either the full path `/home/user/.ssh/example` or the glob after the dotfolder path
component `/home/user/.ssh/*`.

If that does not work in your use case then you can configure the plugin to treat any component
as a valid path literal.

```json title="src-tauri/tauri.conf.json
 "plugins": {
    "fs": {
      "requireLiteralLeadingDot": false
    }
  }
```

:::

[NSPrivacyAccessedAPICategoryFileTimestamp]: https://developer.apple.com/documentation/bundleresources/privacy_manifest_files/describing_use_of_required_reason_api#4278393
[C617.1]: https://developer.apple.com/documentation/bundleresources/privacy_manifest_files/describing_use_of_required_reason_api#4278393
[base directory]: /reference/javascript/api/namespacepath/#basedirectory
[path API]: /reference/javascript/api/namespacepath/
[@tauri-apps/plugin-fs - Security]: /reference/javascript/fs/#security
[Command Scopes]: /security/scope/
```

---

## 📄 文件: global-shortcut.mdx

---

```mdx
---
title: Global Shortcut
description: Register global shortcuts.
plugin: global-shortcut
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Register global shortcuts.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the global-shortcut plugin to get started.

<Tabs>
  <TabItem label="Automatic" >

    Use your project's package manager to add the dependency:

    { ' ' }

    <CommandTabs
      npm="npm run tauri add global-shortcut"
      yarn="yarn run tauri add global-shortcut"
      pnpm="pnpm tauri add global-shortcut"
      deno="deno task tauri add global-shortcut"
      bun="bun tauri add global-shortcut"
      cargo="cargo tauri add global-shortcut"
    />

  </TabItem>
  <TabItem label = "Manual">
    <Steps>

    1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

        ```sh frame=none
        cargo add tauri-plugin-global-shortcut --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
        ```

    2.  Modify `lib.rs` to initialize the plugin:

        ```rust title="src-tauri/src/lib.rs" ins={4-5}
        pub fn run() {
            tauri::Builder::default()
                .setup(|app| {
                    #[cfg(desktop)]
                    app.handle().plugin(tauri_plugin_global_shortcut::Builder::new().build());
                    Ok(())
                })
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        ```

    3.  Install the JavaScript Guest bindings using your preferred JavaScript package manager:

        <CommandTabs
          npm = "npm install @tauri-apps/plugin-global-shortcut"
          yarn = "yarn add @tauri-apps/plugin-global-shortcut"
          pnpm = "pnpm add @tauri-apps/plugin-global-shortcut"
          deno = "deno add npm:@tauri-apps/plugin-global-shortcut"
          bun = "bun add @tauri-apps/plugin-global-shortcut"
        />

    </Steps>

  </TabItem>
</Tabs>

## Usage

The global-shortcut plugin is available in both JavaScript and Rust.

<Tabs syncKey="lang">
  <TabItem label="JavaScript" >

```javascript
import { register } from '@tauri-apps/plugin-global-shortcut';
// when using `"withGlobalTauri": true`, you may use
// const { register } = window.__TAURI__.globalShortcut;

await register('CommandOrControl+Shift+C', () => {
  console.log('Shortcut triggered');
});
```

  </TabItem>
  <TabItem label = "Rust" >

```rust title="src-tauri/src/lib.rs"
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
                        println!("{:?}", shortcut);
                        if shortcut == &ctrl_n_shortcut {
                            match event.state() {
                              ShortcutState::Pressed => {
                                println!("Ctrl-N Pressed!");
                              }
                              ShortcutState::Released => {
                                println!("Ctrl-N Released!");
                              }
                            }
                        }
                    })
                    .build(),
                )?;

                app.global_shortcut().register(ctrl_n_shortcut)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

  </TabItem>
</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={7-9}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "global-shortcut:allow-is-registered",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister"
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: http-client.mdx

---

```mdx
---
title: HTTP Client
description: Access the HTTP client written in Rust.
plugin: http
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Make HTTP requests with the http plugin.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the http plugin to get started.

<Tabs>
    <TabItem label="Automatic">

        Use your project's package manager to add the dependency:

        <CommandTabs npm="npm run tauri add http"
        yarn="yarn run tauri add http"
        pnpm="pnpm tauri add http"
        deno="deno task tauri add http"
        bun="bun tauri add http"
        cargo="cargo tauri add http" />

    </TabItem>
    <TabItem label="Manual">
        <Steps>

        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-http
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={4}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .plugin(tauri_plugin_http::init())
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

        3.  If you'd like to make http requests in JavaScript then install the npm package as well:

            <CommandTabs
                npm="npm install @tauri-apps/plugin-http"
                yarn="yarn add @tauri-apps/plugin-http"
                pnpm="pnpm add @tauri-apps/plugin-http"
                deno="deno add npm:@tauri-apps/plugin-http"
                bun="bun add @tauri-apps/plugin-http"
            />

        </Steps>
    </TabItem>

</Tabs>

## Usage

The HTTP plugin is available in both Rust as a [reqwest](https://docs.rs/reqwest/) re-export and JavaScript.

### JavaScript

<Steps>

1.  Configure the allowed URLs

    ```json
    //src-tauri/capabilities/default.json
    {
      "permissions": [
        {
          "identifier": "http:default",
          "allow": [{ "url": "https://*.tauri.app" }],
          "deny": [{ "url": "https://private.tauri.app" }]
        }
      ]
    }
    ```

    For more information, please see the documentation for [Permissions Overview](/security/permissions/)

2.  Send a request

    The `fetch` method tries to be as close and compliant to the [`fetch` Web API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) as possible.

    ```javascript
    import { fetch } from '@tauri-apps/plugin-http';

    // Send a GET request
    const response = await fetch('http://test.tauri.app/data.json', {
      method: 'GET',
    });
    console.log(response.status); // e.g. 200
    console.log(response.statusText); // e.g. "OK"
    ```

    :::note

    [Forbidden request headers] are ignored by default. To use them you must enable the `unsafe-headers` feature flag:

    ```toml title="src-tauri/Cargo.toml"
    [dependencies]
    tauri-plugin-http = { version = "2", features = ["unsafe-headers"] }
    ```

    :::

</Steps>

### Rust

In Rust you can utilize the `reqwest` crate re-exported by the plugin. For more details refer to [reqwest docs](https://docs.rs/reqwest/).

```rust
use tauri_plugin_http::reqwest;

let res = reqwest::get("http://my.api.host/data.json").await;
println!("{:?}", res.status()); // e.g. 200
println!("{:?}", res.text().await); // e.g Ok("{ Content }")
```

<PluginPermissions plugin={frontmatter.plugin} />

[Forbidden request headers]: https://fetch.spec.whatwg.org/#terminology-headers
```

---

## 📄 文件: index.mdx

---

```mdx
---
title: Features & Recipes
i18nReady: true
sidebar:
  label: Overview
---

import { LinkCard } from '@astrojs/starlight/components';
import FeaturesList from '@components/list/Features.astro';
import CommunityList from '@components/list/Community.astro';
import Search from '@components/CardGridSearch.astro';
import AwesomeTauri from '@components/AwesomeTauri.astro';
import TableCompatibility from '@components/plugins/TableCompatibility.astro';

Tauri comes with extensibility in mind. On this page you'll find:

- **[Features](#features)**: Built-in Tauri features and functionality
- **[Community Resources](#community-resources)**: More plugins and recipes built by the Tauri community

<Search>
  ## Features
  <FeaturesList />
  ## Community Resources
  <LinkCard
    title="Have something to share?"
    description="Open a pull request to show us your amazing resource."
    href="https://github.com/tauri-apps/awesome-tauri/pulls"
  />
  ### Plugins
  <AwesomeTauri section="plugins-no-official" />
  ### Integrations
  <AwesomeTauri section="integrations" />
</Search>

## Support Table

Hover "\*" to see notes. For more details visit the plugin page

<TableCompatibility />
```

---

## 📄 文件: localhost.mdx

---

```mdx
---
title: Localhost
description: Use a localhost server in production apps.
plugin: localhost
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';

<PluginLinks plugin={frontmatter.plugin} showJsLinks={false} />

Expose your app's assets through a localhost server instead of the default custom protocol.

:::caution
This plugin brings considerable security risks and you should only use it if you know what you are doing. If in doubt, use the default custom protocol implementation.
:::

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the localhost plugin to get started.

<Tabs>
    <TabItem label="Automatic">

        Use your project's package manager to add the dependency:

        <CommandTabs npm="npm run tauri add localhost"
        yarn="yarn run tauri add localhost"
        pnpm="pnpm tauri add localhost"
        deno="deno task tauri add localhost"
        bun="bun tauri add localhost"
        cargo="cargo tauri add localhost" />

    </TabItem>
    <TabItem label="Manual">

        <Steps>

        1.  Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-localhost
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={4}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .plugin(tauri_plugin_localhost::Builder::new().build())
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

        </Steps>

</TabItem>
</Tabs>

## Usage

The localhost plugin is available in Rust.

```rust title="src-tauri/src/lib.rs" {4} {7-14}
use tauri::{webview::WebviewWindowBuilder, WebviewUrl};

pub fn run() {
  let port: u16 = 9527;

  tauri::Builder::default()
      .plugin(tauri_plugin_localhost::Builder::new(port).build())
      .setup(move |app| {
          let url = format!("http://localhost:{}", port).parse().unwrap();
          WebviewWindowBuilder::new(app, "main".to_string(), WebviewUrl::External(url))
              .title("Localhost Example")
              .build()?;
          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
```
```

---

## 📄 文件: logging.mdx

---

```mdx
---
title: Logging
description: Configurable logging.
plugin: log
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Configurable logging for your Tauri app.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the log plugin to get started.

<Tabs>
  <TabItem label="Automatic" >

    Use your project's package manager to add the dependency:

    { ' ' }

    <CommandTabs
      npm="npm run tauri add log"
      yarn="yarn run tauri add log"
      pnpm="pnpm tauri add log"
      deno="deno task tauri add log"
      bun="bun tauri add log"
      cargo="cargo tauri add log"
    />

  </TabItem>
  <TabItem label = "Manual">
    <Steps>

    1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

        ```sh frame=none
        cargo add tauri-plugin-log
        ```

    2.  Modify `lib.rs` to initialize the plugin:

        ```rust title="src-tauri/src/lib.rs" ins={4}
        #[cfg_attr(mobile, tauri::mobile_entry_point)]
        pub fn run() {
            tauri::Builder::default()
                .plugin(tauri_plugin_log::Builder::new().build())
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        ```

    3.  Install the JavaScript Guest bindings using your preferred JavaScript package manager:

        <CommandTabs
          npm = "npm install @tauri-apps/plugin-log"
          yarn = "yarn add @tauri-apps/plugin-log"
          pnpm = "pnpm add @tauri-apps/plugin-log"
          deno = "deno add npm:@tauri-apps/plugin-log"
          bun = "bun add @tauri-apps/plugin-log"
        />

    </Steps>

  </TabItem>
</Tabs>

## Usage

  <Steps>

1.  First, you need to register the plugin with Tauri.

    ```rust title="src-tauri/src/lib.rs" {1} {6-14}
    use tauri_plugin_log::{Target, TargetKind};

    #[cfg_attr(mobile, tauri::mobile_entry_point)]
    pub fn run() {
        tauri::Builder::default()
            .plugin(tauri_plugin_log::Builder::new().build())
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    ```

2.  Afterwards, all the plugin's APIs are available through the JavaScript guest bindings:

    ```javascript
    import {
      warn,
      debug,
      trace,
      info,
      error,
      attachConsole,
      attachLogger,
    } from '@tauri-apps/plugin-log';
    // when using `"withGlobalTauri": true`, you may use
    // const { warn, debug, trace, info, error, attachConsole, attachLogger } = window.__TAURI__.log;
    ```

  </Steps>

## Logging

<Tabs syncKey="lang">
  <TabItem label="JavaScript">
Use one of the plugin's `warn`, `debug`, `trace`, `info` or  `error` APIs to produce a log record from JavaScript code:

```js
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

trace('Trace');
info('Info');
error('Error');
```

To automatically forward all `console` messages to the log plugin you can rewrite them:

```ts
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

function forwardConsole(
  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  logger: (message: string) => Promise<void>
) {
  const original = console[fnName];
  console[fnName] = (message) => {
    original(message);
    logger(message);
  };
}

forwardConsole('log', trace);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);
```

</TabItem>

<TabItem label="Rust">
To create your own logs on the Rust side you can use the [`log` crate]:

```rust
log::error!("something bad happened!");
log::info!("Tauri is awesome!");
```

Note that the [`log` crate] must be added to your `Cargo.toml` file:

```toml
[dependencies]
log = "0.4"
```

</TabItem>
</Tabs>

## Log targets

The log plugin builder has a `targets` function that lets you configure common destination of all your application logs.

:::note
By default the plugin logs to stdout and to a file in the application logs directory.
To only use your own log targets, call `clear_targets`:

```rust
tauri_plugin_log::Builder::new()
.clear_targets()
.build()
```

:::

### Printing logs to the terminal

To forward all your logs to the terminal, enable the `Stdout` or `Stderr` targets:

```rust
tauri_plugin_log::Builder::new()
  .target(tauri_plugin_log::Target::new(
    tauri_plugin_log::TargetKind::Stdout,
  ))
  .build()
```

This target is enabled by default.

### Logging to the webview console

To view all your Rust logs in the webview console, enable the `Webview` target and run `attachConsole` in your frontend:

```rust
tauri_plugin_log::Builder::new()
  .target(tauri_plugin_log::Target::new(
    tauri_plugin_log::TargetKind::Webview,
  ))
  .build()
```

```js
import { attachConsole } from '@tauri-apps/plugin-log';
const detach = await attachConsole();
// call detach() if you do not want to print logs to the console anymore
```

### Persisting logs

To write all logs to a file, you can use either the `LogDir` or the `Folder` targets.

- `LogDir`:

```rust
tauri_plugin_log::Builder::new()
  .target(tauri_plugin_log::Target::new(
    tauri_plugin_log::TargetKind::LogDir {
      file_name: Some("logs".to_string()),
    },
  ))
  .build()
```

When using the LogDir target, all logs are stored in the recommended log directory.
The following table describes the location of the logs per platform:

| Platform | Value                                                                                    | Example                                           |
| -------- | ---------------------------------------------------------------------------------------- | ------------------------------------------------- |
| Linux    | `$XDG_DATA_HOME/{bundleIdentifier}/logs` or `$HOME/.local/share/{bundleIdentifier}/logs` | `/home/alice/.local/share/com.tauri.dev/logs`     |
| macOS    | `{homeDir}/Library/Logs/{bundleIdentifier}`                                              | `/Users/Alice/Library/Logs/com.tauri.dev`         |
| Windows  | `{FOLDERID_LocalAppData}/{bundleIdentifier}/logs`                                        | `C:\Users\Alice\AppData\Local\com.tauri.dev\logs` |

- `Folder`:

The Folder target lets you write logs to a custom location in the filesystem.

```rust
tauri_plugin_log::Builder::new()
  .target(tauri_plugin_log::Target::new(
    tauri_plugin_log::TargetKind::Folder {
      path: std::path::PathBuf::from("/path/to/logs"),
      file_name: None,
    },
  ))
  .build()
```

The default `file_name` is the application name.

#### Configuring log file behavior

By default the log file gets discarded when it reaches the maximum size.
The maximum file size can be configured via the builder's `max_file_size` function:

```rust
tauri_plugin_log::Builder::new()
  .max_file_size(50_000 /* bytes */)
  .build()
```

Tauri can automatically rotate your log file when it reaches the size limit instead of discarding the previous file.
This behavior can be configured using `rotation_strategy`:

```rust
tauri_plugin_log::Builder::new()
  .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
  .build()
```

### Filtering

By default **all** logs are processed. There are some mechanisms to reduce the amount of logs and filter only relevant information.

### Maximum log level

To set a maximum log level, use the `level` function:

```rust
tauri_plugin_log::Builder::new()
  .level(log::LevelFilter::Info)
  .build()
```

In this example, debug and trace logs are discarded as they have a lower level than _info_.

It is also possible to define separate maximum levels for individual modules:

```rust
tauri_plugin_log::Builder::new()
  .level(log::LevelFilter::Info)
  // verbose logs only for the commands module
  .level_for("my_crate_name::commands", log::LevelFilter::Trace)
  .build()
```

Note that these APIs use the [`log` crate], which must be added to your `Cargo.toml` file:

```toml
[dependencies]
log = "0.4"
```

### Target filter

A `filter` function can be defined to discard unwanted logs by checking their metadata:

```rust
tauri_plugin_log::Builder::new()
  // exclude logs with target `"hyper"`
  .filter(|metadata| metadata.target() != "hyper")
  .build()
```

### Formatting

The log plugin formats each log record as `DATE[TARGET][LEVEL] MESSAGE`.
A custom format function can be provided with `format`:

```rust
tauri_plugin_log::Builder::new()
  .format(|out, message, record| {
    out.finish(format_args!(
      "[{} {}] {}",
      record.level(),
      record.target(),
      message
    ))
  })
  .build()
```

#### Log dates

By default the log plugin uses the UTC timezone to format dates
but you can configure it to use the local timezone with `timezone_strategy`:

```rust
tauri_plugin_log::Builder::new()
  .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
  .build()
```

## Permissions

By default, all plugin commands are blocked and cannot be accessed.
You must define a list of permissions in your `capabilities` configuration.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={6}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["log:default"]
}
```

<PluginPermissions plugin={frontmatter.plugin} />

[`log` crate]: https://crates.io/crates/log
```

---

## 📄 文件: nfc.mdx

---

```mdx
---
title: NFC
description: Read and write NFC tags on Android and iOS.
plugin: nfc
i18nReady: true
---

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';
import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

<PluginLinks plugin={frontmatter.plugin} />

Read and write NFC tags on Android and iOS.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the nfc plugin to get started.

<Tabs>
  <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    {' '}

    <CommandTabs
      npm="npm run tauri add nfc"
      yarn="yarn run tauri add nfc"
      pnpm="pnpm tauri add nfc"
      bun="bun tauri add nfc"
      cargo="cargo tauri add nfc"
    />

  </TabItem>
  <TabItem label="Manual">
    <Steps>

      1.  Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

          ```sh frame=none
          cargo add tauri-plugin-nfc --target 'cfg(any(target_os = "android", target_os = "ios"))'
          ```

      2.  Modify `lib.rs` to initialize the plugin:

          ```rust title="src-tauri/src/lib.rs" ins={5-6}
          #[cfg_attr(mobile, tauri::mobile_entry_point)]
          pub fn run() {
              tauri::Builder::default()
                  .setup(|app| {
                      #[cfg(mobile)]
                      app.handle().plugin(tauri_plugin_nfc::init());
                      Ok(())
                  })
                  .run(tauri::generate_context!())
                  .expect("error while running tauri application");
          }
          ```

      3.  Install the JavaScript Guest bindings using your preferred JavaScript package manager:

          <CommandTabs
              npm="npm install @tauri-apps/plugin-nfc"
              yarn="yarn add @tauri-apps/plugin-nfc"
              pnpm="pnpm add @tauri-apps/plugin-nfc"
              deno="deno add npm:@tauri-apps/plugin-nfc"
              bun="bun add @tauri-apps/plugin-nfc"
          />

    </Steps>

  </TabItem>
</Tabs>

## Configuration

The NFC plugin requires native configuration for iOS.

### iOS

To access the NFC APIs on iOS you must adjust the target iOS version, configure a usage description on the Info.plist file and add the NFC capability to your application.

#### Target IOS version

The NFC plugin requires iOS 14+. This is the default for Tauri applications created with Tauri CLI v2.8 and above, but you can edit your Xcode project to configure it.

In the `src-tauri/gen/apple/<project-name>.xcodeproj/project.pbxproj` file, set all `IPHONEOS_DEPLOYMENT_TARGET` properties to `14.0`:

```title="src-tauri/gen/apple/<project-name>.xcodeproj/project.pbxproj"
/* Begin XCBuildConfiguration section */
		<random-id> /* release */ = {
			isa = XCBuildConfiguration;
			buildSettings = {
				...
				IPHONEOS_DEPLOYMENT_TARGET = 14.0;
			};
			name = release;
		};
		<random-id> /* debug */ = {
			isa = XCBuildConfiguration;
			buildSettings = {
        ...
				IPHONEOS_DEPLOYMENT_TARGET = 14.0;
			};
			name = debug;
		};
```

Alternatively you can set the deployment target from Xcode in the `General > Minimum Deployments > iOS` configuration.

#### Info.plist

On iOS the NFC plugin requires the `NFCReaderUsageDescription` information property list value, which should describe why your app needs to scan or write to NFC tags.

In the `src-tauri/Info.ios.plist` file, add the following snippet:

```xml title=src-tauri/Info.ios.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
	<dict>
		<key>NFCReaderUsageDescription</key>
		<string>Read and write various NFC tags</string>
	</dict>
</plist>
```

#### NFC Capability

Additionally iOS requires the NFC capability to be associated with your application.

The capability can be added in Xcode in the project configuration's "Signing & Capabilities" tab by clicking the "+ Capability" button and
selecting the "Near Field Communication Tag Reading" capability (see [Add a capability to a target] for more information)
or by adding the following configuration to the `gen/apple/<app-name>_iOS/<app-name>_iOS.entitlements` file:

```xml title="gen/apple/<app-name>_iOS/<app-name>_iOS.entitlements"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>com.apple.developer.nfc.readersession.formats</key>
	<array>
		<string>TAG</string>
	</array>
</dict>
</plist>
```

## Usage

The NFC plugin is available in both JavaScript and Rust, allowing you to scan and write to NFC tags.

### Checking if NFC is supported

Not every mobile device has the capability to scan NFC tags, so you should check for availability before using the scan and write APIs.

<Tabs syncKey="lang">

<TabItem label="JavaScript">

```javascript
import { isAvailable } from '@tauri-apps/plugin-nfc';

const canScanNfc = await isAvailable();
```

</TabItem>
<TabItem label="Rust">

```rust
tauri::Builder::default()
  .setup(|app| {
    #[cfg(mobile)]
    {
      use tauri_plugin_nfc::NfcExt;

      app.handle().plugin(tauri_plugin_nfc::init());

      let can_scan_nfc = app.nfc().is_available()?;
    }
    Ok(())
  })
```

</TabItem>
</Tabs>

### Scanning NFC tags

The plugin can scan either generic NFC tags or NFC tags with a NDEF (NFC Data Exchange Format) message,
which is a standard format to encapsulate typed data in an NFC tag.

<Tabs syncKey="lang">

<TabItem label="JavaScript">

```javascript
import { scan } from '@tauri-apps/plugin-nfc';

const scanType = {
  type: 'ndef', // or 'tag',
};

const options = {
  keepSessionAlive: false,
  // configure the messages displayed in the "Scan NFC" dialog on iOS
  message: 'Scan a NFC tag',
  successMessage: 'NFC tag successfully scanned',
};

const tag = await scan(scanType, options);
```

</TabItem>

<TabItem label="Rust">

```rust
tauri::Builder::default()
  .setup(|app| {
    #[cfg(mobile)]
    {
      use tauri_plugin_nfc::NfcExt;

      app.handle().plugin(tauri_plugin_nfc::init());

      let tag = app
        .nfc()
        .scan(tauri_plugin_nfc::ScanRequest {
            kind: tauri_plugin_nfc::ScanKind::Ndef {
                mime_type: None,
                uri: None,
                tech_list: None,
            },
            keep_session_alive: false,
        })?
        .tag;
    }
    Ok(())
  })
```

</TabItem>

</Tabs>

:::note
The `keepSessionAlive` option can be used to directly write to the scanned NFC tag later.

If you do not provide that option, the session is recreated on the next `write()` call,
which means the app will try to rescan the tag.
:::

#### Filters

The NFC scanner can also filter tags with a specific URI format, mime type or NFC tag technologies.
In this case, the scan will only detect tags that matches the provided filters.

:::note
Filtering is only available on Android, so you should always check the scanned NFC tag contents.

The mime type is case sensitive and must be provided with lower case letters.
:::

<Tabs syncKey="lang">

<TabItem label="JavaScript">

```javascript
import { scan, TechKind } from '@tauri-apps/plugin-nfc';

const techLists = [
  // capture anything using NfcF
  [TechKind.NfcF],
  // capture all MIFARE Classics with NDEF payloads
  [TechKind.NfcA, TechKind.MifareClassic, TechKind.Ndef],
];

const tag = await scan({
  type: 'ndef', // or 'tag'
  mimeType: 'text/plain',
  uri: {
    scheme: 'https',
    host: 'my.domain.com',
    pathPrefix: '/app',
  },
  techLists,
});
```

</TabItem>

<TabItem label="Rust">

```rust
tauri::Builder::default()
  .setup(|app| {
    #[cfg(mobile)]
    {
      use tauri_plugin_nfc::NfcExt;

      app.handle().plugin(tauri_plugin_nfc::init());

      let tag = app
        .nfc()
        .scan(tauri_plugin_nfc::ScanRequest {
            kind: tauri_plugin_nfc::ScanKind::Ndef {
                mime_type: Some("text/plain".to_string()),
                uri: Some(tauri_plugin_nfc::UriFilter {
                  scheme: Some("https".to_string()),
                  host: Some("my.domain.com".to_string()),
                  path_prefix: Some("/app".to_string()),
                }),
                tech_list: Some(vec![
                  vec![tauri_plugin_nfc::TechKind::Ndef],
                ]),
            },
        })?
        .tag;
    }
    Ok(())
  })
```

</TabItem>

</Tabs>

### Writing to NFC tags

The `write` API can be used to write a payload to a NFC tag.
If there's no scanned tag with `keepSessionAlive: true`, the application will first scan an NFC tag.

<Tabs syncKey="lang">

<TabItem label="JavaScript">

```javascript
import { write, textRecord, uriRecord } from '@tauri-apps/plugin-nfc';

const payload = [uriRecord('https://tauri.app'), textRecord('some payload')];

const options = {
  // the kind is only required if you do not have a scanned tag session alive
  // its format is the same as the argument provided to scan()
  kind: {
    type: 'ndef',
  },
  // configure the messages displayed in the "Scan NFC" dialog on iOS
  message: 'Scan a NFC tag',
  successfulReadMessage: 'NFC tag successfully scanned',
  successMessage: 'NFC tag successfully written',
};

await write(payload, options);
```

</TabItem>

<TabItem label="Rust">
:::caution
The Rust API currently only provides a low level interface for writing NFC payloads.

The API will be enhanced soon.
:::

```rust
tauri::Builder::default()
  .setup(|app| {
    #[cfg(mobile)]
    {
      use tauri_plugin_nfc::NfcExt;

      app.handle().plugin(tauri_plugin_nfc::init());

      app
        .nfc()
        .write(vec![
          tauri_plugin_nfc::NfcRecord {
            format: tauri_plugin_nfc::NFCTypeNameFormat::NfcWellKnown,
            kind: vec![0x55], // URI record
            id: vec![],
            payload: vec![], // insert payload here
          }
        ])?;
    }
    Ok(())
  })
```

</TabItem>
</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4}
{
  "permissions": [
    ...,
    "nfc:default",
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />

[Add a capability to a target]: https://help.apple.com/xcode/mac/current/#/dev88ff319e7
```

---

## 📄 文件: notification.mdx

---

```mdx
---
title: Notifications
description: Send native notifications to the user.
i18nReady: true
plugin: notification
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';
import PluginPermissions from '@components/PluginPermissions.astro';
import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';

<PluginLinks plugin={frontmatter.plugin} />

Send native notifications to your user using the notification plugin.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the notifications plugin to get started.

<Tabs>
    <TabItem label="Automatic">

        Use your project's package manager to add the dependency:

        <CommandTabs npm="npm run tauri add notification"
            yarn="yarn run tauri add notification"
            pnpm="pnpm tauri add notification"
            bun="bun tauri add notification"
        deno="deno task tauri add notification"
            cargo="cargo tauri add notification" />


    </TabItem>
    <TabItem label="Manual">
        <Steps>

        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

           ```sh frame=none
           cargo add tauri-plugin-notification
           ```

        2. Modify `lib.rs` to initialize the plugin:

           ```rust title="src-tauri/src/lib.rs" ins={4}
           #[cfg_attr(mobile, tauri::mobile_entry_point)]
           pub fn run() {
               tauri::Builder::default()
                   .plugin(tauri_plugin_notification::init())
                   .run(tauri::generate_context!())
                   .expect("error while running tauri application");
           }
           ```

        3. If you'd like to use notifications in JavaScript then install the npm package as well:


            <CommandTabs
                npm="npm install @tauri-apps/plugin-notification"
                yarn="yarn add @tauri-apps/plugin-notification"
                pnpm="pnpm add @tauri-apps/plugin-notification"
                deno="bun add npm:@tauri-apps/plugin-notification"
                bun="bun add @tauri-apps/plugin-notification"
            />

        </Steps>
    </TabItem>

</Tabs>

## Usage

Here are a few examples of how to use the notification plugin:

- [Send notification to users](#send-notification)
- [Add an action to a notification](#actions)
- [Add an attachment to a notification](#attachments)
- [Send a notification in a specific channel](#channels)

The notification plugin is available in both JavaScript and Rust.

### Send Notification

Follow these steps to send a notification:

<Steps>
1. Check if permission is granted

2. Request permission if not granted

3. Send the notification

</Steps>

<Tabs syncKey="lang">
<TabItem label="JavaScript">

```javascript
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';
// when using `"withGlobalTauri": true`, you may use
// const { isPermissionGranted, requestPermission, sendNotification, } = window.__TAURI__.notification;

// Do you have permission to send a notification?
let permissionGranted = await isPermissionGranted();

// If not we need to request it
if (!permissionGranted) {
  const permission = await requestPermission();
  permissionGranted = permission === 'granted';
}

// Once permission has been granted we can send the notification
if (permissionGranted) {
  sendNotification({ title: 'Tauri', body: 'Tauri is awesome!' });
}
```

</TabItem>
<TabItem label="Rust">

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .setup(|app| {
        use tauri_plugin_notification::NotificationExt;
        app.notification()
            .builder()
            .title("Tauri")
            .body("Tauri is awesome")
            .show()
            .unwrap();

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

</TabItem>
</Tabs>

### Actions

:::caution[Mobile Only]
The Actions API is only available on mobile platforms.
:::

Actions add interactive buttons and inputs to notifications. Use them to create a responsive experience for your users.

#### Register Action Types

Register action types to define interactive elements:

```javascript
import { registerActionTypes } from '@tauri-apps/plugin-notification';

await registerActionTypes([
  {
    id: 'messages',
    actions: [
      {
        id: 'reply',
        title: 'Reply',
        input: true,
        inputButtonTitle: 'Send',
        inputPlaceholder: 'Type your reply...',
      },
      {
        id: 'mark-read',
        title: 'Mark as Read',
        foreground: false,
      },
    ],
  },
]);
```

#### Action Properties

| Property                 | Description                             |
| ------------------------ | --------------------------------------- |
| `id`                     | Unique identifier for the action        |
| `title`                  | Display text for the action button      |
| `requiresAuthentication` | Requires device authentication          |
| `foreground`             | Brings app to foreground when triggered |
| `destructive`            | Shows action in red on iOS              |
| `input`                  | Enables text input                      |
| `inputButtonTitle`       | Text for input submit button            |
| `inputPlaceholder`       | Placeholder text for input field        |

#### Listen for Actions

Listen to user interactions with notification actions:

```javascript
import { onAction } from '@tauri-apps/plugin-notification';

await onAction((notification) => {
  console.log('Action performed:', notification);
});
```

### Attachments

Attachments add media content to notifications. Support varies by platform.

```javascript
import { sendNotification } from '@tauri-apps/plugin-notification';

sendNotification({
  title: 'New Image',
  body: 'Check out this picture',
  attachments: [
    {
      id: 'image-1',
      url: 'asset:///notification-image.jpg',
    },
  ],
});
```

#### Attachment Properties

| Property | Description                                    |
| -------- | ---------------------------------------------- |
| `id`     | Unique identifier                              |
| `url`    | Content URL using asset:// or file:// protocol |

Note: Test attachments on your target platforms to ensure compatibility.

### Channels

Channels organize notifications into categories with different behaviors. While primarily used on Android, they provide a consistent API across platforms.

#### Create a Channel

```javascript
import {
  createChannel,
  Importance,
  Visibility,
} from '@tauri-apps/plugin-notification';

await createChannel({
  id: 'messages',
  name: 'Messages',
  description: 'Notifications for new messages',
  importance: Importance.High,
  visibility: Visibility.Private,
  lights: true,
  lightColor: '#ff0000',
  vibration: true,
  sound: 'notification_sound',
});
```

#### Channel Properties

| Property      | Description                                    |
| ------------- | ---------------------------------------------- |
| `id`          | Unique identifier                              |
| `name`        | Display name                                   |
| `description` | Purpose description                            |
| `importance`  | Priority level (None, Min, Low, Default, High) |
| `visibility`  | Privacy setting (Secret, Private, Public)      |
| `lights`      | Enable notification LED (Android)              |
| `lightColor`  | LED color (Android)                            |
| `vibration`   | Enable vibrations                              |
| `sound`       | Custom sound filename                          |

#### Managing Channels

List existing channels:

```javascript
import { channels } from '@tauri-apps/plugin-notification';

const existingChannels = await channels();
```

Remove a channel:

```javascript
import { removeChannel } from '@tauri-apps/plugin-notification';

await removeChannel('messages');
```

#### Using Channels

Send a notification using a channel:

```javascript
import { sendNotification } from '@tauri-apps/plugin-notification';

sendNotification({
  title: 'New Message',
  body: 'You have a new message',
  channelId: 'messages',
});
```

Note: Create channels before sending notifications that reference them. Invalid channel IDs prevent notifications from displaying.

## Security Considerations

Aside from normal sanitization procedures of user input there are currently no known security considerations.

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: opener.mdx

---

```mdx
---
title: Opener
description: Open files and URLs in external applications.
plugin: opener
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

This plugin allows you to open files and URLs in a specified, or the default, application. It also supports "revealing" files in the system's file explorer.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the opener plugin to get started.

<Tabs>
	<TabItem label="Automatic" >
		Use your project's package manager to add the dependency:

    	{ ' ' }

    	<CommandTabs
            npm="npm run tauri add opener"
            yarn="yarn run tauri add opener"
            pnpm="pnpm tauri add opener"
            deno="deno task tauri add opener"
            bun="bun tauri add opener"
            cargo="cargo tauri add opener"
    	/>
    </TabItem>
    <TabItem label = "Manual">
    	<Steps>
          1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

              ```sh frame=none
              cargo add tauri-plugin-opener
              ```

          2. Modify `lib.rs` to initialize the plugin:

              ```rust title="src-tauri/src/lib.rs" ins={4}
              #[cfg_attr(mobile, tauri::mobile_entry_point)]
              pub fn run() {
                  tauri::Builder::default()
                      .plugin(tauri_plugin_opener::init())
                      .run(tauri::generate_context!())
                      .expect("error while running tauri application");
              }
              ```

          3. Install the JavaScript Guest bindings using your preferred JavaScript package manager:

              <CommandTabs
                  npm = "npm install @tauri-apps/plugin-opener"
                  yarn = "yarn add @tauri-apps/plugin-opener"
                  pnpm = "pnpm add @tauri-apps/plugin-opener"
                  deno = "deno add npm:@tauri-apps/plugin-opener"
                  bun = "bun add @tauri-apps/plugin-opener"
              />
    	</Steps>
    </TabItem>

</Tabs>

## Usage

The opener plugin is available in both JavaScript and Rust.

<Tabs syncKey="lang">
	<TabItem label="JavaScript" >

```javascript
import { openPath } from '@tauri-apps/plugin-opener';
// when using `"withGlobalTauri": true`, you may use
// const { openPath } = window.__TAURI__.opener;

// opens a file using the default program:
await openPath('/path/to/file');
// opens a file using `vlc` command on Windows:
await openPath('C:/path/to/file', 'vlc');
```

    </TabItem>
    <TabItem label = "Rust" >

Note that `app` is an instance of `App` or [`AppHandle`](https://docs.rs/tauri/2.0.0/tauri/struct.AppHandle.html).

```rust
use tauri_plugin_opener::OpenerExt;

// opens a file using the default program:
app.opener().open_path("/path/to/file", None::<&str>);
// opens a file using `vlc` command on Windows:
app.opener().open_path("C:/path/to/file", Some("vlc"));
```

    </TabItem>

</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

Below are two example scope configurations. Both `path` and `url` use the [glob pattern syntax](https://docs.rs/glob/latest/glob/struct.Pattern.html) to define allowed file paths and URLs.

First, an example on how to add permissions to specific paths for the `openPath()` function:

```json title="src-tauri/capabilities/default.json" ins={6-15}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "opener:allow-open-path",
      "allow": [
        {
          "path": "/path/to/file"
        },
        {
          "path": "$APPDATA/file"
        }
      ]
    }
  ]
}
```

Lastly, an example on how to add permissions for the exact `https://tauri.app` URL and all URLs on a custom protocol (must be known to the OS) for the `openUrl()` function:

```json title="src-tauri/capabilities/default.json" ins={6-15}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "opener:allow-open-url",
      "allow": [
        {
          "url": "https://tauri.app"
        },
        {
          "url": "custom:*"
        }
      ]
    }
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: os-info.mdx

---

```mdx
---
title: OS Information
description: Read information about the operating system.
plugin: os
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Read information about the operating system using the OS Information plugin.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the OS Information plugin to get started.

<Tabs>
  <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    <CommandTabs npm="npm run tauri add os"
          yarn="yarn run tauri add os"
          pnpm="pnpm tauri add os"
          deno="deno task tauri add os"
          bun="bun tauri add os"
          cargo="cargo tauri add os" />

  </TabItem>
  <TabItem label="Manual">
    <Steps>

    1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

        ```sh frame=none
        cargo add tauri-plugin-os
        ```

    2.  Modify `lib.rs` to initialize the plugin:

        ```rust title="src-tauri/src/lib.rs" ins={4}
        #[cfg_attr(mobile, tauri::mobile_entry_point)]
        pub fn run() {
            tauri::Builder::default()
                .plugin(tauri_plugin_os::init())
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        ```

    3.  If you'd like to use in JavaScript then install the npm package as well:

        <CommandTabs
            npm="npm install @tauri-apps/plugin-os"
            yarn="yarn add @tauri-apps/plugin-os"
            pnpm="pnpm add @tauri-apps/plugin-os"
            deno="deno add npm:@tauri-apps/plugin-os"
            bun="bun add @tauri-apps/plugin-os"
        />

    </Steps>

  </TabItem>
</Tabs>

## Usage

With this plugin you can query multiple information from current operational system. See all available functions in the [JavaScript API](/reference/javascript/os/) or [Rust API](https://docs.rs/tauri-plugin-os/) references.

{/* TODO: Link to which language to use, frontend vs. backend guide when it's made */}

#### Example: OS Platform

`platform` returns a string describing the specific operating system in use. The value is set at compile time. Possible values are `linux`, `macos`, `ios`, `freebsd`, `dragonfly`, `netbsd`, `openbsd`, `solaris`, `android`, `windows`.

<Tabs syncKey="lang">
<TabItem label="JavaScript">

```javascript
import { platform } from '@tauri-apps/plugin-os';
// when using `"withGlobalTauri": true`, you may use
// const { platform } = window.__TAURI__.os;

const currentPlatform = platform();
console.log(currentPlatform);
// Prints "windows" to the console
```

</TabItem>
<TabItem label="Rust">

```rust
let platform = tauri_plugin_os::platform();
println!("Platform: {}", platform);
// Prints "windows" to the terminal
```

</TabItem>
</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4}
{
  "permissions": [
    ...,
    "os:default"
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: persisted-scope.mdx

---

```mdx
---
title: Persisted Scope
description: Persist runtime scope changes on the filesystem.
plugin: persisted-scope
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';

<PluginLinks plugin={frontmatter.plugin} showJsLinks={false} />

Save filesystem and asset scopes and restore them when the app is reopened.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the persisted-scope plugin to get started.

<Tabs>
  <TabItem label="Automatic" >

    Use your project's package manager to add the dependency:

    { ' ' }

    <CommandTabs
      npm="npm run tauri add persisted-scope"
      yarn="yarn run tauri add persisted-scope"
      pnpm="pnpm tauri add persisted-scope"
      deno="deno task tauri add persisted-scope"
      bun="bun tauri add persisted-scope"
      cargo="cargo tauri add persisted-scope"
    />

  </TabItem>
  <TabItem label = "Manual">
    <Steps>

    1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

        ```sh frame=none
        cargo add tauri-plugin-persisted-scope
        ```

    2.  Modify `lib.rs` to initialize the plugin:

        ```rust title="src-tauri/src/lib.rs" ins={4}
        #[cfg_attr(mobile, tauri::mobile_entry_point)]
        pub fn run() {
            tauri::Builder::default()
                .plugin(tauri_plugin_persisted_scope::init())
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        ```

    </Steps>

  </TabItem>
</Tabs>

## Usage

After setup the plugin will automatically save and restore filesystem and asset scopes.
```

---

## 📄 文件: positioner.mdx

---

```mdx
---
title: Positioner
description: Move windows to common locations.
plugin: positioner
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Position your windows at well-known locations.

This plugin is a port of [electron-positioner](https://github.com/jenslind/electron-positioner) for Tauri.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the positioner plugin to get started.

:::note
If you only intend on moving the window from Rust code, you only need the dependency in `src-tauri/Cargo.toml`, and can remove the plugin registration from `lib.rs` if you choose to setup automatically.
:::

<Tabs>
	<TabItem label="Automatic" >

    	Use your project's package manager to add the dependency:

    	{ ' ' }

    	<CommandTabs
            npm="npm run tauri add positioner"
            yarn="yarn run tauri add positioner"
            pnpm="pnpm tauri add positioner"
            bun="bun tauri add positioner"
            deno="deno task tauri add positioner"
            cargo="cargo tauri add positioner"
    	/>

    </TabItem>
    <TabItem label = "Manual">
    	<Steps>
    			1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

    				```sh frame=none
    				cargo add tauri-plugin-positioner --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
    				```

    			2. Modify `lib.rs` to initialize the plugin:

    				```rust title="src-tauri/src/lib.rs" ins={4}
    				#[cfg_attr(mobile, tauri::mobile_entry_point)]
    				pub fn run() {
    						tauri::Builder::default()
    								.setup(|app| {
    										#[cfg(desktop)]
    										app.handle().plugin(tauri_plugin_positioner::init());
    										Ok(())
    								})
    								.run(tauri::generate_context!())
    								.expect("error while running tauri application");
    				}
    				```

    			3. Install the JavaScript Guest bindings using your preferred JavaScript package manager:

    				<CommandTabs
    							npm = "npm install @tauri-apps/plugin-positioner"
    							yarn = "yarn add @tauri-apps/plugin-positioner"
    							pnpm = "pnpm add @tauri-apps/plugin-positioner"
    							deno = "deno add npm:@tauri-apps/plugin-positioner"
    							bun = "bun add @tauri-apps/plugin-positioner"
    						/>
    	</Steps>
    </TabItem>

</Tabs>

Additional setup is required to get tray-relative positions to work.

<Steps>
	1. Add `tray-icon` feature to your `Cargo.toml` file:
		```toml title="src-tauri/Cargo.toml" ins={2}
		[dependencies]
		tauri-plugin-positioner = { version = "2.0.0", features = ["tray-icon"] }
		```

    2. Setup `on_tray_event` for positioner plugin:
    	```rust title="src-tauri/src/lib.rs" ins={4-12}
    	pub fn run() {
    		tauri::Builder::default()
    			// This is required to get tray-relative positions to work
    			.setup(|app| {
    					#[cfg(desktop)]
    					{
    						app.handle().plugin(tauri_plugin_positioner::init());
    							tauri::tray::TrayIconBuilder::new()
    								.on_tray_icon_event(|tray_handle, event| {
    									tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);
    								})
    								.build(app)?;
    					}
    				Ok(())
    			})
    			.run(tauri::generate_context!())
    			.expect("error while running tauri application");
    	}
    	```

</Steps>

## Usage

The plugin's APIs are available through the JavaScript guest bindings:

```javascript
import { moveWindow, Position } from '@tauri-apps/plugin-positioner';
// when using `"withGlobalTauri": true`, you may use
// const { moveWindow, Position } = window.__TAURI__.positioner;

moveWindow(Position.TopRight);
```

You can import and use the Window trait extension directly through Rust:

```rust
use tauri_plugin_positioner::{WindowExt, Position};

let mut win = app.get_webview_window("main").unwrap();
let _ = win.as_ref().window().move_window(Position::TopRight);
```

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4}
{
  "permissions": [
    ...,
    "positioner:default",
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: process.mdx

---

```mdx
---
title: Process
description: Access the current process.
plugin: process
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import PluginPermissions from '@components/PluginPermissions.astro';
import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';

<PluginLinks plugin={frontmatter.plugin} />

This plugin provides APIs to access the current process. To spawn child processes, see the [shell](/plugin/shell/) plugin.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the plugin-process to get started.

<Tabs>
    <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    <CommandTabs npm="npm run tauri add process"
    yarn="yarn run tauri add process"
    pnpm="pnpm tauri add process"
    deno="deno task tauri add process"
    bun="bun tauri add process"
    cargo="cargo tauri add process" />

    </TabItem>
    <TabItem label="Manual">
        <Steps>

        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-process
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={4}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .plugin(tauri_plugin_process::init())
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

        3.  If you'd like to utilize the plugin in JavaScript then install the npm package as well:

            <CommandTabs
                npm="npm install @tauri-apps/plugin-process"
                yarn="yarn add @tauri-apps/plugin-process"
                pnpm="pnpm add @tauri-apps/plugin-process"
                deno="deno add npm:@tauri-apps/plugin-process"
                bun="bun add @tauri-apps/plugin-process"
            />

        </Steps>
    </TabItem>

</Tabs>

## Usage

The process plugin is available in both JavaScript and Rust.

<Tabs syncKey="lang">
<TabItem label="JavaScript">

```javascript
import { exit, relaunch } from '@tauri-apps/plugin-process';
// when using `"withGlobalTauri": true`, you may use
// const { exit, relaunch } = window.__TAURI__.process;

// exits the app with the given status code
await exit(0);

// restarts the app
await relaunch();
```

</TabItem>
<TabItem label="Rust">

Note that `app` is an instance of [`AppHandle`](https://docs.rs/tauri/2.0.0/tauri/struct.AppHandle.html).

```rust
// exits the app with the given status code
app.exit(0);

// restarts the app
app.restart();
```

</TabItem>
</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4}
{
  "permissions": [
    ...,
    "process:default",
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: shell.mdx

---

```mdx
---
title: Shell
description: Access the system shell to spawn child processes.
plugin: shell
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Access the system shell. Allows you to spawn child processes.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Opener

If you're looking for documentation for the `shell.open` API, check out the new [Opener plugin](../opener/) instead.

## Setup

Install the shell plugin to get started.

<Tabs>
	<TabItem label="Automatic" >
		Use your project's package manager to add the dependency:

    	{ ' ' }

    	<CommandTabs
            npm="npm run tauri add shell"
            yarn="yarn run tauri add shell"
            pnpm="pnpm tauri add shell"
            deno="deno task tauri add shell"
            bun="bun tauri add shell"
            cargo="cargo tauri add shell"
    	/>
    </TabItem>
    <TabItem label = "Manual">
    	<Steps>
          1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

              ```sh frame=none
              cargo add tauri-plugin-shell
              ```

          2. Modify `lib.rs` to initialize the plugin:

              ```rust title="src-tauri/src/lib.rs" ins={4}
              #[cfg_attr(mobile, tauri::mobile_entry_point)]
              pub fn run() {
                  tauri::Builder::default()
                      .plugin(tauri_plugin_shell::init())
                      .run(tauri::generate_context!())
                      .expect("error while running tauri application");
              }
              ```

          3. Install the JavaScript Guest bindings using your preferred JavaScript package manager:

              <CommandTabs
                  npm = "npm install @tauri-apps/plugin-shell"
                  yarn = "yarn add @tauri-apps/plugin-shell"
                  pnpm = "pnpm add @tauri-apps/plugin-shell"
                  deno = "deno add npm:@tauri-apps/plugin-shell"
                  bun = "bun add @tauri-apps/plugin-shell"
              />
    	</Steps>
    </TabItem>

</Tabs>

## Usage

The shell plugin is available in both JavaScript and Rust.

<Tabs syncKey="lang">
	<TabItem label="JavaScript" >

```javascript
import { Command } from '@tauri-apps/plugin-shell';
// when using `"withGlobalTauri": true`, you may use
// const { Command } = window.__TAURI__.shell;

let result = await Command.create('exec-sh', [
  '-c',
  "echo 'Hello World!'",
]).execute();
console.log(result);
```

    </TabItem>
    <TabItem label = "Rust" >

```rust
use tauri_plugin_shell::ShellExt;

let shell = app_handle.shell();
let output = tauri::async_runtime::block_on(async move {
		shell
				.command("echo")
				.args(["Hello from Rust!"])
				.output()
				.await
				.unwrap()
});
if output.status.success() {
		println!("Result: {:?}", String::from_utf8(output.stdout));
} else {
		println!("Exit with code: {}", output.status.code().unwrap());
}
```

    </TabItem>

</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={6-23}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    {
      "identifier": "shell:allow-execute",
      "allow": [
        {
          "name": "exec-sh",
          "cmd": "sh",
          "args": [
            "-c",
            {
              "validator": "\\S+"
            }
          ],
          "sidecar": false
        }
      ]
    }
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: single-instance.mdx

---

```mdx
---
title: Single Instance
description: Ensure that a single instance of your Tauri app is running at a time.
plugin: single-instance
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';

<PluginLinks plugin={frontmatter.plugin} showJsLinks={false} />

Ensure that a single instance of your tauri app is running at a time using the Single Instance Plugin.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the Single Instance plugin to get started.

<Tabs>
  <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    {' '}

    <CommandTabs
      npm="npm run tauri add single-instance"
      yarn="yarn run tauri add single-instance"
      pnpm="pnpm tauri add single-instance"
      deno="deno task tauri add single-instance"
      bun="bun tauri add single-instance"
      cargo="cargo tauri add single-instance"
    />

  </TabItem>
    <TabItem label="Manual">
      <Steps>

        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-single-instance --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="lib.rs" ins={5-6}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .setup(|app| {
                        #[cfg(desktop)]
                        app.handle().plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}));
                        Ok(())
                    })
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

      </Steps>
    </TabItem>

</Tabs>

:::info

The Single Instance plugin must be the first one to be registered to work well. This assures that it runs before other plugins can interfere.

:::

## Usage

The plugin is already installed and initialized, and it should be functioning correctly right away. Nevertheless, we can also enhance its functionality with the `init()` method.

The plugin `init()` method takes a closure that is invoked when a new app instance was started, but closed by the plugin.
The closure has three arguments:

  <Steps>

1. **`app`:** The [AppHandle](https://docs.rs/tauri/2.0.0/tauri/struct.AppHandle.html) of the application.
2. **`args`:** The list of arguments, that was passed by the user to initiate this new instance.
3. **`cwd`:** The Current Working Directory denotes the directory from which the new application instance was launched.

  </Steps>

So, the closure should look like below

```rust
.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
  // Write your code here...
}))
```

### Focusing on New Instance

By default, when you initiate a new instance while the application is already running, no action is taken. To focus the window of the running instance when user tries to open a new instance, alter the callback closure as follows:

```rust title="src-tauri/src/lib.rs" {1} {5-12}
use tauri::{AppHandle, Manager};

pub fn run() {
    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = app.get_webview_window("main")
                       .expect("no main window")
                       .set_focus();
        }));
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Usage in Snap and Flatpak

On Linux the Single Instance plugin uses DBus to ensure that there will be only one instance running. It does so by publishing a service to DBus when the first instance starts running.
Then, the following instances will try to publish the same service and, if it is already published, they will send a request to the service to notify the first instance, and exit right away.

Despite this working pretty well when your app is bundled as a deb or rpm package or an AppImage, it won't work as intended for snap or flatpak packages by default because these packages run in a constrained sandboxed environment, where most of the communication to DBus services will be blocked if not explicitly declared on the packaging manifest.

Here's a guide that shows how to declare the needed permissions to enable the Single Instance for snap and flatpak packages:

### Getting your app ID

The Single Instance plugin will publish a service named `org.{id}.SingleInstance`.

`{id}` will be the `identifier` from your `tauri.conf.json` file, but with with dots (`.`) and dashes (`-`) replaced by underline (`_`).

For example, if your identifier is `net.mydomain.MyApp`:

- `net_mydomain_MyApp` will be your app `{id}`
- `org.net_mydomain_MyApp.SingleInstance` will be your app SingleInstance service name

You will need the service name to authorize your app to use the DBus service on snap and flatpak manifests, as seen below.

### Snap

In your snapcraft.yml file, declare a plug and a slot for the single instance service, and use both on your app declaration:

```yaml title="snapcraft.yml"
# ...
slots:
  single-instance:
    interface: dbus
    bus: session
    name: org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID

plugs:
  single-instance-plug:
    interface: dbus
    bus: session
    name: org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID

# .....
apps:
  my-app:
    # ...
    plugs:
      # ....
      - single-instance-plug
    slots:
      # ...
      - single-instance

    # ....
```

This will allow your app to send and receive requests from/to the DBus service as expected by the Single Instance plugin.

### Flatpak

In your flatpak manifest file (your.app.id.yml or your.app.id.json), declare a `--talk-name` and a `--own-name` finish args with the service name:

```yaml title="net.mydomain.MyApp.yml"
# ...
finish-args:
  - --socket=wayland
  - --socket=fallback-x11
  - --device=dri
  - --share=ipc
  # ....
  - --talk-name=org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID
  - --own-name=org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID
# ...
```

This will allow your app to send and receive requests from/to the DBus service as expected by the Single Instance plugin.
```

---

## 📄 文件: sql.mdx

---

```mdx
---
title: SQL
description: Tauri Plugin providing an interface for the frontend to communicate with SQL databases through sqlx.
plugin: sql
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import PluginPermissions from '@components/PluginPermissions.astro';
import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';

<PluginLinks plugin={frontmatter.plugin} />

Plugin providing an interface for the frontend to communicate with SQL databases through [sqlx](https://github.com/launchbadge/sqlx). It supports the SQLite, MySQL and PostgreSQL drivers, enabled by a Cargo feature.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the SQL plugin to get started.

<Tabs>
    <TabItem label="Automatic">

        Use your project's package manager to add the dependency:

        <CommandTabs npm="npm run tauri add sql"
        yarn="yarn run tauri add sql"
        pnpm="pnpm tauri add sql"
        bun="bun tauri add sql"
        deno="deno task tauri add sql"
        cargo="cargo tauri add sql" />

    </TabItem>
    <TabItem label="Manual">

        <Steps>

        1.  Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-sql
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={4}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
    		    pub fn run() {
                tauri::Builder::default()
                    .plugin(tauri_plugin_sql::Builder::default().build())
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

        3.	Install the JavaScript Guest bindings using your preferred JavaScript package manager:

            <CommandTabs
                npm="npm install @tauri-apps/plugin-sql"
                yarn="yarn add @tauri-apps/plugin-sql"
                pnpm="pnpm add @tauri-apps/plugin-sql"
                deno="deno add npm:@tauri-apps/plugin-sql"
                bun="bun add @tauri-apps/plugin-sql"
            />

        </Steps>

</TabItem>
</Tabs>

After installing the plugin, you must select the supported database engine.
The available engines are Sqlite, MySQL and PostgreSQL.
Run the following command in the `src-tauri` folder to enable your preferred engine:

<Tabs syncKey='SQLvariant'>
  <TabItem label="SQLite">

    ```sh frame=none
    cargo add tauri-plugin-sql --features sqlite
    ```

  </TabItem>
  <TabItem label="MySQL">

    ```sh frame=none
    cargo add tauri-plugin-sql --features mysql
    ```

  </TabItem>
  <TabItem label="PostgreSQL">
  
    ```sh frame=none
    cargo add tauri-plugin-sql --features postgres
    ```
  
  </TabItem>
</Tabs>

## Usage

All the plugin's APIs are available through the JavaScript guest bindings:

<Tabs syncKey='SQLvariant'>
  <TabItem label="SQLite">

The path is relative to [`tauri::api::path::BaseDirectory::AppConfig`](https://docs.rs/tauri/2.0.0/tauri/path/enum.BaseDirectory.html#variant.AppConfig).

```javascript
import Database from '@tauri-apps/plugin-sql';
// when using `"withGlobalTauri": true`, you may use
// const Database = window.__TAURI__.sql;

const db = await Database.load('sqlite:test.db');
await db.execute('INSERT INTO ...');
```

  </TabItem>
  <TabItem label="MySQL">

```javascript
import Database from '@tauri-apps/plugin-sql';
// when using `"withGlobalTauri": true`, you may use
// const Database = window.__TAURI__.sql;

const db = await Database.load('mysql://user:password@host/test');
await db.execute('INSERT INTO ...');
```

  </TabItem>
  <TabItem label="PostgreSQL">

```javascript
import Database from '@tauri-apps/plugin-sql';
// when using `"withGlobalTauri": true`, you may use
// const Database = window.__TAURI__.sql;

const db = await Database.load('postgres://user:password@host/test');
await db.execute('INSERT INTO ...');
```

  </TabItem>
</Tabs>

## Syntax

We use [sqlx](https://docs.rs/sqlx/latest/sqlx/) as the underlying library and adopt their query syntax.

<Tabs syncKey='SQLvariant'>
  <TabItem label="SQLite">
Use the "$#" syntax when substituting query data
```javascript
const result = await db.execute(
  "INSERT into todos (id, title, status) VALUES ($1, $2, $3)",
  [todos.id, todos.title, todos.status],
);

const result = await db.execute(
"UPDATE todos SET title = $1, status = $2 WHERE id = $3",
[todos.title, todos.status, todos.id],
);

````
  </TabItem>
  <TabItem label="MySQL">
Use "?" when substituting query data
```javascript
const result = await db.execute(
  "INSERT into todos (id, title, status) VALUES (?, ?, ?)",
  [todos.id, todos.title, todos.status],
);

const result = await db.execute(
  "UPDATE todos SET title = ?, status = ? WHERE id = ?",
  [todos.title, todos.status, todos.id],
);
````

  </TabItem>
  <TabItem label="PostgreSQL">
Use the "$#" syntax when substituting query data
```javascript
const result = await db.execute(
  "INSERT into todos (id, title, status) VALUES ($1, $2, $3)",
  [todos.id, todos.title, todos.status],
);

const result = await db.execute(
"UPDATE todos SET title = $1, status = $2 WHERE id = $3",
[todos.title, todos.status, todos.id],
);

````
  </TabItem>
</Tabs>

## Migrations

This plugin supports database migrations, allowing you to manage database schema evolution over time.

### Defining Migrations

Migrations are defined in Rust using the [`Migration`](https://docs.rs/tauri-plugin-sql/latest/tauri_plugin_sql/struct.Migration.html) struct. Each migration should include a unique version number, a description, the SQL to be executed, and the type of migration (Up or Down).

Example of a migration:

```rust
use tauri_plugin_sql::{Migration, MigrationKind};

let migration = Migration {
    version: 1,
    description: "create_initial_tables",
    sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
    kind: MigrationKind::Up,
};
````

Or if you want to use SQL from a file, you can include it by using `include_str!`:

```rust
use tauri_plugin_sql::{Migration, MigrationKind};

let migration = Migration {
    version: 1,
    description: "create_initial_tables",
    sql: include_str!("../drizzle/0000_graceful_boomer.sql"),
    kind: MigrationKind::Up,
};
```

### Adding Migrations to the Plugin Builder

Migrations are registered with the [`Builder`](https://docs.rs/tauri-plugin-sql/latest/tauri_plugin_sql/struct.Builder.html) struct provided by the plugin. Use the `add_migrations` method to add your migrations to the plugin for a specific database connection.

Example of adding migrations:

```rust title="src-tauri/src/main.rs" {1} {6-11} {17}
use tauri_plugin_sql::{Builder, Migration, MigrationKind};

fn main() {
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:mydatabase.db", migrations)
                .build(),
        )
        ...
}
```

### Applying Migrations

To apply the migrations when the plugin is initialized, add the connection string to the `tauri.conf.json` file:

```json title="src-tauri/tauri.conf.json" {3-5}
{
  "plugins": {
    "sql": {
      "preload": ["sqlite:mydatabase.db"]
    }
  }
}
```

Alternatively, the client side `load()` also runs the migrations for a given connection string:

```ts
import Database from '@tauri-apps/plugin-sql';
const db = await Database.load('sqlite:mydatabase.db');
```

Ensure that the migrations are defined in the correct order and are safe to run multiple times.

### Migration Management

- **Version Control**: Each migration must have a unique version number. This is crucial for ensuring the migrations are applied in the correct order.
- **Idempotency**: Write migrations in a way that they can be safely re-run without causing errors or unintended consequences.
- **Testing**: Thoroughly test migrations to ensure they work as expected and do not compromise the integrity of your database.

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4-5}
{
  "permissions": [
    ...,
    "sql:default",
    "sql:allow-execute",
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: store.mdx

---

```mdx
---
title: Store
description: Persistent key value storage.
plugin: store
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

This plugin provides a persistent key-value store. This is one of many options to handle state in your application. See the [state management overview](/develop/state-management/) for more information on additional options.

This store will allow you to persist state to a file which can be saved and loaded on demand including between app restarts. Note that this process is asynchronous which will require handling it within your code. It can be used both in the webview or within Rust.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the store plugin to get started.

<Tabs>
    <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    <CommandTabs npm="npm run tauri add store"
    yarn="yarn run tauri add store"
    pnpm="pnpm tauri add store"
    deno="deno task tauri add store"
    bun="bun tauri add store"
    cargo="cargo tauri add store" />

    </TabItem>
    <TabItem label="Manual">
        <Steps>
        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-store
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="src-tauri/src/lib.rs" ins={4}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .plugin(tauri_plugin_store::Builder::new().build())
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

        3.  Install the JavaScript Guest bindings using your preferred JavaScript package manager:

            <CommandTabs
            npm = "npm install @tauri-apps/plugin-store"
            yarn = "yarn add @tauri-apps/plugin-store"
            pnpm = "pnpm add @tauri-apps/plugin-store"
            deno = "deno add npm:@tauri-apps/plugin-store"
            bun = "bun add @tauri-apps/plugin-store"
            />
        </Steps>
    </TabItem>

</Tabs>

## Usage

<Tabs syncKey="lang">
<TabItem label="JavaScript">

```typescript
import { load } from '@tauri-apps/plugin-store';
// when using `"withGlobalTauri": true`, you may use
// const { load } = window.__TAURI__.store;

// Create a new store or load the existing one,
// note that the options will be ignored if a `Store` with that path has already been created
const store = await load('store.json', { autoSave: false });

// Set a value.
await store.set('some-key', { value: 5 });

// Get a value.
const val = await store.get<{ value: number }>('some-key');
console.log(val); // { value: 5 }

// You can manually save the store after making changes.
// Otherwise, it will save upon graceful exit
// And if you set `autoSave` to a number or left empty,
// it will save the changes to disk after a debounce delay, 100ms by default.
await store.save();
```

</TabItem>
<TabItem label="Rust">

```rust title="src-tauri/src/lib.rs"
use tauri::Wry;
use tauri_plugin_store::StoreExt;
use serde_json::json;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // Create a new store or load the existing one
            // this also put the store in the app's resource table
            // so your following calls `store` calls (from both rust and js)
            // will reuse the same store
            let store = app.store("store.json")?;

            // Note that values must be serde_json::Value instances,
            // otherwise, they will not be compatible with the JavaScript bindings.
            store.set("some-key", json!({ "value": 5 }));

            // Get a value from the store.
            let value = store.get("some-key").expect("Failed to get value from store");
            println!("{}", value); // {"value":5}

            // Remove the store from the resource table
            store.close_resource();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

</TabItem>
</Tabs>

### LazyStore

There's also a high level JavaScript API `LazyStore` which only loads the store on first access

```typescript
import { LazyStore } from '@tauri-apps/plugin-store';

const store = new LazyStore('settings.json');
```

## Migrating from v1 and v2 beta/rc

<Tabs syncKey="lang">
<TabItem label="JavaScript">

```diff
- import { Store } from '@tauri-apps/plugin-store';
+ import { LazyStore } from '@tauri-apps/plugin-store';
```

</TabItem>
<TabItem label="Rust">

```diff
- with_store(app.handle().clone(), stores, path, |store| {
-     store.insert("some-key".to_string(), json!({ "value": 5 }))?;
-     Ok(())
- });
+ let store = app.store(path)?;
+ store.set("some-key".to_string(), json!({ "value": 5 }));
```

</TabItem>
</Tabs>

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4}
{
  "permissions": [
    ...,
    "store:default",
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: stronghold.mdx

---

```mdx
---
title: Stronghold
description: Encrypted, secure database.
plugin: stronghold
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Store secrets and keys using the [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs) secret management engine.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the stronghold plugin to get started.

<Tabs>
	<TabItem label="Automatic" >
		Use your project's package manager to add the dependency:

    	{ ' ' }

    	<CommandTabs
            npm="npm run tauri add stronghold"
            yarn="yarn run tauri add stronghold"
            pnpm="pnpm tauri add stronghold"
            bun="bun tauri add stronghold"
            deno="deno task tauri add stronghold"
            cargo="cargo tauri add stronghold"
    	/>
    </TabItem>

    <TabItem label = "Manual">
    	<Steps>

    			1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

    				```sh frame=none
    				cargo add tauri-plugin-stronghold
    				```

    			2. Modify `lib.rs` to initialize the plugin:

    				```rust title="src-tauri/src/lib.rs" ins={4}
    				#[cfg_attr(mobile, tauri::mobile_entry_point)]
    				pub fn run() {
    						tauri::Builder::default()
    								.plugin(tauri_plugin_stronghold::Builder::new(|password| {}).build())
    								.run(tauri::generate_context!())
    								.expect("error while running tauri application");
    				}
    				```

    			3. Install the JavaScript Guest bindings using your preferred JavaScript package manager:

    				<CommandTabs
    								npm = "npm install @tauri-apps/plugin-stronghold"
    								yarn = "yarn add @tauri-apps/plugin-stronghold"
    								pnpm = "pnpm add @tauri-apps/plugin-stronghold"
    								deno = "deno add npm:@tauri-apps/plugin-stronghold"
    								bun = "bun add @tauri-apps/plugin-stronghold"
    				/>

    	</Steps>
    </TabItem>

</Tabs>

## Usage

The plugin must be initialized with a password hash function, which takes the password string and must return a 32 bytes hash derived from it.

### Initialize with argon2 password hash function

The Stronghold plugin offers a default hash function using the [argon2] algorithm.

```rust title="src-tauri/src/lib.rs"
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Initialize with custom password hash function

Alternatively you can provide your own hash algorithm by using the `tauri_plugin_stronghold::Builder::new` constructor.

:::note
The password hash must contain exactly 32 bytes. This is a Stronghold requirement.
:::

```rust title="src-tauri/src/lib.rs"
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_stronghold::Builder::new(|password| {
                // Hash the password here with e.g. argon2, blake2b or any other secure algorithm
                // Here is an example implementation using the `rust-argon2` crate for hashing the password
                use argon2::{hash_raw, Config, Variant, Version};

                let config = Config {
                    lanes: 4,
                    mem_cost: 10_000,
                    time_cost: 10,
                    variant: Variant::Argon2id,
                    version: Version::Version13,
                    ..Default::default()
                };
                let salt = "your-salt".as_bytes();
                let key = hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

                key.to_vec()
            })
            .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Usage from JavaScript

The stronghold plugin is available in JavaScript.

```javascript
import { Client, Stronghold } from '@tauri-apps/plugin-stronghold';
// when using `"withGlobalTauri": true`, you may use
// const { Client, Stronghold } = window.__TAURI__.stronghold;
import { appDataDir } from '@tauri-apps/api/path';
// when using `"withGlobalTauri": true`, you may use
// const { appDataDir } = window.__TAURI__.path;

const initStronghold = async () => {
	const vaultPath = `${await appDataDir()}/vault.hold`;
	const vaultPassword = 'vault password';
	const stronghold = await Stronghold.load(vaultPath, vaultPassword);

	let client: Client;
	const clientName = 'name your client';
	try {
		client = await stronghold.loadClient(clientName);
	} catch {
		client = await stronghold.createClient(clientName);
	}

	return {
		stronghold,
		client,
	};
};

// Insert a record to the store
async function insertRecord(store: any, key: string, value: string) {
	const data = Array.from(new TextEncoder().encode(value));
	await store.insert(key, data);
}

// Read a record from store
async function getRecord(store: any, key: string): Promise<string> {
	const data = await store.get(key);
	return new TextDecoder().decode(new Uint8Array(data));
}

const { stronghold, client } = await initStronghold();

const store = client.getStore();
const key = 'my_key';

// Insert a record to the store
insertRecord(store, key, 'secret value');

// Read a record from store
const value = await getRecord(store, key);
console.log(value); // 'secret value'

// Save your updates
await stronghold.save();

// Remove a record from store
await store.remove(key);
```

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4}
{
	...,
	"permissions": [
		"stronghold:default",
	]
}
```

<PluginPermissions plugin={frontmatter.plugin} />

[argon2]: https://docs.rs/rust-argon2/latest/argon2/
```

---

## 📄 文件: updater.mdx

---

```mdx
---
title: Updater
description: In-app updates for Tauri applications.
plugin: updater
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import PluginPermissions from '@components/PluginPermissions.astro';
import CommandTabs from '@components/CommandTabs.astro';
import { TabItem, Steps, Tabs } from '@astrojs/starlight/components';

<PluginLinks plugin={frontmatter.plugin} />

Automatically update your Tauri app with an update server or a static JSON.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the Tauri updater plugin to get started.

<Tabs>
  <TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    <CommandTabs
      npm="npm run tauri add updater"
      yarn="yarn run tauri add updater"
      pnpm="pnpm tauri add updater"
      deno="deno task tauri add updater"
      bun="bun tauri add updater"
      cargo="cargo tauri add updater"
    />

  </TabItem>
    <TabItem label="Manual">
      <Steps>

        1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

            ```sh frame=none
            cargo add tauri-plugin-updater --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
            ```

        2.  Modify `lib.rs` to initialize the plugin:

            ```rust title="lib.rs" ins={5-6}
            #[cfg_attr(mobile, tauri::mobile_entry_point)]
            pub fn run() {
                tauri::Builder::default()
                    .setup(|app| {
                        #[cfg(desktop)]
                        app.handle().plugin(tauri_plugin_updater::Builder::new().build());
                        Ok(())
                    })
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
            }
            ```

        3.  You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

            <CommandTabs
                npm="npm install @tauri-apps/plugin-updater"
                yarn="yarn add @tauri-apps/plugin-updater"
                pnpm="pnpm add @tauri-apps/plugin-updater"
                deno="deno add npm:@tauri-apps/plugin-updater"
                bun="bun add @tauri-apps/plugin-updater"
            />

      </Steps>
    </TabItem>

</Tabs>

## Signing updates

Tauri's updater needs a signature to verify that the update is from a trusted source. This cannot be disabled.

To sign your updates you need two keys:

1. The public key, which will be set in the `tauri.conf.json` to validate the artifacts before the installation. This public key can be uploaded and shared safely as long as your private key is secure.
2. The private key, which is used to sign your installer files. You should NEVER share this key with anyone. Also, if you lose this key you will NOT be able to publish new updates to the users that have the app already installed. It is important to store this key in a safe place!

To generate the keys the Tauri CLI provides the `signer generate` command. You can run this to create the keys in the home folder:

<Tabs>
  <CommandTabs
    npm="npm run tauri signer generate -- -w ~/.tauri/myapp.key"
    yarn="yarn tauri signer generate -w ~/.tauri/myapp.key"
    pnpm="pnpm tauri signer generate -w ~/.tauri/myapp.key"
    deno="deno task tauri signer generate -w ~/.tauri/myapp.key"
    bun="bunx tauri signer generate -w ~/.tauri/myapp.key"
    cargo="cargo tauri signer generate -w ~/.tauri/myapp.key"
  />
</Tabs>

### Building

While building your update artifacts, you need to have the private key you generated above in your environment variables. `.env` files do _not_ work!

<Tabs>
  <TabItem label="Mac/Linux">
  ```sh frame=none
  export TAURI_SIGNING_PRIVATE_KEY="Path or content of your private key"
  # optionally also add a password
  export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""
  ```
  </TabItem>
  <TabItem label="Windows">
  Run this in `PowerShell`:
  ```ps frame=none
  $env:TAURI_SIGNING_PRIVATE_KEY="Path or content of your private key"
  <# optionally also add a password #>
  $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""
  ```
  </TabItem>
</Tabs>

After that, you can run Tauri build as usual and Tauri will generate the update bundles and their signatures.
The generated files depend on the [`createUpdaterArtifacts`] configuration value configured below.

<Tabs>
  <TabItem label="v2">

```json
{
  "bundle": {
    "createUpdaterArtifacts": true
  }
}
```

On Linux, Tauri will create the normal AppImage inside the `target/release/bundle/appimage/` folder:

- `myapp.AppImage` - The standard app bundle. It will be re-used by the updater.
- `myapp.AppImage.sig` - The signature of the updater bundle.

On macOS, Tauri will create a .tar.gz archive from the application bundle inside the target/release/bundle/macos/ folder:

- `myapp.app` - The standard app bundle.
- `myapp.app.tar.gz` - The updater bundle.
- `myapp.app.tar.gz.sig` - The signature of the update bundle.

On Windows, Tauri will create the normal MSI and NSIS installers inside the target/release/bundle/msi/ and target/release/bundle/nsis folders:

- `myapp-setup.exe` - The standard app bundle. It will be re-used by the updater.
- `myapp-setup.exe.sig` - The signature of the update bundle.
- `myapp.msi` - The standard app bundle. It will be re-used by the updater.
- `myapp.msi.sig` - The signature of the update bundle.

{''}

</TabItem>
<TabItem label="v1 compatible">

```json
{
  "bundle": {
    "createUpdaterArtifacts": "v1Compatible"
  }
}
```

On Linux, Tauri will create a .tar.gz archive from the AppImage inside the `target/release/bundle/appimage/` folder:

- `myapp.AppImage` - The standard app bundle.
- `myapp.AppImage.tar.gz` - The updater bundle.
- `myapp.AppImage.tar.gz.sig` - The signature of the update bundle.

On macOS, Tauri will create a .tar.gz archive from the application bundle inside the target/release/bundle/macos/ folder:

- `myapp.app` - The standard app bundle.
- `myapp.app.tar.gz` - The updater bundle.
- `myapp.app.tar.gz.sig` - The signature of the update bundle.

On Windows, Tauri will create .zip archives from the MSI and NSIS installers inside the target/release/bundle/msi/ and target/release/bundle/nsis folders:

- `myapp-setup.exe` - The standard app bundle.
- `myapp-setup.nsis.zip` - The updater bundle.
- `myapp-setup.nsis.zip.sig` - The signature of the update bundle.
- `myapp.msi` - The standard app bundle.
- `myapp.msi.zip` - The updater bundle.
- `myapp.msi.zip.sig` - The signature of the update bundle.

{''}

  </TabItem>
</Tabs>

## Tauri Configuration

Set up the `tauri.conf.json` in this format for the updater to start working.

| Keys                                 | Description                                                                                                                                                                                                                                                                                    |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `createUpdaterArtifacts`             | Setting this to `true` tells Tauri's app bundler to create updater artifacts. If you're migrating your app from an older Tauri version, set it to `"v1Compatible"` instead. **This setting will be removed in v3** so make sure to change it to `true` once all your users are migrated to v2. |
| `pubkey`                             | This has to be the public key generated from the Tauri CLI in the step above. It **cannot** be a file path!                                                                                                                                                                                    |
| `endpoints`                          | This must be an array of endpoint URLs as strings. TLS is enforced in production mode. Tauri will only continue to the next url if a non-2XX status code is returned!                                                                                                                          |
| `dangerousInsecureTransportProtocol` | Setting this to `true` allows the updater to accept non-HTTPS endpoints. Use this configuration with caution!                                                                                                                                                                                  |

Each updater URL can contain the following dynamic variables, allowing you to determine server-side if an update is available.

- `{{current_version}}`: The version of the app that is requesting the update.
- `{{target}}`: The operating system name (one of `linux`, `windows` or `darwin`).
- `{{arch}}`: The architecture of the machine (one of `x86_64`, `i686`, `aarch64` or `armv7`).

```json title=tauri.conf.json
{
  "bundle": {
    "createUpdaterArtifacts": true
  },
  "plugins": {
    "updater": {
      "pubkey": "CONTENT FROM PUBLICKEY.PEM",
      "endpoints": [
        "https://releases.myapp.com/{{target}}/{{arch}}/{{current_version}}",
        // or a static github json file
        "https://github.com/user/repo/releases/latest/download/latest.json"
      ]
    }
  }
}
```

:::tip
Custom variables are not supported, but you can define a [custom `{{target}}`](#custom-target).
:::

### `installMode` on Windows

On Windows there is an additional optional `"installMode"` config to change how the update is installed.

```json title=tauri.conf.json
{
  "plugins": {
    "updater": {
      "windows": {
        "installMode": "passive"
      }
    }
  }
}
```

- `"passive"`: There will be a small window with a progress bar. The update will be installed without requiring any user interaction. Generally recommended and the default mode.
- `"basicUi"`: There will be a basic user interface shown which requires user interaction to finish the installation.
- `"quiet"`: There will be no progress feedback to the user. With this mode the installer cannot request admin privileges by itself so it only works in user-wide installations or when your app itself already runs with admin privileges. Generally not recommended.

## Server Support

The updater plugin can be used in two ways. Either with a dynamic update server or a static JSON file (to use on services like S3 or GitHub gists).

### Static JSON File

When using static, you just need to return a JSON containing the required information.

| Keys        | Description                                                                                                                                                     |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `version`   | Must be a valid [SemVer](https://semver.org/), with or without a leading `v`, meaning that both `1.0.0` and `v1.0.0` are valid.                                 |
| `notes`     | Notes about the update.                                                                                                                                         |
| `pub_date`  | The date must be formatted according to [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) if present.                                                   |
| `platforms` | Each platform key is in the `OS-ARCH` format, where `OS` is one of `linux`, `darwin` or `windows`, and `ARCH` is one of `x86_64`, `aarch64`, `i686` or `armv7`. |
| `signature` | The content of the generated `.sig` file, which may change with each build. A path or URL does not work!                                                        |

:::info
When using [custom targets](#custom-target) the provided target string is matched against the `platforms` key
instead of the default `OS-ARCH` value.
:::

The required keys are `"version"`, `"platforms.[target].url"` and `"platforms.[target].signature"`; the others are optional.

```json
{
  "version": "",
  "notes": "",
  "pub_date": "",
  "platforms": {
    "linux-x86_64": {
      "signature": "",
      "url": ""
    },
    "windows-x86_64": {
      "signature": "",
      "url": ""
    },
    "darwin-x86_64": {
      "signature": "",
      "url": ""
    }
  }
}
```

Note that Tauri will validate the whole file before checking the version field, so make sure all existing platform configurations are valid and complete.

:::tip
[Tauri Action](https://github.com/tauri-apps/tauri-action) generates a static JSON file for you to use on CDNs such as GitHub Releases.
:::

### Dynamic Update Server

When using a dynamic update server, Tauri will follow the server's instructions. To disable the internal version check you can overwrite the [plugin's version comparison](https://docs.rs/tauri-plugin-updater/latest/tauri_plugin_updater/struct.UpdaterBuilder.html#method.version_comparator), this will install the version sent by the server (useful if you need to roll back your app).

Your server can use variables defined in the `endpoint` URL above to determine if an update is required. If you need more data, you can include additional [request headers in Rust](https://docs.rs/tauri-plugin-updater/latest/tauri_plugin_updater/struct.UpdaterBuilder.html#method.header) to your liking.

Your server should respond with a status code of [`204 No Content`](https://datatracker.ietf.org/doc/html/rfc2616#section-10.2.5) if there is no update available.

If an update is required, your server should respond with a status code of [`200 OK`](http://tools.ietf.org/html/rfc2616#section-10.2.1) and a JSON response in this format:

| Keys        | Description                                                                                                                          |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `version`   | This Must be a valid [SemVer](https://semver.org/), with or without a leading `v`, meaning that both `1.0.0` and `v1.0.0` are valid. |
| `notes`     | Notes about the update.                                                                                                              |
| `pub_date`  | The date must be formatted according to [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) if present.                        |
| `url`       | This Must be a valid URL to the update bundle.                                                                                       |
| `signature` | The content of the generated `.sig` file, which may change with each build. A path or URL does not work!                             |

The required keys are `"url"`, `"version"` and `"signature"`; the others are optional.

```json
{
  "version": "",
  "pub_date": "",
  "url": "",
  "signature": "",
  "notes": ""
}
```

:::tip
CrabNebula, an official Tauri partner, offers a dynamic update server. For more information, see the [Distributing with CrabNebula Cloud] documentation.
:::

## Checking for Updates

The default API for checking updates and installing them leverages the configured endpoints
and can be accessed by both JavaScript and Rust code.

<Tabs syncKey="lang">
  <TabItem label="JavaScript">

```js
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

const update = await check();
if (update) {
  console.log(
    `found update ${update.version} from ${update.date} with notes ${update.body}`
  );
  let downloaded = 0;
  let contentLength = 0;
  // alternatively we could also call update.download() and update.install() separately
  await update.downloadAndInstall((event) => {
    switch (event.event) {
      case 'Started':
        contentLength = event.data.contentLength;
        console.log(`started downloading ${event.data.contentLength} bytes`);
        break;
      case 'Progress':
        downloaded += event.data.chunkLength;
        console.log(`downloaded ${downloaded} from ${contentLength}`);
        break;
      case 'Finished':
        console.log('download finished');
        break;
    }
  });

  console.log('update installed');
  await relaunch();
}
```

For more information see the [JavaScript API documentation].

</TabItem>

<TabItem label="Rust">

```rust title="src-tauri/src/lib.rs"
use tauri_plugin_updater::UpdaterExt;

pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        update(handle).await.unwrap();
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .unwrap();
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;

    // alternatively we could also call update.download() and update.install() separately
    update
      .download_and_install(
        |chunk_length, content_length| {
          downloaded += chunk_length;
          println!("downloaded {downloaded} from {content_length:?}");
        },
        || {
          println!("download finished");
        },
      )
      .await?;

    println!("update installed");
    app.restart();
  }

  Ok(())
}
```

:::tip
To notify the frontend of the download progress consider using a command with a [channel].

<details>
  <summary>Updater command</summary>

```rust
#[cfg(desktop)]
mod app_updates {
    use std::sync::Mutex;
    use serde::Serialize;
    use tauri::{ipc::Channel, AppHandle, State};
    use tauri_plugin_updater::{Update, UpdaterExt};

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Updater(#[from] tauri_plugin_updater::Error),
        #[error("there is no pending update")]
        NoPendingUpdate,
    }

    impl Serialize for Error {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.to_string().as_str())
        }
    }

    type Result<T> = std::result::Result<T, Error>;

    #[derive(Clone, Serialize)]
    #[serde(tag = "event", content = "data")]
    pub enum DownloadEvent {
        #[serde(rename_all = "camelCase")]
        Started {
            content_length: Option<u64>,
        },
        #[serde(rename_all = "camelCase")]
        Progress {
            chunk_length: usize,
        },
        Finished,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UpdateMetadata {
        version: String,
        current_version: String,
    }

    #[tauri::command]
    pub async fn fetch_update(
        app: AppHandle,
        pending_update: State<'_, PendingUpdate>,
    ) -> Result<Option<UpdateMetadata>> {
        let channel = "stable";
        let url = url::Url::parse(&format!(
            "https://cdn.myupdater.com/{{{{target}}}}-{{{{arch}}}}/{{{{current_version}}}}?channel={channel}",
        )).expect("invalid URL");

      let update = app
          .updater_builder()
          .endpoints(vec![url])?
          .build()?
          .check()
          .await?;

      let update_metadata = update.as_ref().map(|update| UpdateMetadata {
          version: update.version.clone(),
          current_version: update.current_version.clone(),
      });

      *pending_update.0.lock().unwrap() = update;

      Ok(update_metadata)
    }

    #[tauri::command]
    pub async fn install_update(pending_update: State<'_, PendingUpdate>, on_event: Channel<DownloadEvent>) -> Result<()> {
        let Some(update) = pending_update.0.lock().unwrap().take() else {
            return Err(Error::NoPendingUpdate);
        };

        let started = false;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    if !started {
                        let _ = on_event.send(DownloadEvent::Started { content_length });
                        started = true;
                    }

                    let _ = on_event.send(DownloadEvent::Progress { chunk_length });
                },
                || {
                    let _ = on_event.send(DownloadEvent::Finished);
                },
            )
            .await?;

        Ok(())
    }

    struct PendingUpdate(Mutex<Option<Update>>);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                app.handle().plugin(tauri_plugin_updater::Builder::new().build());
                app.manage(app_updates::PendingUpdate(Mutex::new(None)));
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            #[cfg(desktop)]
            app_updates::fetch_update,
            #[cfg(desktop)]
            app_updates::install_update
        ])
}
```

</details>
:::

For more information see the [Rust API documentation].

</TabItem>
</Tabs>

Note that restarting your app immediately after installing an update is not required and you can choose
how to handle the update by either waiting until the user manually restarts the app, or prompting them to select when to do so.

:::note
On Windows the application is automatically exited when the install step is executed due to a limitation of Windows installers.
:::

When checking and downloading updates it is possible to define a custom request timeout, a proxy and request headers.

<Tabs syncKey="lang">
<TabItem label="JavaScript">

```js
import { check } from '@tauri-apps/plugin-updater';

const update = await check({
  proxy: '<proxy url>',
  timeout: 30000 /* milliseconds */,
  headers: {
    Authorization: 'Bearer <token>',
  },
});
```

  </TabItem>

  <TabItem label="Rust">

```rust
use tauri_plugin_updater::UpdaterExt;
let update = app
  .updater_builder()
  .timeout(std::time::Duration::from_secs(30))
  .proxy("<proxy-url>".parse().expect("invalid URL"))
  .header("Authorization", "Bearer <token>")
  .build()?
  .check()
  .await?;
```

  </TabItem>
</Tabs>

### Runtime Configuration

The updater APIs also allows the updater to be configured at runtime for more flexibility.
For security reasons some APIs are only available for Rust.

#### Endpoints

Setting the URLs that should be requested to check updates at runtime allows
more dynamic updates such as separate release channels:

```rust
use tauri_plugin_updater::UpdaterExt;
let channel = if beta { "beta" } else { "stable" };
let update_url = format!("https://{channel}.myserver.com/{{{{target}}}}-{{{{arch}}}}/{{{{current_version}}}}");

let update = app
  .updater_builder()
  .endpoints(vec![update_url])?
  .build()?
  .check()
  .await?;
```

:::tip
Note that when using format!() to interpolate the update URL you need double escapes for the variables
e.g. `{{{{target}}}}`.
:::

#### Public key

Setting the public key at runtime can be useful to implement a key rotation logic.
It can be set by either the plugin builder or updater builder:

```rust
tauri_plugin_updater::Builder::new().pubkey("<your public key>").build()
```

```rust
use tauri_plugin_updater::UpdaterExt;

let update = app
  .updater_builder()
  .pubkey("<your public key>")
  .build()?
  .check()
  .await?;
```

#### Custom target

By default the updater lets you use the `{{target}}` and `{{arch}}` variables to determine which update asset must be delivered.
If you need more information on your updates (e.g. when distributing a Universal macOS binary option or having more build flavors)
you can set a custom target.

<Tabs syncKey="lang">
<TabItem label="JavaScript">

```js
import { check } from '@tauri-apps/plugin-updater';

const update = await check({
  target: 'macos-universal',
});
```

  </TabItem>

  <TabItem label="Rust">

Custom targets can be set by either the plugin builder or updater builder:

```rust
tauri_plugin_updater::Builder::new().target("macos-universal").build()
```

```rust
use tauri_plugin_updater::UpdaterExt;
let update = app
  .updater_builder()
  .target("macos-universal")
  .build()?
  .check()
  .await?;
```

:::tip
The default `$target-$arch` key can be retrieved using `tauri_plugin_updater::target()`
which returns an `Option<String>` that is `None` when the updater is not supported on the current platform.
:::

  </TabItem>
</Tabs>

:::note

- When using a custom target it might be easier to use it exclusively to determine the update platform,
  so you could remove the `{{arch}}` variable.
- The value provided as target is the key that is matched against the platform key when using a [Static JSON file](#static-json-file).

:::

#### Allowing downgrades

By default Tauri checks if the update version is greater than the current app version to verify if it should update or not.
To allow downgrades, you must use the updater builder's `version_comparator` API:

```rust
use tauri_plugin_updater::UpdaterExt;

let update = app
  .updater_builder()
  .version_comparator(|current, update| {
    // default comparison: `update.version > current`
    update.version != current
  })
  .build()?
  .check()
  .await?;
```

#### Windows before exit hook

Due to a limitation of Windows installers, Tauri will automatically quit your application before installing updates on Windows.
To perform an action before that happens, use the `on_before_exit` function:

```rust
use tauri_plugin_updater::UpdaterExt;

let update = app
  .updater_builder()
  .on_before_exit(|| {
    println!("app is about to exit on Windows!");
  })
  .build()?
  .check()
  .await?;
```

:::note
The values from the [configuration](#tauri-configuration) are used as fallback if any of the builder values are not set.
:::

[`createUpdaterArtifacts`]: /reference/config/#createupdaterartifacts
[Distributing with CrabNebula Cloud]: /distribute/crabnebula-cloud/
[channel]: /develop/calling-frontend/#channels
[JavaScript API Documentation]: /reference/javascript/updater/
[Rust API Documentation]: https://docs.rs/tauri-plugin-updater

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4}
{
  "permissions": [
    ...,
    "updater:default",
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: upload.mdx

---

```mdx
---
title: Upload
description: File uploads through HTTP.
plugin: upload
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Upload files from disk to a remote server over HTTP. Download files from a remote HTTP server to disk.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

<Tabs>
	<TabItem label="Automatic">

    Use your project's package manager to add the dependency:

    <CommandTabs
    npm="npm run tauri add upload"
    yarn="yarn run tauri add upload"
    pnpm="pnpm tauri add upload"
    deno="deno task tauri add upload"
    bun="bun tauri add upload"
    cargo="cargo tauri add upload"
    />

    </TabItem>
    <TabItem label="Manual">

    	<Steps>

          1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

              ```sh frame=none
              cargo add tauri-plugin-upload
              ```

          2. Modify `lib.rs` to initialize the plugin:

              ```rust title="src-tauri/src/lib.rs" ins={4}
              #[cfg_attr(mobile, tauri::mobile_entry_point)]
              pub fn run() {
                tauri::Builder::default()
                  .plugin(tauri_plugin_upload::init())
                    .run(tauri::generate_context!())
                    .expect("error while running tauri application");
              }
              ```

          3. Install the JavaScript Guest bindings using your preferred JavaScript package manager:

              <CommandTabs
                  npm="npm install @tauri-apps/plugin-upload"
                  yarn="yarn add @tauri-apps/plugin-upload"
                  pnpm="pnpm add @tauri-apps/plugin-upload"
                  deno="deno add npm:@tauri-apps/plugin-upload"
                  bun="bun add @tauri-apps/plugin-upload"
              />

    	</Steps>
    </TabItem>

</Tabs>

## Usage

Once you've completed the registration and setup process for the plugin, you can access all of its APIs through the JavaScript guest bindings.

Here's an example of how you can use the plugin to upload and download files:

```javascript
import { upload } from '@tauri-apps/plugin-upload';
// when using `"withGlobalTauri": true`, you may use
// const { upload } = window.__TAURI__.upload;

upload(
  'https://example.com/file-upload',
  './path/to/my/file.txt',
  ({ progress, total }) =>
    console.log(`Uploaded ${progress} of ${total} bytes`), // a callback that will be called with the upload progress
  { 'Content-Type': 'text/plain' } // optional headers to send with the request
);
```

```javascript
import { download } from '@tauri-apps/plugin-upload';
// when using `"withGlobalTauri": true`, you may use
// const { download } = window.__TAURI__.upload;

download(
  'https://example.com/file-download-link',
  './path/to/save/my/file.txt',
  ({ progress, total }) =>
    console.log(`Downloaded ${progress} of ${total} bytes`), // a callback that will be called with the download progress
  { 'Content-Type': 'text/plain' } // optional headers to send with the request
);
```

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4}
{
  "permissions": [
    ...,
    "upload:default",
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: websocket.mdx

---

```mdx
---
title: Websocket
description: Open a WebSocket connection using a Rust client in JavaScript.
plugin: websocket
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Steps, Tabs, TabItem } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Open a WebSocket connection using a Rust client in JavaScript.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the websocket plugin to get started.

<Tabs>
	<TabItem label="Automatic" >
		Use your project's package manager to add the dependency:

    		{ ' ' }

    		<CommandTabs
                npm="npm run tauri add websocket"
                yarn="yarn run tauri add websocket"
                pnpm="pnpm tauri add websocket"
                bun="bun tauri add websocket"
                deno="deno task tauri add websocket"
                cargo="cargo tauri add websocket"
    		/>
    </TabItem>

    <TabItem label = "Manual">
    	<Steps>

          1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

              ```sh frame=none
              cargo add tauri-plugin-websocket
              ```

          2. Modify `lib.rs` to initialize the plugin:

              ```rust title="src-tauri/src/lib.rs" ins={4}
              #[cfg_attr(mobile, tauri::mobile_entry_point)]
              pub fn run() {
                  tauri::Builder::default()
                      .plugin(tauri_plugin_websocket::init())
                      .run(tauri::generate_context!())
                      .expect("error while running tauri application");
              }
              ```

          3. Install the JavaScript Guest bindings using your preferred JavaScript package manager:

              <CommandTabs
                      npm = "npm install @tauri-apps/plugin-websocket"
                      yarn = "yarn add @tauri-apps/plugin-websocket"
                      pnpm = "pnpm add @tauri-apps/plugin-websocket"
                      deno = "deno add npm:@tauri-apps/plugin-websocket"
                      bun = "bun add @tauri-apps/plugin-websocket"
              />

    	</Steps>
    </TabItem>

</Tabs>

## Usage

The websocket plugin is available in JavaScript.

```javascript
import WebSocket from '@tauri-apps/plugin-websocket';
// when using `"withGlobalTauri": true`, you may use
// const WebSocket = window.__TAURI__.websocket;

const ws = await WebSocket.connect('ws://127.0.0.1:8080');

ws.addListener((msg) => {
  console.log('Received Message:', msg);
});

await ws.send('Hello World!');

await ws.disconnect();
```

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={6}
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": ["websocket:default"]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```

---

## 📄 文件: window-state.mdx

---

```mdx
---
title: Window State
description: Persist window sizes and positions.
plugin: window-state
i18nReady: true
---

import PluginLinks from '@components/PluginLinks.astro';
import Compatibility from '@components/plugins/Compatibility.astro';

import { Tabs, TabItem, Steps } from '@astrojs/starlight/components';
import CommandTabs from '@components/CommandTabs.astro';
import PluginPermissions from '@components/PluginPermissions.astro';

<PluginLinks plugin={frontmatter.plugin} />

Save window positions and sizes and restore them when the app is reopened.

## Supported Platforms

<Compatibility plugin={frontmatter.plugin} />

## Setup

Install the window-state plugin to get started.

<Tabs>
  <TabItem label="Automatic">

Use your project's package manager to add the dependency:

{' '}

<CommandTabs
  npm="npm run tauri add window-state"
  yarn="yarn run tauri add window-state"
  pnpm="pnpm tauri add window-state"
  deno="deno task tauri add window-state"
  bun="bun tauri add window-state"
  cargo="cargo tauri add window-state"
/>

  </TabItem>
  <TabItem label="Manual">
    <Steps>

    1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

        ```sh frame=none
        cargo add tauri-plugin-window-state --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
        ```

    2.  Modify `lib.rs` to initialize the plugin:

        ```rust title="src-tauri/src/lib.rs" ins={4}
        #[cfg_attr(mobile, tauri::mobile_entry_point)]
        pub fn run() {
            tauri::Builder::default()
                .setup(|app| {
                    #[cfg(desktop)]
                    app.handle().plugin(tauri_plugin_window_state::Builder::default().build());
                    Ok(())
                })
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        ```

    3.  Install the JavaScript Guest bindings using your preferred JavaScript package manager:

        <CommandTabs
            npm="npm install @tauri-apps/plugin-window-state"
            yarn="yarn add @tauri-apps/plugin-window-state"
            pnpm="pnpm add @tauri-apps/plugin-window-state"
            deno="deno add npm:@tauri-apps/plugin-window-state"
            bun="bun add @tauri-apps/plugin-window-state"
        />

      </Steps>

  </TabItem>
</Tabs>

## Usage

After adding the window-state plugin, all windows will remember their state when the app is being closed and will restore to their previous state on the next launch.

You can also access the window-state plugin in both JavaScript and Rust.

:::tip
Restoring the state will happen after window creation,
so to prevent the window from flashing, you can set `visible` to `false` when creating the window,
the plugin will show the window when it restores the state
:::

### JavaScript

You can use `saveWindowState` to manually save the window state:

```javascript
import { saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state';
// when using `"withGlobalTauri": true`, you may use
// const { saveWindowState, StateFlags } = window.__TAURI__.windowState;

saveWindowState(StateFlags.ALL);
```

Similarly you can manually restore a window's state from disk:

```javascript
import {
  restoreStateCurrent,
  StateFlags,
} from '@tauri-apps/plugin-window-state';
// when using `"withGlobalTauri": true`, you may use
// const { restoreStateCurrent, StateFlags } = window.__TAURI__.windowState;

restoreStateCurrent(StateFlags.ALL);
```

### Rust

You can use the `save_window_state()` method exposed by the `AppHandleExt` trait:

```rust
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

// `tauri::AppHandle` now has the following additional method
app.save_window_state(StateFlags::all()); // will save the state of all open windows to disk
```

Similarly you can manually restore a window's state from disk using the `restore_state()` method exposed by the `WindowExt` trait:

```rust
use tauri_plugin_window_state::{WindowExt, StateFlags};

// all `Window` types now have the following additional method
window.restore_state(StateFlags::all()); // will restore the window's state from disk
```

## Permissions

By default all potentially dangerous plugin commands and scopes are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](/security/capabilities/) for more information and the [step by step guide](/learn/security/using-plugin-permissions/) to use plugin permissions.

```json title="src-tauri/capabilities/default.json" ins={4}
{
  "permissions": [
    ...,
    "window-state:default",
  ]
}
```

<PluginPermissions plugin={frontmatter.plugin} />
```
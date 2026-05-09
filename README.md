# Regos

Regos is a lightweight, background RAM reader and data exporter for **an anime train game**. 
It quietly runs in the background and extracts your account's character builds, relics, and light cones into a format fully compatible with popular community optimizers (like Fribbels).

> [!IMPORTANT]
> **Compatibility Note:** This tool is specifically designed for game version **4.3 and above (4.3++)**. It may not function correctly on older versions.

## Features
- **Ultra Lightweight:** The core DLL is extremely small.
- **On-Demand Dumping:** Data is only processed when you specifically request it.
- **Auto-Injector & Updater:** Comes with a smart loader that manages everything for you.
- **Seamless Updates:** The loader automatically checks for the latest `regos.dll` and updates it before injection.

## Requirements
- Windows 10 / 11 (64-bit)
- Rust toolchain (if you want to build from source)

## How to Build
Clone the repository and build using Cargo:
```bash
git clone https://github.com/leovn85/regos
cd regos
cargo build --release
```
The compiled files will be located in the `target/release/` folder. You need two files:
- `regos.dll` (The core reader)
- `regos_loader.exe` (The auto-injector & updater)


## How to Use
1. Place both **regos.dll** and **regos_loader.exe** in the same folder. *Note: Don't put them inside the game folder.*
2. Run **regos_loader.exe** as Administrator.
3. The loader will automatically check if a new version of `regos.dll` is available on GitHub and update it for you.
    - *Note: If you choose to use a third-party injector instead of our loader, you must manually download the latest `regos.dll` from the GitHub Releases page.*
4. Open your game launcher and start the game.
5. The loader will detect the game and automatically inject **regos.dll**.
6. In the game, open your **Character/Inventory** screen (this ensures the game loads your relic data into RAM).
7. Press the **F10** key on your keyboard.
8. You will hear a Windows "Ding" sound indicating the process is complete.
9. Look inside your game's directory. You will find an `archive_output-YYYY-MM-DD_HH-MM-SS.json` file.
10. Import this JSON file into your favorite optimizer!

## Safety & Disclaimer
Regos does not modify game memory or inject any malicious code. It only reads the structures already present in your RAM. However, using any third-party tool carries inherent risks.

Use at your own risk. The authors are not responsible for any account penalties that may occur from using this software.

## License
MIT License. See the LICENSE file for details.

## Credits
* Hessiser: (https://github.com/hessiser/il2cpp-rust)
* NuShen: (Base IL2CPP implementation)
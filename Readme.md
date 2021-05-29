# Minimal Windows 10 SDK

Installs only the necessary Windows 10 `.lib` files to save you having to download the full Visual Studio package.

## How to Install

First Visual Studio needs to be installed, but not as much as usual. Using powershell:

    > Invoke-WebRequest https://aka.ms/vs/16/release/vs_buildtools.exe -OutFile vs_buildtools.exe
    > .\vs_buildtools --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.Component.VC.Runtime.UCRTSDK

This will install the VS code package manager then open up the GUI installer. You can click straight on install. All the necessary components are already selected.

Wait for that to finish then run this installer. Alternatively, see [Manually Install Only the Libs](#manually-install-only-the-libs).

    > Invoke-WebRequest https://github.com/ChrisDenton/minwinsdk/releases/download/0.0.1/minwinsdk.exe  -OutFile minwinsdk.exe
    > .\minwinsdk

If all goes well you should finally be able to install rustup

    > Invoke-WebRequest https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe
    > .\rustup-init -y --default-host x86_64-pc-windows-msvc

## Manually Install Only the Libs

Download [Minimal.Windows.SDK.zip](https://github.com/ChrisDenton/minwinsdk/releases/download/0.0.1/Minimal.Windows.SDK.zip) and unzip it somewhere nice. You may want to set the `LIB` environment variable for your default target.

## Using the libs

If you used the installer or manually set the `LIB` environment variable then it should "just work" for the default target. If using another target you may need to set the library search path appropriately. You can do this globally in your [`cargo configuration file`](https://doc.rust-lang.org/cargo/reference/config.html). By default it's located at `%USERPROFILE%\.cargo\config.toml` but you might need to create it.

Add this to the config (replacing `path\to\libs` with the folder the libs are in):
```
[target.x86_64-pc-windows-msvc]
rustflags = [ "-L", "path\to\libs\x64"]

[target.i686-pc-windows-msvc]
rustflags = [ "-L", "path\to\libs\x86"]

[target.aarch64-pc-windows-msvc]
rustflags = [ "-L", "path\to\libs\arm64"]

[target.thumbv7a-pc-windows-msvc]
rustflags = [ "-L", "path\to\libs\arm"]
```

## Limitations

* I have not yet tested that all the imports work.
* I only provide an `x86_64` installer. There's no inherent reason for this, it's just that I'm being lazy.
* These libs do not (yet?) include `ucrt.lib`.
* I wrote this in a very short space of time so this should be considered alpha quality software (if we're being very generous). Honestly it's probably better to just download the zip file and install the libs manually. That way you'll avoid any scary warnings berating you for downloading software from the internet. Which you should never do. 👀

## The future

This should become obsolete once Rust has [raw-dylibs](https://rust-lang.github.io/rfcs/2627-raw-dylib-kind.html).

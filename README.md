# FFmpeg Context Menu

Adds (my) commonly used FFmpeg commands to the right click context menu in Windows.

## Install

1. You need [FFmpeg](https://ffmpeg.org/download.html) downloaded and included in your path.
2. Have [Rust](https://www.rust-lang.org/tools/install) installed.
3. Clone or download this repo
4. While in the folder run `cargo build --release`.
5. Add the binary path and values (listed below) to the registry so they appear on a right click in File Explorer (tutorial coming soon).

## Run

### Right Click Context Menu

You can either add the binary path and values to the registry to have it on your right click context menu when you click on a file.

### Command Line

Alternatively, you can add the binary to your path and run it like this from command line:

```bash
ffmpeg-context-menu <full-path-to-file> <value>
```

Example:

```bash
ffmpeg-context-menu "C:\Users\<username>\Videos\video.mp4" "new"
```

All of these make a new video file, the original isn't edited in any way. These are the accepted values and what they do:

- `"new"` - new file, uses the flag `-crf 28`
- `"half_size"` - new file, half the width of the original, and with the flag `-crf 28`
- `"audio"` - new .mp3 file from the audio (currently only pulls audio from the first audio track)
- `"mp4"` - new .mp4 file
- `"no_audio"` - new file, all audio tracks removed, and with the `-crf 28` flag
- `"fix"` - new file with the `-err_detect ignore_err` flag
- `"265"` - new file in the 265 format
- `"remove_border"` - **Coming soon**, finds and removes the black borders / space around the video, and crops it

[Here is the documentation](https://www.ffmpeg.org/ffmpeg-codecs.html) on FFmpeg flags, if you would like to make your own commands.

## Todo

- [ ] finding and cropping out black borders from videos
- [ ] make the `"audio"` command pull audio from all audio tracks and not just the first one
- [ ] writing install script in Rust that will add all these commands to registry in Windows
- [ ] writing uninstall script in Rust that will remove all these commands from the registry in Windows
- [ ] add tutorial on how to add and remove these values from the registry manually
- [ ] add support for Linux?

## What does this program do?

There's some FFmpeg commands I frequently use and I found it helpful to have them easily available from the right click context menu, rather than have to type everything out in command line, or use a GUI like [HandBrake](https://handbrake.fr) or [Shutter Encoder](https://www.shutterencoder.com).

## Why Rust?

I originally made this in Python but I would like to learn Rust so I decided rewriting this simple command line program would be a good way to learn.

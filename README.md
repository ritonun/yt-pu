# yt-pu

YouTube Playlist updater is a tool to keep a local download of your playlist (MP3 format)

## Install
Dependencies: [yt-dlp](https://github.com/yt-dlp/yt-dlp)

Download the latest version of [yt-pu](https://github.com/ritonun/yt-pu/releases)
Run with `./yt-pu` or add yt-pu to `$PATH`

## Use
Standard use:
```bash
yt-pu --output_path [path] url
```

If you want to sync your local folder to your playlist, you can delete the local files when you remove a video from the playlist by using the `--delete_local` flag.
```bash
yt-pu --output_path [path] --delete_local url
```

# cmdj

Simple CLI tool that lets you be notified when your shell command finishes execution

## Usage

Player `cmdj` lets tou play music along your shell command excecution and on its finish. It very useful during long build, install or update
processes, since you can focus on other thing and stay alerted whenether process finishes.

Audio files are defined in `<HOME>/.cmdj/config.json` config file. To see them run `cmdj ls`

```sh
$ cmdj ls
My playlist:
  start: Music/my_start_music.mp3
  success: Music/this_is_good.mp3
  error: Music/something_bad.mp3

Other playlist:
  success: Music/opa.mp3
  error: Music/opa.mp3

```

Choose your playlist and run `cmdj run [PLAYLIST]`. If no `PLAYLIST` was specified then the first
one defined in `config.json` will be chosen. Then type your shell command and enjoy some music

```sh
$ cmdj run MyPlaylist
>> sudo apt update && sudo apt upgrade
...

```

`start` audio plays right after command is entered. `success` play after command exit status 0 execution and `error` on any other.

## Install

Unix/Linux
```sh
git clone https://github.com/Ask4r/cmdj
cd cmdj
cargo build --release
sudo mv target/release/cmdj /usr/bin/cmdj
```

## Uninstall

Unix/Linux
```sh
rm -rf ~/.cmdj
sudo rm /usr/bin/cmdj
```

## Config

Configuration file is located at `<HOME>/.cmdj/config.json`. It only stores your `playlists` - a set of music files paths relative
to `<HOME>` directory. Each playlist can optionally have `start`, `success` or `error` fields.

```json
{
    "My playlist": {
        "start": "Music/my_start_music.mp3",
        "success": "Music/this_is_good.mp3",
        "error": "Music/something_bad.mp3"
    },
    "Other playlist": {
        "success": "Music/opa.mp3",
        "error": "Music/opa.mp3"
    }
}
```

Every field can be or be not specified. If last option is chosen the player won't start audio at according step.
Possibly you can create a new directory under `.cmdj` dotdir to store audios separately from other system files
since only `config.json` is used by `cmdj` itself.

## LICENSE
cmdj is granted under the MIT liscense. Contributions are welcome!

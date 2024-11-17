# Invaders

Invaders is an open source terminal arcade game with audio, based off of the "Space Invaders" classic arcade game.

### Sound Files

If you want the sound files used in the course video, but don't want to clone the repository, here are all the sounds in two different archive formats (the sound files are the same):

- [sounds.zip](https://github.com/CleanCut/invaders/files/6312508/sounds.zip)
- [sounds.tar.gz](https://github.com/CleanCut/invaders/files/6312511/sounds.tar.gz)

Notice the new path to the sound files used in the course.
It is now under `audio` > `original`.
All new contributions can be placed under `audio` > `contributions`.

We have now a new set of instrumental music sound files in MP3 format.
You may listen to the sounds on [Musescore.com](https://musescore.com/user/9047536/sets/5156900).

### Dependencies on Linux

Audio should work out-of-the-box on macOS, Windows, and iOS.  For Linux, the
downstream package for actually _playing_ sound ([CPAL]) requires
the *Alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev pkg-config
```
**Arch Linux**

```bash
sudo pacman -S alsa-lib pkgconf libx11
```
You will also need `pipewire-alsa` or `pulseaudio-alsa` depending on the sound server you are using.

## Community Games!

Were you inspired to make your own terminal-based game? Open a PR to add it to the list here!

* [Pong](https://github.com/basilkohler/rusty_pong)
* [TETRIS](https://github.com/madchicken/rust-tetris)
* [Columns](https://github.com/Rendez/rust_columns)
* [Q-Agent Driven Invaders](https://github.com/indiVar0508/Q-agent-driven-invaders/tree/q_agent)

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license.

See [license](LICENSE)


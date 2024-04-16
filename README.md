<p align="center">
  <a href="https://youtu.be/WXDTsJ4cqO4"><img src="docs/demo.apng"></a>
</p>

</br>

<div align="center">
  <img width="128px" src="data/fyi.zoey.Boop-GTK.svg" >
</div>

<h1 align="center">Boop-GTK</h1>
<h3 align="center">A scriptable scratchpad for developers. Port of <a href="https://github.com/IvanMathy"><b>@IvanMathy</b></a>'s <a href="https://github.com/IvanMathy/Boop">Boop</a> to GTK</h3>
<p align="center"><i>Also checkout <a href="https://github.com/zoeyfyi/TeX-Match">TeX Match</a>: Find LaTeX symbols by sketching</i></p>


<p align="center">
  <a href="#what-is-boop-gtk">What is Boop-GTK?</a> • <a href="#features">Features</a> • <a href="#downloads">Downloads</a> • <a href="#screenshots">Screenshots</a> • <a href="#usage">Usage</a> • <a href="#additional-scripts">Additional Scripts</a> • <a href="#building">Building</a>
</p>


</br>

### What is Boop-GTK?

[Boop](https://github.com/IvanMathy) is a simple editor that allows you to execute scripts on the buffer. The idea is that you don’t have to paste potentially secret information into shady websites to do some simple transforms, like format json and decoding query strings.

Boop-GTK is a port of Boop to GTK, so users on Linux can Boop it!

### Features

- 50+ builtin scripts including "Base64 Encode", "Format JSON", "Hex to RGB" and more
- 100% script compatibility with [Boop](https://github.com/IvanMathy/Boop)
- Completely crossplatform!

### Screenshots

![linux screenshot](screenshot.png)

There is also a quick demo on [youtube](https://youtu.be/WXDTsJ4cqO4).

### Usage

More documentation can be found in [Boop's docs](https://github.com/IvanMathy/Boop/blob/main/Boop/Documentation/Readme.md).

Boop-GTK is easy to use: open it, paste some text, run some scripts, optionally copy the text out.

- [Custom Scripts](https://github.com/IvanMathy/Boop/blob/main/Boop/Documentation/CustomScripts.md)
- [Modules](https://github.com/IvanMathy/Boop/blob/main/Boop/Documentation/Modules.md)
- [Converting Node Modules](https://github.com/IvanMathy/Boop/blob/main/Boop/Documentation/ConvertingNodeModules.md)
- [Global Scripts](docs/GlobalScripts.md) (unique to Boop-GTK)

### Additional Scripts

More scripts can be found in the [Boop repo](https://github.com/IvanMathy/Boop/tree/main/Scripts). These scripts can also be found in the <a href="https://aur.archlinux.org/packages/boop-gtk-extra-scripts/"><code>boop-gtk-extra-scripts</code></a> package on the AUR.

### Building

#### Binary

```shell
sudo apt-get install -y libgtk-3-dev libgtksourceview-3.0-dev
cargo build
```

#### Nix

```shell
nix run github:msfjarvis/Boop-GTK?submodules=1#
```

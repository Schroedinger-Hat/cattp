<div align='center'>
  
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h1>ca<i>ttp</i></h1>
  
  <a href="https://github.com/Schrodinger-Hat/cattp">
    <img src="public/sh.png" alt="Logo" width="80" height="80">
  </a>

  <p align="center">
    <br />
    <a href="https://github.com/Schrodinger-Hat/cattp/blob/main/README.md"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://www.schrodinger-hat.it/">View Demo</a>
    ·
    <a href="https://github.com/Schrodinger-Hat/cattp/issues">Report Bug</a>
    ·
    <a href="https://github.com/Schrodinger-Hat/cattp/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#tech-stack">Built With</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

### Tech stack

This is built in `rust`

<!-- USAGE EXAMPLES -->

## Installation

### Install it with Homebrew

Coming soon...

```
 brew tap Schrodinger-Hat/cattp https://github.com/Schrodinger-Hat/cattp
 brew update
 brew install cattp
```

### Install it manually

Unzip the `cattp.tar.gz` and `mv` the executable so that it can be found in your `$PATH` variable.

```
mv cattp /usr/local/bin/

chmod +x /usr/local/bin/cattp

cattp
```

### Build it

Install `rust` on your systems. Run `cargo install --path [a path of your choice]`.

## Usage

`cattp [-s [--status] int] [-d [--description] -o [--open]]`

`-s, --status`

`-d, --description`
Default value `false`

`-o, --open`
Default value `false`

_example_
`cattp -s 404 -d`
`cattp -s 404 -o`
`cattp -s 404 -do`

The console will output the status code and also open your system's web browser if `o` flag is present.

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

<!-- CONTACT -->

## Contact

Schrodinger's Hat Team - [@schrodinger_hat](mailto:schrodinger.hat.show@gmail.com)

Project Link: [https://github.com/Schrodinger-Hat/cattp](https://github.com/Schrodinger-Hat/cattp)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/Schrodinger-Hat/cattp.svg?style=for-the-badge
[contributors-url]: https://github.com/Schrodinger-Hat/cattp/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Schrodinger-Hat/cattp.svg?style=for-the-badge
[forks-url]: https://github.com/Schrodinger-Hat/cattp/network/members
[stars-shield]: https://img.shields.io/github/stars/Schrodinger-Hat/cattp?style=for-the-badge
[stars-url]: https://github.com/Schrodinger-Hat/cattp/stargazers
[issues-shield]: https://img.shields.io/github/issues/Schrodinger-Hat/cattp.svg?style=for-the-badge
[issues-url]: https://github.com/Schrodinger-Hat/cattp/issues

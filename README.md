# cvtemp

Rust-OpenCV bounded together to do template matching, thing

## Sample Usage

Template matching is to find where some portion (needle) of a image (haystack) is

| Haystack                      | Needle                            | Match                           |
|:------------------------------|:----------------------------------|:--------------------------------|
| ![host](./sample/host.png)    | ![target](./sample/target.png)    | ![match](./sample/match.png)    |
| ![host 2](./sample/host2.png) | ![target 2](./sample/target2.png) | ![match 2](./sample/match2.png) |

You can reproduce the examples by running
```shell
git clone https://github.com/zhufucdev/cvtemp.git
cd cvtemp
cargo run -- -o -t 0.99 ./sample/host.png ./sample/target.png ./sample/match.png
```

## Credits

[![twistedfall/opencv-rust - GitHub](https://gh-card.dev/repos/twistedfall/opencv-rust.svg)](https://github.com/twistedfall/opencv-rust)
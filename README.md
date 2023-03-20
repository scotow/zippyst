# zippyst

[![crates.io](https://img.shields.io/crates/v/zippyst.svg)](https://crates.io/crates/zippyst)
[![Documentation](https://docs.rs/zippyst/badge.svg)](https://docs.rs/zippyst)
![Test](https://github.com/scotow/zippyst/actions/workflows/tests.yml/badge.svg)

Extract direct download link from a [zippyshare.com](https://www.zippyshare.com) page. 

## ⚠️ Following Zippyshare's March 19, 2023 [announcement](https://blog.zippyshare.com/?p=1211)  of closing their services, this project will not receive any new development and has been archived. ⚠️ 

### Usage

```sh
USAGE:
    zippyst LINK...
```

### Examples

The links bellow were used to demonstrate the usage of the command. They may have expired.

```sh
$ cargo run 'https://www3.zippyshare.com/v/CDCi2wVT/file.html'
https://www3.zippyshare.com/d/CDCi2wVT/43392/Gillette%20%2c%20the%20best%20a%20man%20can%20get.wav

$ cargo run 'https://www20.zippyshare.com/v/oRFjDgWy/file.html' 'https://www20.zippyshare.com/v/GTU4Fiku/file.html' 'https://www20.zippyshare.com/v/QW589nBO/file.html'
https://www20.zippyshare.com/d/oRFjDgWy/40318/dev-v2.0.json
https://www20.zippyshare.com/d/GTU4Fiku/36115/run.js
https://www20.zippyshare.com/d/QW589nBO/26959/Gillette%20%2c%20the%20best%20a%20man%20can%20get.wav.download.zip

$ xargs -a links.txt zippyst
https://www20.zippyshare.com/d/oRFjDgWy/40318/dev-v2.0.json
https://www20.zippyshare.com/d/GTU4Fiku/36115/run.js
https://www20.zippyshare.com/d/QW589nBO/26959/Gillette%20%2c%20the%20best%20a%20man%20can%20get.wav.download.zip
```

### Algorithm changes

If you find any link that may not work with this project, please open an issue to let me know, so I might try to support the new algorithm.

### Disclaimer

This project uses and may run script that are downloaded from the download page. Use it at your own risk. 
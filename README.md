# zippyst

Extract direct download link from a Zippyshare page. 

### Usage

```sh
USAGE:
    zippyst LINK...
```

### Example

```sh
$ cargo run 'https://www3.zippyshare.com/v/CDCi2wVT/file.html'
https://www3.zippyshare.com/d/CDCi2wVT/43392/Gillette%20%2c%20the%20best%20a%20man%20can%20get.wav

$ cargo run 'https://www20.zippyshare.com/v/oRFjDgWy/file.html' 'https://www20.zippyshare.com/v/GTU4Fiku/file.html' 'https://www20.zippyshare.com/v/QW589nBO/file.html'
https://www20.zippyshare.com/d/oRFjDgWy/40318/dev-v2.0.json
https://www20.zippyshare.com/d/GTU4Fiku/36115/run.js
https://www20.zippyshare.com/d/QW589nBO/26959/Gillette%20%2c%20the%20best%20a%20man%20can%20get.wav.download.zip

$ cat links.txt | xargs zippyst
https://www20.zippyshare.com/d/oRFjDgWy/40318/dev-v2.0.json
https://www20.zippyshare.com/d/GTU4Fiku/36115/run.js
https://www20.zippyshare.com/d/QW589nBO/26959/Gillette%20%2c%20the%20best%20a%20man%20can%20get.wav.download.zip
```

### Algorithm changes

If you find any link that may not work with this project, please open an issue to let me know, so I can (try to) support the new algorithm.
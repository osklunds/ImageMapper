
# ImageMapper

`ImageMapper` is a Rust command line tool for mapping/synchronizing a directory structure into a new structure with the following properties:

- Only images and videos are kept
- The exif date/time of images are prepended to their file name
- The images are downscaled and compressed
- Images/videos/directories are only transformed/copied on change

The image displays an example run

![](Example.png)

The purpose of having the date/time prepended is to be able to sort images, from multiple cameras, by the time they were taken, without needing a special viewer software that can read exif tags. Windows Explorer or Finder can't sort by exif. But images transformed by `ImageMapper` become sorted by exif since the exif date/time is in the file name.

The purpose of compressing the images is so that it's faster to view and browse them over a network connection. On the internet as well as LANs.

This program has a real use case for me. Once a week, a Raspbery Pi runs the program to transform our collection of family photos to the new form. We then view the transformed, not the original, photos on the TV. Compared to viewing the original phots, we now get them in the correct order and faster loading times.

## Running

Show the help info of `ImageMapper` by typing `cargo run -- --help` and you see the following:

```
ImageMapper 
Maps the source directory structure to an equivalent structure in the destination
directory. The differences are: 1. Images will be downscaled and compressed. 2. Images
will get their exif date/time prepended to their file names. 3. Images (and optionally
videos) are the only files that will be kept.

USAGE:
    image_mapper [FLAGS] <source directory> <destination directory> <image quality>

FLAGS:
    -h, --help              Prints help information
    -i, --include-videos    Instead of just images, with this option, videos will also be
                            included in the destination. Note that they will just be
                            copied as-is without any conversion.
    -v, --verbose           Print when a directory is enterted and when a file is
                            added/delted. No matter of this setting, errors will always be
                            printed.

ARGS:
    <source directory>         The path to the directory that will be mapped.
    <destination directory>    The path to the directory where the result of the
                               mapping will be placed.
    <image quality>            Select if the images should be converted to the mobile
                               quality (1024x1024, 30 % compression) or the TV quality
                               (1920x1080, 70 % compression) [possible values: Mobile,
                               TV]
```

In a nutshell: supply the paths of the source and destination directories and the desired image quality. Optionally, use `-i` to copy videoes and `-v` for being verbose. Note that files in the destination directory can be deleted, so be careful to specify the correct path.

## Building, running, testing

Use `cargo build`, `cargo run` and `cargo test` as usual. When building the program for real use, include the `--release` flag. Then image conversions become significantly faster.

## Compatibility

TODO

## Credits

The images found in `test_resources` were taken by [Johann Siemens](https://unsplash.com/@johannsiemens?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText) and downloaded from [Unsplash](https://unsplash.com/search/photos/tree?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText).
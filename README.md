# imgreduce

## Introduction
imgreduce is a small CLI tool to bulk resize and/or convert images into a different file format. It uses [ImageMagick](https://imagemagick.org/index.php) to convert the images.

## Flags
| Short | Takes value | Function |
| ----- | ----------- | -------- |
| -d | Yes | Provides the base directory to perform the tasks on. Defaults to pwd if not used. |
| -s | Yes | Provides the desired new resolution of files. Should be provided as "numberxnumber" |
| -f | Yes | Provides the desired new format of files. Should be provided as .extention. |
| -r | No | Enables recursion to recursively add files from the provided base directory. |
| -p | No | Pretty print, displays when a file is being worked on, and at the end displays disk usage. |
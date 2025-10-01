# metanuke

A CLI tool that deletes all metadata from files.

Supported formats: `DOCX`, `FLAC`, `JPEG`, `MP3`, `MP4`, `PDF`, `PNG`, `WEBP`,
`XLSX`, `ZIP`


## Usage

```
metanuke [-o <file>|--replace] <file>
```

When run with no options, the original file is preserved and a new file is
created with a unique name. Use `-o` to set the name of the resulting file or
`--replace` to overwrite the original file instead.

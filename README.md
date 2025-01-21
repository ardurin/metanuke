# metanuke

A CLI tool that deletes all metadata from files.

Supported formats: `DOCX`, `JPEG`, `MP3`, `MP4`, `PDF`, `PNG`, `XLSX`

Considerations:
* Files are not validated. Malformed files produce undefined results.
* The actual content of a file is not scanned or processed in any way. If the content has sensitive information, it will not be deleted.

#!/bin/sh
check() {
	case "${1}" in
		*.flac)
			flac --test "${1}" >/dev/null 2>&1 && [ $(exiftool "${1}" | wc -l) = 21 ]
			;;
		*.jpeg)
			jpeginfo "${1}" >/dev/null 2>&1 && [ $(exiftool "${1}" | wc -l) = 19 ]
			;;
		*.mp3)
			mp3check -c "${1}" >/dev/null 2>&1 && [ $(exiftool "${1}" | wc -l) = 22 ]
			;;
		*.mp4)
			mp4ff-info "${1}" >/dev/null 2>&1
			;;
		*.pdf)
			qpdf --check "${name}" >/dev/null 2>&1 && [ $(pdfinfo "${name}" 2>/dev/null | wc -l) = 14 ]
			;;
		*.png)
			pngcheck "${1}" >/dev/null 2>&1
			;;
		*.zip)
			zip -T "${1}" >/dev/null 2>&1
			;;
	esac
}

status=0
for name in ${1}/*; do
	if ! check "${name}"; then
		echo "${name}"
		status=1
	fi
done
exit ${status}

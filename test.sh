#!/bin/sh
check() {
	case "${1}" in
		*.jpeg)
			jpeginfo "${1}" >/dev/null 2>&1 && [ $(exiftool "${1}" | wc -l) = 19 ]
			;;
		*.pdf)
			if ! message=$(qpdf --check "${1}" 2>&1); then
				[ $(printf "${message}" | grep -v 'is not one plus the highest' | wc -l) = 5 ]
			else
				[ $(pdfinfo "${1}" 2>/dev/null | wc -l) = 14 ]
			fi
			;;
		*.png)
			pngcheck "${1}" >/dev/null 2>&1
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

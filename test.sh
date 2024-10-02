#/!bin/sh
directory=$(mktemp -d)
processed=$(mktemp)
for suite in ${1:-jpeg pdf png}; do
	i=1
	for original in test/"${suite}"/*; do
		target/debug/mwipe -o "${processed}" "${original}"
		if [ "${suite}" = jpeg ]; then
			jpeginfo "${processed}" >/dev/null || echo "JPEG #${i}: failed"
		elif [ "${suite}" = png ]; then
			pngcheck "${processed}" >/dev/null || echo "PNG #${i}: failed"
		elif [ "${suite}" = pdf ]; then
			(qpdf --check "${processed}" >/dev/null 2>&1 && [ $(pdfinfo "${processed}" | wc -l) = 14 ]) || echo "PDF #${i}: failed"
		fi
		i=$((i + 1))
	done
done

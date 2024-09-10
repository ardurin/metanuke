#/!bin/sh
download() {
	curl -Lo "$2" "$1" >/dev/null 2>&1
}

directory=$(mktemp -d)
processed=$(mktemp)

for suite in ${1:-jpeg png}; do
	i=1
	if [ "${suite}" = jpeg ]; then
		original="${directory}"/x.jpeg
		while read address; do
			download "${address}" "${original}"
			../target/debug/mwipe -o "${processed}" "${original}"
			jpeginfo "${processed}" >/dev/null || echo "JPEG #${i}: failed"
			i=$((i + 1))
		done < "${suite}"
	elif [ "${suite}" = png ]; then
		original="${directory}"/x.png
		while read address; do
			download "${address}" "${original}"
			../target/debug/mwipe -o "${processed}" "${original}"
			pngcheck "${processed}" >/dev/null || echo "PNG #${i}: failed"
			i=$((i + 1))
		done < "${suite}"
	fi
done

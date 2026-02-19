while read -r i; do
	echo "$i"
	code "$i"
done < <(find src -name \*.rs | sort)

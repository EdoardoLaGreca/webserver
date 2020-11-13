#! /bin/sh

# This file should be used only by the developers to make relases.

# Check if a binary exists
check_bin() {
    local check=`command -v $1`
    if [ -z $check ]
    then
        echo "1"
    else
        echo "0"
    fi
}

# Check if dependencies are installed
echo "Checking the dependencies..."
for bin in "zip"
do
	if [ `check_bin $bin` = "0" ]
	then
		echo "$bin is installed."
	else
		echo "$bin is not installed, install it and try again."
		exit
	fi
done
echo "All the dependencies are installed."

target_dir=`cat script/target_dir.txt`
final_dir="$target_dir/final"
mkdir $final_dir

echo "Packing the releases..."
while read target
do
	# Create the pack name
	release_pack_name=""

	# Platform name/OS
	if [[ $target == *"windows"* ]]
	then
  		$release_pack_name="Windows"
	elif [[ $target == *"apple-darwin"* ]]
	then
		$release_pack_name="macOS"
	elif [[ $target == *"linux"* ]]
	then
		$release_pack_name="Linux"
	elif [[ $target == *"freebsd"* ]]
	then
		$release_pack_name="FreeBSD"
	fi
	
	# Architecture
	if [[ $target == *"x86_64"* ]]
	then
  		$release_pack_name="$release_pack_name-x86_64"
	elif [[ $target == *"i686"* ]]
	then
		$release_pack_name="$release_pack_name-i686"
	fi


	# Put things inside the pack
	release_pack_dir="$final_dir/$release_pack_name"
	mkdir "$release_pack_dir"

	# Copy the executable
	if [[ $target == *"windows"* ]]
	then
  		cp "$target_dir/$target/release/webserver.exe" "$release_pack_dir"
	else
		cp "$target_dir/$target/release/webserver" "$release_pack_dir"
	fi

	# Copy the license
	cp "LICENSE" "$release_pack_dir"

	# Copy other things if strictly necessary
	#cp ... "$release_pack_dir"

	zip -r "$release_pack_dir.zip" "$release_pack_dir"
done < script/target_list.txt
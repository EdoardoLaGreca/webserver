#! /bin/sh

### WARNING, READ BELOW ###
# This file should be used only by the developers to make relases.
# This script must be run only after the cross_compile.sh script.
# This script must be run in the repo's root directory
### WARNING, READ ABOVE ###

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

# Add the new dependencies after "in" (e.g. for bin in "dep1" "dep2" "dep3" do [...])
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

# Pack everthing
target_dir=`cat script/target_dir.txt`
mkdir $targets_dir

echo "Packing the releases..."

# If a platform or architecture does not have an alias, it will be skipped and
# skipped targets will be printed
skipped_targets=""
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
	else
		# Platform/OS not recognized, skip the target
		$skipped_targets="$skipped_targets\n$target"
		continue
	fi
	
	# Architecture
	if [[ $target == *"x86_64"* ]]
	then
  		$release_pack_name="$release_pack_name-amd64"
	elif [[ $target == *"i686"* ]]
	then
		$release_pack_name="$release_pack_name-i686"
	else
		# Architecture not recognized, skip the target
		$skipped_targets="$skipped_targets\n$target"
		continue
	fi

	# Put things inside the pack
	release_pack_dir="$target_dir/$release_pack_name"
	mkdir "$release_pack_dir"

	# Copy the executable
	if [[ $target == *"windows"* ]]
	then
  		cp "target/$target/release/webserver.exe" "$release_pack_dir"
	else
		cp "target/$target/release/webserver" "$release_pack_dir"
	fi

	# Copy the license
	cp "LICENSE" "$release_pack_dir"

	# Copy readme.txt
	cp "script/readme.txt" "$release_pack_dir"

	# Create the package
	zip -r "$release_pack_dir.zip" "$release_pack_dir"
done < script/target_list.txt

echo "Creating a pack containing the source code"

# Create the source code package directory
sc_target="$target_dir/source_code"
mkdir "$sc_target"

# Copy the source code package contents
cp -r "src/*" "$sc_target/src"
cp "Cargo.toml" "$sc_target/Cargo.toml"
cp "LICENSE" "$sc_target/LICENSE"
cp "README.md" "$sc_target/README.md"

# Make a script that compiles the source code
echo "#! /bin/sh

# Check if a binary exists
check_bin() {
    local check=`command -v \$1`
    if [ -z \$check ]
    then
        echo \"1\"
    else
        echo \"0\"
    fi
}

# Check if dependencies are installed
echo \"Checking the dependencies...\"
for bin in \"cargo\"
do
	if [ `check_bin \$bin` = \"0\" ]
	then
		echo \"\$bin is installed.\"
	else
		echo \"\$bin is not installed, install it and try again.\"
		exit
	fi
done
echo \"All the dependencies are installed.\"

mkdir \"bin\"

cargo build --release --out-dir bin
then" > "$sc_target/compile.sh"

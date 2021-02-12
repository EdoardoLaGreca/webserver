#! /bin/sh

# This file should be used only by the developers to make relases.

# This file is used to cross-compile to the following operating systems:
#  - Windows (Windows 7+)
#  - macOS (10.7+, Lion+)
#  - Linux (kernel 2.6.32+)
#  - FreeBSD
# in both i686 and x86_64.
# More operating systems (and architectures) may be added in future.

printf "This script should be used exclusively to make a new release since it requires a lot of time and space.
Continue? [y/n] "

read answer

if [ $answer != "y" ] && [ $answer != "Y" ]
then
	echo "Aborted."
	exit 1
fi

echo "!! MAKE SURE TO HAVE mingw-w64-gcc INSTALLED !!"

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
for bin in "rustup" "cargo"
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

# Install targets
echo "Installing targets..."
while read target
do
	rustup target add $target
	rustup toolchain install stable-$target
done < script/target_list.txt

#rustup component add rust-src
echo "Targets installed."

# Create the release directory where there will be placed all the binaries
target_dir=`cat script/target_dir.txt`
mkdir $target_dir

echo "Building..."
while read target
do
	cargo build --release --target-dir $target_dir --target $target
done < script/target_list.txt

echo "Now everything is ready for release, you can find the binaries on $target_dir/"
echo "It is advised to run pack.sh to automatically create the packages."
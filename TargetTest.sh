#Check if rust can succesfully build to selected target
# [?arg] if no arg was given build to system source

#dectect operating system
echo "Detecting system"
echo "OS:"$OSTYPE

#check the chipset for arm based stuff
echo "Detecting chipset"
chip=$(uname -m)
echo "Chip:"$chip

#Check if rust is installed on this machine
RUSTVERSION=$(rustc -V)
echo "Detecting rust"
#if rust is not installed exit
if [[ "$RUSTVERSION" = "" ]]; then
	echo "Rust is not installed"
	echo "Exiting automated build test"
	exit
fi
echo "Running rust version:"$RUSTVERSION

#Check the build targets
echo "Checking if there is a build test for selected target"
FILE="./test/targets/$chip-$OSTYPE.rs"
if [[ -f "$FILE" ]]; then
    echo "Running target test: $FILE"
    rustc $FILE -o TARGET.test

else
	echo "There is no test for specified target."
	echo "Running general target test..."
	rustc ./test/targets/general.rs -o TARGET.test
fi
#Move the file to the correct build location
if [ -d "./test/.builds/" ]; then
	cd ./test/.builds
	rm ./TARGET.test
	cd ../../
else
	cd ./test
	mkdir .builds
	cd ..
fi
mv ./TARGET.test ./test/.builds/
./test/.builds/TARGET.test
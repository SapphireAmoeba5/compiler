nasm -f elf64 -o output.o output.asm

if [ $? -ne 0 ]; then
	echo -e "\033[31mFailed to assemble progran\033[0m"
	exit
fi

ld output.o -o output

if [ $? -ne 0 ]; then
	echo -e "\033[31mFailed to link program\033[0m"
	exit
fi

rm output.o
./output

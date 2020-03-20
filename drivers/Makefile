CXX = i686-elf-gcc		# C compiler
CXXFLAGS = -ffreestanding -std=gnu99 -O2 -g -Wall -Wextra		# C compiler flags

SRC = $(wildcard ./*.c)
OBJ = $(patsubst %.c, %.o, $(SRC))

all: $(OBJ)

%.o: %.c
	$(CXX) $(CXXFLAGS) -o $@ -c $< 

clean:
	rm -f *.o

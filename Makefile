SRC      = Main.cpp
TARGET   = a.out
TEMPLATE = library/misc/template.cpp

CC = g++
CFLAGS  = -std=gnu++1y -O2
INCDIRS = -I/opt/boost/gcc/include
LDFLAGS = -L/opt/boost/gcc/lib

RM = rm -f
CP = cp -f

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) $(INCDIRS) $(LDFLAGS) -o $@ $^

.PHONY: clean

clean:
	$(RM) $(TARGET)
	$(CP) $(TEMPLATE) $(SRC)

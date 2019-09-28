SRC    = Main.cpp
TARGET = a.out
MISC   = library/misc

CC = g++
CFLAGS  = -std=gnu++1y -O2
INCDIRS = -I/opt/boost/gcc/include
LDFLAGS = -L/opt/boost/gcc/lib

RM = rm -f
CP = cp -f

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) $(INCDIRS) $(LDFLAGS) -o $@ $^

.PHONY: clean aoj

clean:
	$(RM) $(TARGET)
	$(CP) $(MISC)/template.cpp $(SRC)

aoj:
	$(CP) $(MISC)/aoj.cpp $(SRC)

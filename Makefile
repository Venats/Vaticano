CPP=c++

OBJ_DIR = obj
SRC := $(wildcard src/*/*.cpp) $(wildcard src/*.cpp) $(wildcard src/*/*/*.cpp) $(wildcard src/*/*/*/*.cpp)
OBJ := $(addprefix $(OBJ_DIR)/,$(addsuffix .o,$(notdir $(basename $(SRC)))))

SRC_DIRS := $(sort $(dir $(SRC)))
BIN = bin

VPATH := $(SRC_DIRS)

EXE:=$(BIN_DIR)/main

CPPFLAGS= --std=gnu++17 \
-Wall \
-Werror \
-Wpedantic \
-Wshadow \
-Wcast-align \
-O3 \
-fno-exceptions

.PHONY: dirs clean

all: dirs $(EXE)

dirs:
	mkdir -p ./$(BIN) ./$(OBJ_DIR)

$(EXE): $(OBJ) 
	$(CPP) -o $(BIN)/main $^ $(LDFLAGS)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp | $(OBJ_DIR)
	$(CPP) $(CPPFLAGS) $(PPFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: src/Board/%.cpp
	$(CPP)  -o $@ -c $< $(CPPFLAGS)

$(OBJ_DIR)/%.o: src/Pieces/%.cpp
	$(CPP)  -o $@ -c $< $(CPPFLAGS)

$(OBJ_DIR)/%.o: src/%.cpp
	$(CPP)  -o $@ -c $< $(CPPFLAGS)

clean:
	@$(RM) -rfv \
		$(OBJ_DIR)\
		$(BIN_DIR)
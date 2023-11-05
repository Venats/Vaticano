CPP=cc

OBJ_DIR = obj
SRC := $(wildcard src/*/*.c) $(wildcard src/*.c) $(wildcard src/*/*/*.c) $(wildcard src/*/*/*/*.c)
OBJ := $(addprefix $(OBJ_DIR)/,$(addsuffix .o,$(notdir $(basename $(SRC)))))

SRC_DIRS := $(sort $(dir $(SRC)))
BIN = bin

VPATH := $(SRC_DIRS)

EXE:=$(BIN_DIR)/main

INCLUDES:= \
	src/

CPPFLAGS= --std=gnu17 \
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

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	$(CPP) $(CPPFLAGS) $(PPFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: src/Board/%.c 
	$(CPP)  -o $@ -c $< -I $(INCLUDES) $(CPPFLAGS)

$(OBJ_DIR)/%.o: src/Pieces/%.c
	$(CPP)  -o $@ -c $< -I $(INCLUDES) $(CPPFLAGS)

$(OBJ_DIR)/%.o: src/%.c
	$(CPP)  -o $@ -c $< -I $(INCLUDES) $(CPPFLAGS)

clean:
	@$(RM) -rfv \
		$(OBJ_DIR)\
		$(BIN_DIR)
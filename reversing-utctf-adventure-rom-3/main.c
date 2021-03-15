#include <gb/gb.h>
#include "grass.h"
#include "key.h"
#include "character.h"
#include "wall.h"
#include "font.h"
#include "text.h"
#include "game.h"
#include <stdio.h>

void updateSwitches() {
    HIDE_WIN;
    SHOW_SPRITES;
}

UINT8 init_tile = 0;
UINT8 init_i = 0;
UINT8 init_j = 0;
void init() {
    DISPLAY_ON;
    SHOW_SPRITES;
    SHOW_BKG;

    set_bkg_data(0, 1, grass_tile);
    set_bkg_data(1, 1, key_tile);
    set_bkg_data(2, 1, wall_tile);
    set_sprite_data(0, 1, character_tile);

    set_bkg_data(17, 40, font_data);

    for(init_i = 0; init_i < 20; init_i++) {
        for(init_j = 0; init_j < 18; init_j++) {
            set_bkg_tiles(init_i, init_j, 1, 1, &init_tile);
        }
    }

    set_sprite_tile(0, 0);
    move_sprite(0, 88, 88);
    init_game();
}

UINT8 input;
void handle_input() {
    input = joypad();
    if(input & J_LEFT) {
        move_character(255, 0);
    }
    if(input & J_RIGHT) {
        move_character(1, 0);
    }
    if(input & J_DOWN) {
        move_character(0, 1);
    }
    if(input & J_UP) {
        move_character(0, 255);
    }
}

void main() {   
    printf(" \n UTCTF MAZE GAME\n BY GG\n\n");
    printf("PRESS START TO BEGIN\n");
   
    while(!(joypad() & J_START)){}
    init();
    while(1) {
        draw_map();
        handle_input();
        wait_vbl_done();
    }
}
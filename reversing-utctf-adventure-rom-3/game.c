#include <gb/gb.h>
#include "game.h"
#include "maze.h"
#include "debug.h"
#include "text.h"

#define CHAR_OFF_X 10
#define CHAR_OFF_Y 9
#define IDX(x, y) (50 * y + x)
#define MAZE(x, y) maze[IDX(x, y)]

UINT16 keys[] = {1820/2, 4900/2, 106/2, 226/2, 166/2, 3450/2, 3800/2, 4792/2, 1058/2, 2050/2};
#define NUM_KEYS 10
UINT8 keys_collected = 0;
UINT8 in_order = 1;
UINT8 game_over = 0;

UINT8 passes[] = {
    223, 32, 5, 39, 54, 118, 222, 196, 36, 95, 65, 234, 90, 78, 85, 131, 2, 171, 157, 143, 122, 26, 54, 115, 209, 148, 66, 142, 153, 64, 87, 162, 7, 91, 33, 73, 154, 222, 18, 113, 190, 55, 65, 222, 184, 9, 251, 89, 132, 200, 75, 211, 236, 128, 155, 207, 152, 241, 129, 87, 237, 56, 204, 152, 76, 117, 95, 201, 85, 207, 200, 69, 142, 121, 223, 232, 144, 255, 21, 125
};
UINT8 curr_pass[] = {0, 0, 0, 0, 0, 0, 0, 0};
UINT8 flag[] = {77, 77, 192, 163, 203, 68, 72, 164, 0}; //VRAQ8C9A

UINT8 keyidx;
UINT8 find_key_idx(UINT16 key_loc) {
    // debug_printf("looking for key %d\n", key_loc);
    for(keyidx = 0; keyidx < NUM_KEYS; keyidx++) {
        if(keys[keyidx] == key_loc) {
            // debug_printf("found key at %d\n", keyidx);
            return keyidx;
        }
    }
    // debug_printf("key not found\n");
    return 0;
}

UINT8 passi;
void apply_key_pass(UINT8 key_idx) {
    for(passi = 0; passi < 8; passi++) {
        curr_pass[passi] *= 3;
        curr_pass[passi] += passes[8 * (9-key_idx) + passi];
        // debug_printf("%d, ", curr_pass[passi]);
    }
    // debug_printf("\n");
}

UINT8 printi;
void print_pass() {
    for(printi = 0; printi < 8; printi++) {
        debug_printf("%d, ", curr_pass[printi]);
    }
    debug_printf("\n");
}

UINT8 curr_x;
UINT8 curr_y;
UINT8 keys_i;
void init_game() {
    curr_x = 25;
    curr_y = 25;
    for(keys_i = 0; keys_i < NUM_KEYS; keys_i++) {
        maze[keys[keys_i]] = MAZE_KEY;
    }
}

UINT8 shadow_bkg[360];
#define BKG_TILE(x, y) shadow_bkg[y * 20 + x]

UINT8 drawx, drawy;
UINT16 mapx, mapy;
UINT8 maze_tile;
UINT8 grass_tilenum = 0;
UINT8 wall_tilenum = 2;
UINT8 key_tilenum = 1;
UINT8 key_count_buf[2];
UINT8 flagi;
UINT8 flag_show_buf[9];
void draw_map() {
    if(game_over) {
        if(in_order) {
            write_text("A WINNER IS YOU", 2, 3);
            write_text("CONGRATULATIONS", 2, 4);
            write_text("THE FLAG IS", 3, 6);
            write_text("UTFLAG ", 2, 7);
            for(flagi = 0; flagi < 8; flagi++) {
                flag_show_buf[flagi] = curr_pass[flagi] ^ flag[flagi];
            }
            flag_show_buf[8] = 0;
            write_text(flag_show_buf, 9, 7);
            write_text(" ", 17, 7);
        }else{
            write_text("YOU LOST", 5, 3);
            write_text(  "OOPS", 7, 4);
            write_text("SORRY", 6, 5);
            write_text("LMOA", 7, 6);
        }

        return;
    }
    for(drawx = 0; drawx < 20; drawx++){
        for(drawy = 0; drawy < 18; drawy++) {
            mapx = curr_x - CHAR_OFF_X + drawx;
            mapy = curr_y - CHAR_OFF_Y + drawy;

            if(mapx >= 50 || mapy >= 50) {
                //set_bkg_tiles(drawx, drawy, 1, 1, &wall_tilenum);
                BKG_TILE(drawx, drawy) = wall_tilenum;
                continue;
            }

            maze_tile = MAZE(mapx, mapy);

            if(maze_tile == MAZE_GRASS) {
                BKG_TILE(drawx, drawy) = grass_tilenum;
            }else if(maze_tile == MAZE_KEY) {
                BKG_TILE(drawx, drawy) = key_tilenum;
            }else{
                BKG_TILE(drawx, drawy) = wall_tilenum;
            }
        }
    }

    BKG_TILE(0, 0) = key_tilenum;
    key_count_buf[0] = keys_collected / 10;
    key_count_buf[1] = keys_collected % 10;
    
    BKG_TILE(1, 0) = key_count_buf[0] + 47;
    BKG_TILE(2, 0) = key_count_buf[1] + 47;

    set_bkg_tiles(0, 0, 20, 18, shadow_bkg);
}

UINT8 newx, newy;
UINT8 maze_checker;
void move_character(UINT8 dx, UINT8 dy) {
    if(game_over) return;
    newx = curr_x + dx;
    newy = curr_y + dy;
    if(newx >= 50 || newy >= 50) return;

    maze_checker = MAZE(newx, newy);
    if(maze_checker == MAZE_WALL) {
        return;
    }

    curr_x = newx;
    curr_y = newy;

    if(maze_checker == MAZE_KEY) {
        MAZE(newx, newy) = MAZE_GRASS;

        in_order &= keys[keys_collected] == IDX(newx, newy);

        apply_key_pass(find_key_idx(IDX(newx, newy)));
        // print_pass();

        keys_collected++;
        if(keys_collected == NUM_KEYS) {
            draw_map();
            game_over = 1;
        }
    }
}
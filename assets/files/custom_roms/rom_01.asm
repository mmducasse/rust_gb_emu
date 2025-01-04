SECTION "Main", ROM0[0x100]
Main:
    ld b, 1
    ld c, 2
    ld d, 3
    ld e, 4
    ld h, 5
    ld l, 6
    ld [hl], 7
    ld a, 8

    ld a, a
    nop
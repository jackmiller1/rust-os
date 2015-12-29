section .multiboot_header
header_start:
    dd 0xE85250D6                   ; magic number (multiboot 2)
    dd 0                            ; architecture 0 (protected mode)
    dd header_end - header_start    ; header length

    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; add optional multiboot tags here

    ; ending tags
    dw 0    ; type
    dw 0    ; flags
    dw 8    ; size
header_end:
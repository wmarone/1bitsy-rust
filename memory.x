MEMORY
{
  /* The 1Bitsy V1.0 has an STM32F415RGT6 with 1MB of Flash and 192KB RAM */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 192K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
    .text : {
        *(.vectors)
        *(.text*)
        . = ALIGN(4);

        *(.rodata*)
        . = ALIGN(4);
    } > RAM
}

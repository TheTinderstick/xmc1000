MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */

   FLASH : ORIGIN = 0x10001000, LENGTH = 200K

   VECTOR : ORIGIN = 0x20000000, LENGTH = 192
   SCRATCH : ORIGIN = 0x200000C0, LENGTH = 512 - 192
   RAM : ORIGIN = 0x20000200, LENGTH = 16K - 512
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Size of the heap (in bytes) */
/* _heap_size = 1024; */

/* Location of remapped vector table */
_vector_remap = ORIGIN(VECTOR);
_vector_remap_end = ORIGIN(VECTOR) + LENGTH(VECTOR);
_vector_start = ORIGIN(FLASH);

/* 320 bytes of RAM that may be cleared during non-power resets */
_bscratch = ORIGIN(SCRATCH);
_escratch = ORIGIN(SCRATCH) + LENGTH(SCRATCH);


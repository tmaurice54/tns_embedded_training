MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 96K
  SIGN : ORIGIN = 0x0801E900,  LENGTH = 8K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .signature :
  {
  	. = ALIGN(4);
  	LONG(0x444F4F47);
  } >SIGN
}
MEMORY
{
  /* Core-visible TCM instruction RAM. */
  ITCM : ORIGIN = 0x00000000, LENGTH = 32K
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  /* link.x expects a region named RAM; on G32R430 this is DTCM. */
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
  /* DMA-visible alias window for ITCM, useful only for explicit section placement. */
  ITCM_DMA : ORIGIN = 0x20004000, LENGTH = 32K
}

# look it up in G32R4_Series.yaml

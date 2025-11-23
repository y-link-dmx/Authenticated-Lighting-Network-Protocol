#include "alnp.h"
#include <stdio.h>

int main(void)
{
  if (alnp_init() != 0)
  {
    printf("Failed to init ALNP\\n");
    return 1;
  }
  printf("Session state: %d\\n", alnp_get_state());
  alnp_start_streaming();
  alnp_stop_streaming();
  return 0;
}

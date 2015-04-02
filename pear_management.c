#include "pear_management.h"
#include <stdio.h> 


maiko_int maiko_int_init () { 
  maiko_int retval = malloc(sizeof(maiko_int_struct) + 16*sizeof(int)); 
  retval->length = 16; 
  retval->filled_to = 0; 
  return retval; 
} 


int maiko_int_expand (maiko_int list) { 
  if ((list->length<<1) <= list->length) { 
    return 1; 
  } 
  maiko_int new_list = realloc(list, sizeof(maiko_int_struct)+(sizeof(int)*(list->length<<1))); 
  if (!new_list) { return 1; } 
  list = new_list; 
  list->length = list->length<<1; 
  return 0; 
} 


int maiko_int_shrink (maiko_int list) { 
  if ((list->length)>>3 < list->filled_to || (list->length)>>3 < 16) { 
    return 1; 
  } 
  maiko_int new_list = realloc (list, sizeof(maiko_int_struct)+(sizeof(int)*(list->length>>1))); 
  if (!new_list) { return 1; } 
  list = new_list; 
  list->length = list->length>>1; 
  return 0; 
} 


int maiko_int_get (maiko_int list, size_t index) { 
  if (index < list->filled_to) { 
    return (*(list->list+index)); 
  } 
  return 0; 
} 


int maiko_int_insert (maiko_int list, int prependee, size_t index) { 
  int error; 
  while ((list->filled_to)+1 >= list->length) { 
    error = maiko_int_expand (list); 
    if (error) { return error; } 
  } 
  error = maiko_int_rs (list, index); 
  if (error) { return error; } 
  list->list[index] = prependee; 
  ++(list->filled_to); 
  return 0; 
} 


int maiko_int_append (maiko_int list, int appendee) { 
  int error = maiko_int_insert (list, appendee, list->filled_to); 
  if (error) { return error; } 
  return 0; 
} 


int maiko_int_prepend (maiko_int list, int prependee) { 
  int error = maiko_int_insert (list, prependee, 0); 
  if (error) { return error; } 
  return 0; 
} 


int maiko_int_delete (maiko_int list, size_t index) { 
  int error = maiko_int_ls (list, index); 
  if (error) { return error; } 
  if ((list->length)>>3 > list->filled_to && (list->length)>>3 > 16) { 
    maiko_int_shrink (list); 
  } 
  --(list->filled_to); 
  return 0; 
} 


int maiko_int_ls (maiko_int list, size_t index) { 
  if (index >= list->filled_to) { return 1; } 
  size_t i = index; 
  for (; i+1 < list->filled_to; ++i) { 
    list->list[i] = list->list[i+1]; 
  } 
  return 0; 
} 


int maiko_int_rs (maiko_int list, size_t index) { 
  if (index > list->filled_to) { return 1; } 
  while ((list->filled_to)+1 >= list->length) { 
    int error = maiko_int_expand (list); 
    if (error) { return error; } 
  } 
  size_t i = (list->filled_to); 
  for (; i > index; --i) { 
    list->list[i] = list->list[i-1]; 
  } 
  return 0; 
} 

int maiko_int_print (maiko_int list) { 
  size_t i = 0; 
  printf("[ "); 
  for (; i<list->filled_to; ++i) { 
    printf ("%d, ", list->list[i]); 
  } 
  if (i) { 
    char backspace = 0x08; 
    printf("%c%c ]\n", backspace, backspace); 
  } else { 
    printf("]\n"); 
  } 
  
  return 0; 
} 

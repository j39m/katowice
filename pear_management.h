// header file for some mucking around with C lists

#include <stdlib.h>

#ifndef MAIKO_INCL
#define MAIKO_INCL

#define DEFAULT_SIZE 16 

// a list of integers 
typedef struct { 
  size_t length; 
  size_t filled_to; 
  int list[]; 
} maiko_int_struct; 

// a pointer type for our integer list 
typedef maiko_int_struct * maiko_int; 

// create an empty list with a default size. 
// returns a pointer to the empty list. 
maiko_int maiko_int_init (); 

// expand a list - by default, doubles its size. 
// returns zero on success, one otherwise. 
int maiko_int_expand (maiko_int list); 

// shrink a list - by default, halves its size when 
// it's filled less than 1/8 of the way. lower bound
// on shrinking is 16 elements. 
// returns zero on success, one otherwise. 
int maiko_int_shrink (maiko_int list); 

// safely fetches an item from a list. 
// returns zero on invalid indices, else it returns
// the requested integer. 
// TODO: Raise a red flag for invalid indices. 
int maiko_int_get (maiko_int list, size_t index); 

// safely adds an item into a list at specified index. 
// returns zero on success, one otherwise. 
int maiko_int_insert (maiko_int list, int insertee, size_t index); 

// safely adds an item to the end of a list. 
// returns zero on success, one otherwise. 
int maiko_int_append (maiko_int list, int appendee); 

// safely prepends an item into a list. 
// returns zero on success, one otherwise. 
int maiko_int_prepend (maiko_int list, int prependee); 

// deletes an item from a list. 
// returns zero on success, one otherwise. 
int maiko_int_delete (maiko_int list, size_t index); 

// deletes an item off the end of the list. 
// returns the item (int) on success, zero otherwise. 
// TODO: raise red flag on unsuccessful pop. 
int maiko_int_pop (maiko_int list); 

// given an index, shifts everything after that once 
// leftward. called internally by maiko_int_delete to
// close the gap left by a deleted value. 
// returns zero on success, one otherwise. 
int maiko_int_ls (maiko_int list, size_t index); 

// given an index, shifts everything after and including 
// that once rightward. called internally by both
// maiko_int_insert and maiko_int_prepend. 
// returns zero on success, one otherwise. 
int maiko_int_rs (maiko_int list, size_t index); 

// given a maiko_int_list, prints out everything in the 
// list. 
int maiko_int_print (maiko_int list); 

#endif // MAIKO_INCL guard

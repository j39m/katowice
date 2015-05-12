// bittermelon_bottle: a study on reading and parsing FLAC metadata in C

#include <stdio.h> 
#include <stdlib.h>
#include <string.h> 
#include <endian.h> 

void _bb_sanity (FILE *); 
void _bb_reset (FILE *); 
char * bb_read_metadata_block_header (FILE *); 
int bb_is_last_mbh (char *); 
int bb_block_type (char *); 
int bb_block_length (char *); 


int main () { 

  char test_name[] = "./test.flac"; 
  FILE *file = fopen (test_name, "r"); 

  _bb_sanity (file); 
  _bb_reset (file); 
  char *mbh = bb_read_metadata_block_header (file); 
  while (!bb_is_last_mbh (mbh)) { 
    fseek (file, bb_block_length(mbh), SEEK_CUR); 
    free (mbh); 
    mbh = bb_read_metadata_block_header (file); 
    printf ("block type %d with length %d\n", bb_block_type(mbh), bb_block_length(mbh)); 
  } 

  return 0; 

} 


// given an appropriately seeked file (i.e. seeked to the start of a 
// METADATA_BLOCK), returns a 32-bit char buffer (which must be freed)
// containing the METADATA_BLOCK_HEADER by itself. 
// file is seeked to the start of the header itself upon return. 
char * bb_read_metadata_block_header ( FILE *file) {
  char * mbh = malloc (4); // dispense with null term
  fread ((void *) mbh, 1, 4, file); 
  return mbh; 
} 


// given a METADATA_BLOCK_HEADER (i.e. the output of bb_read_metadata...)
// returns whether or not the block identified is last or not. 1 if so,
// 0 otherwise (!!!)
int bb_is_last_mbh (char *mbh) {
  return ((int) 128 & mbh[0])>>7; 
} 


// given a METADATA_BLOCK_HEADER (i.e. the output of bb_read_metadata...)
// returns the BLOCK_TYPE as an int. 
int bb_block_type (char *mbh) { 
  int retval = 0; 
  memcpy ((void *) &retval, mbh, 1); 
  return retval & 127; 
} 


// given a METADATA_BLOCK_HEADER (i.e. the output of bb_read_metadata...)
// returns the length in bytes of metadata to follow. 
int bb_block_length (char *mbh) { 
  int retval = 0; 
  memcpy ((void *) &retval, mbh+1, 3); 
  return ntohl(retval)>>8; 
} 


// seek resetter. starts the file seek at the head of the first METADATA_
// BLOCK_HEADER, right after the "fLaC" magic. 
void _bb_reset (FILE *file) { 
  int err = fseek (file, 4, SEEK_SET); 
  if (err) { 
    fprintf (stderr, "error in fseek() in _bb_reset\n"); 
  } 
  return; 
} 


// sanity function that prints out silly verifications: the "fLaC" header, 
// the sample rate of the file (probably 44100), and the total number of 
// samples in the file. Upon its return, the position of the seek will 
// point to whatever I left this function implemented as. 
void _bb_sanity (FILE *file) { 
  fseek (file, 0, SEEK_SET); 

  unsigned char header[5]; 
  header[4] = '\0'; 
  fread ((void *) header, sizeof(unsigned char), 4, file); 
  printf ("%s\n", header); 

  int sample_rate = 0; 
  fseek (file, 14, SEEK_CUR); 
  fread ((void *) &sample_rate, sizeof(char), 3, file); 
  printf ("%d\n", ntohl(sample_rate)>>12); 
  
  long total_samples = 0; 
  fread((void *) &total_samples, sizeof(char), 5, file); 
  printf ("%ld\n", (be64toh(total_samples)<<4)>>28); 

  return; 
} 


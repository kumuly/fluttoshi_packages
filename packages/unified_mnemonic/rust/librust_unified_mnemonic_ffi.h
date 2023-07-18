#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Language to be used for the mnemonic phrase.
 */
typedef enum Language {
  /**
   * The English language.
   */
  English,
  /**
   * The Spanish language.
   */
  Spanish,
} Language;

typedef enum Network {
  Bitcoin,
  Testnet,
  Signet,
  Regtest,
} Network;

/**
 * Type describing entropy length (aka word count) in the mnemonic
 */
typedef enum WordCount {
  /**
   * 12 words mnemonic (128 bits entropy)
   */
  Words12 = 12,
  /**
   * 15 words mnemonic (160 bits entropy)
   */
  Words15 = 15,
  /**
   * 18 words mnemonic (192 bits entropy)
   */
  Words18 = 18,
  /**
   * 21 words mnemonic (224 bits entropy)
   */
  Words21 = 21,
  /**
   * 24 words mnemonic (256 bits entropy)
   */
  Words24 = 24,
} WordCount;

typedef struct Mnemonic {
  char *phrase;
  /**
   * The language the mnemonic.
   */
  enum Language language;
} Mnemonic;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Creates a new mnemonic phrase.
 *
 * # Safety
 *
 * This function is marked `unsafe` because it returns a raw pointer to a heap-allocated
 * structure, and the caller (in C or another language) is responsible for deallocating
 * this structure through the appropriate function (`free_mnemonic` for example).
 *
 * It is undefined behavior if the caller does not ensure that the `Mnemonic` is properly
 * deallocated.
 *
 * # Parameters
 *
 * - `language`: The language in which the mnemonic phrase should be generated.
 * - `word_count`: The number of words in the mnemonic phrase.
 *
 * # Returns
 *
 * A pointer to the newly created `Mnemonic`. The caller must ensure that the `Mnemonic`
 * is properly deallocated to avoid memory leaks.
 */
struct Mnemonic *new(enum Language language, enum WordCount word_count);

/**
 * Creates a new `Mnemonic` structure from an existing mnemonic phrase.
 *
 * # Safety
 *
 * This function is marked `unsafe` because it takes a raw pointer to a C string as
 * an argument and returns a raw pointer to a heap-allocated structure.
 *
 * The caller must ensure that:
 * - The `phrase` pointer points to a valid null-terminated string.
 * - The string data is encoded in UTF-8.
 * - The `Mnemonic` returned is properly deallocated through the appropriate function
 *   (`free_mnemonic` for example).
 *
 * It is undefined behavior if the caller does not ensure that the `Mnemonic` is properly
 * deallocated or if the `phrase` pointer is not valid.
 *
 * # Parameters
 *
 * - `phrase`: A pointer to a null-terminated string that contains the mnemonic phrase.
 *
 * # Returns
 *
 * A pointer to the newly created `Mnemonic`. The caller must ensure that the `Mnemonic`
 * is properly deallocated to avoid memory leaks.
 */
struct Mnemonic *from_phrase(const char *phrase);

/**
 * # Safety
 *
 * This function is unsafe because it dereferences raw pointers.
 * Callers must ensure that the Mnemonic struct is properly initialized
 * and that the memory it points to remains allocated for the duration of the call.
 *
 * Additionally, the caller is responsible for managing the memory of the returned array,
 * which is allocated on the heap.
 */
char *derive_lightning_seed(const struct Mnemonic *self,
                            enum Network network,
                            const uint32_t *hardened_child_index);

/**
 * # Safety
 *
 * This function is unsafe because it dereferences a raw pointer.
 * Callers must ensure that the pointer is valid.
 *
 * This function should only be used to free memory that was allocated by the Rust code.
 */
void free_mnemonic(struct Mnemonic *mnemonic);

/**
 * # Safety
 *
 * This function is unsafe because it dereferences a raw pointer.
 * Callers must ensure that the pointer is valid.
 *
 * This function should only be used to free memory that was allocated by the Rust code.
 */
void free_lightning_seed(uint8_t (*seed)[32]);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

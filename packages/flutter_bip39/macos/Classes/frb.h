#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_Mnemonic {
  int32_t language;
  struct wire_StringList *words;
} wire_Mnemonic;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_all__static_method__Language(int64_t port_);

void wire_word_list__method__Language(int64_t port_, int32_t that);

void wire_words_by_prefix__method__Language(int64_t port_,
                                            int32_t that,
                                            struct wire_uint_8_list *prefix);

void wire_find_word__method__Language(int64_t port_, int32_t that, struct wire_uint_8_list *word);

void wire_generate_in__static_method__Mnemonic(int64_t port_, int32_t language, int32_t word_count);

void wire_parse__static_method__Mnemonic(int64_t port_, struct wire_StringList *words);

void wire_parse_in__static_method__Mnemonic(int64_t port_,
                                            int32_t language,
                                            struct wire_StringList *words);

void wire_to_seed__method__Mnemonic(int64_t port_,
                                    struct wire_Mnemonic *that,
                                    struct wire_uint_8_list *passphrase);

struct wire_StringList *new_StringList_0(int32_t len);

struct wire_Mnemonic *new_box_autoadd_mnemonic_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_all__static_method__Language);
    dummy_var ^= ((int64_t) (void*) wire_word_list__method__Language);
    dummy_var ^= ((int64_t) (void*) wire_words_by_prefix__method__Language);
    dummy_var ^= ((int64_t) (void*) wire_find_word__method__Language);
    dummy_var ^= ((int64_t) (void*) wire_generate_in__static_method__Mnemonic);
    dummy_var ^= ((int64_t) (void*) wire_parse__static_method__Mnemonic);
    dummy_var ^= ((int64_t) (void*) wire_parse_in__static_method__Mnemonic);
    dummy_var ^= ((int64_t) (void*) wire_to_seed__method__Mnemonic);
    dummy_var ^= ((int64_t) (void*) new_StringList_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_mnemonic_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}

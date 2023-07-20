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

typedef struct wire_Mnemonic {
  struct wire_uint_8_list *phrase;
  int32_t language;
} wire_Mnemonic;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_new__static_method__Mnemonic(int64_t port_, int32_t language, int32_t word_count);

void wire_from_phrase__static_method__Mnemonic(int64_t port_, struct wire_uint_8_list *phrase);

void wire_derive_lightning_seed__method__Mnemonic(int64_t port_,
                                                  struct wire_Mnemonic *that,
                                                  int32_t network,
                                                  uint32_t *hardened_child_index);

struct wire_Mnemonic *new_box_autoadd_mnemonic_0(void);

uint32_t *new_box_autoadd_u32_0(uint32_t value);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__Mnemonic);
    dummy_var ^= ((int64_t) (void*) wire_from_phrase__static_method__Mnemonic);
    dummy_var ^= ((int64_t) (void*) wire_derive_lightning_seed__method__Mnemonic);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_mnemonic_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u32_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}

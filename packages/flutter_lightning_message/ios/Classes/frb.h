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

typedef struct wire_Signer {
  struct wire_uint_8_list *secret_key_bytes;
  struct wire_uint_8_list *node_id;
} wire_Signer;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_verify(int64_t port_,
                 struct wire_uint_8_list *message,
                 struct wire_uint_8_list *signature,
                 struct wire_uint_8_list *public_key);

void wire_recover_node_id(int64_t port_,
                          struct wire_uint_8_list *message,
                          struct wire_uint_8_list *signature);

void wire_from_seed__static_method__Signer(int64_t port_, struct wire_uint_8_list *seed);

void wire_from_ldk_seed__static_method__Signer(int64_t port_, struct wire_uint_8_list *seed);

void wire_sign__method__Signer(int64_t port_,
                               struct wire_Signer *that,
                               struct wire_uint_8_list *message);

struct wire_Signer *new_box_autoadd_signer_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_verify);
    dummy_var ^= ((int64_t) (void*) wire_recover_node_id);
    dummy_var ^= ((int64_t) (void*) wire_from_seed__static_method__Signer);
    dummy_var ^= ((int64_t) (void*) wire_from_ldk_seed__static_method__Signer);
    dummy_var ^= ((int64_t) (void*) wire_sign__method__Signer);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_signer_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}

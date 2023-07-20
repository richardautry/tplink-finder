#include <stdint.h>

struct device_data;

const char* rust_greeting(const char* to);
void rust_greeting_free(char *);
const struct device*_Nonnull *tplinker_discovery(unsigned int *len);
void tplinker_vec_destroy(int8_t *arr);
const char* _Nonnull device_data_get_alias(const struct device_data* _Nonnull device_data);

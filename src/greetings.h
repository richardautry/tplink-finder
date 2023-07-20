#include <stdint.h>

struct device_data;
struct full_device;

const char* rust_greeting(const char* to);
void rust_greeting_free(char *);
const struct device_data*_Nonnull *tplinker_discovery(unsigned int *len);
void tplinker_vec_destroy(int8_t *arr);
const char* _Nonnull device_data_get_alias(const struct device_data* _Nonnull device_data);

const struct full_device*_Nonnull *tplinker_device_discovery(unsigned int *len);
const char* _Nonnull full_device_get_alias(const struct full_device* _Nonnull full_device);
const char* _Nonnull full_device_get_addr(const struct full_device* _Nonnull full_device);
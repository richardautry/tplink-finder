#include <stdint.h>
#include <stdbool.h>

struct device_data;
struct full_device;
struct timer;
struct ffifuture;

const char* rust_greeting(const char* to);
void rust_greeting_free(char *);
const struct device_data*_Nonnull *tplinker_discovery(unsigned int *len);
void tplinker_vec_destroy(int8_t *arr);
const char* _Nonnull device_data_get_alias(const struct device_data* _Nonnull device_data);

const struct full_device*_Nonnull *tplinker_device_discovery(unsigned int *len);
const char* _Nonnull full_device_get_alias(const struct full_device* _Nonnull full_device);
const char* _Nonnull full_device_get_addr(const struct full_device* _Nonnull full_device);
const bool full_device_is_on(const struct full_device* _Nonnull full_device);
const bool full_device_switch_off(const struct full_device* _Nonnull full_device);
const bool full_device_switch_on(const struct full_device* _Nonnull full_device);
const bool turn_off_after(const int length_ms, const struct full_device* _Nonnull full_device);
const struct timer* get_timer(const unsigned int length_ms);
const unsigned int poll_timer(const struct timer* _Nonnull timer);
const bool start_timer(const int length_ms, struct timer * _Nonnull timer);
const bool discover_services();
// const ffifuture* start_timer_test(const int length_ms);
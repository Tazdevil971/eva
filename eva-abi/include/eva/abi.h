#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "common.h"
#include "abi_manual.h"

#define OK 0

#define NOT_JOINABLE_ERROR -1

#define ALREADY_RUNNING_ERROR -2

#define INVALID_THREAD_ERROR -3

#define INVALID_TLS_KEY_ERROR -4

typedef struct Duration2 {
  uint64_t secs;
  uint32_t nanos;
} Duration2;

typedef void *Thread2;

typedef int8_t Priority;

typedef uint32_t ThreadId2;
#define ThreadId2_INVALID 0

typedef uint32_t TlsKey2;
#define TlsKey2_INVALID 0

typedef struct ALIGNED(8) Mutex2 {
  uint8_t _0[32];
} Mutex2;

typedef struct ALIGNED(8) Condvar2 {
  uint8_t _0[32];
} Condvar2;

extern struct Duration2 eva_c_get_time(void);

extern uintptr_t eva_c_io_kwrite(const uint8_t *data, uintptr_t len);

extern uintptr_t eva_c_io_kread(uint8_t *data, uintptr_t len);

extern void *eva_c_alloc(uintptr_t size, uintptr_t align);

extern void eva_c_dealloc(void *ptr, uintptr_t size, uintptr_t align);

extern void *eva_c_emu_malloc(uintptr_t size);

extern void eva_c_emu_free(void *ptr);

extern Thread2 eva_c_rt_spawn(uintptr_t stack_size,
                              Priority priority,
                              void (*entry)(void*),
                              const char *name,
                              void *user);

extern bool eva_c_rt_exists(Thread2 thread);

extern bool eva_c_rt_exists_paused(Thread2 thread);

extern int eva_c_rt_join(Thread2 thread);

extern int eva_c_rt_join_unchecked(Thread2 thread);

extern int eva_c_rt_detach(Thread2 thread);

extern int eva_c_rt_detach_unchecked(Thread2 thread);

extern Priority eva_c_rt_get_current_priority(void);

extern ThreadId2 eva_c_rt_get_current_tid(void);

extern void eva_c_rt_suspend_paused(void);

extern void eva_c_rt_suspend_and_yield(void);

extern void eva_c_rt_suspend_and_yield_paused(void);

extern int eva_c_rt_resume(Thread2 thread);

extern int eva_c_rt_resume_unchecked(Thread2 thread);

extern void eva_c_rt_resume_irq_unchecked(Thread2 thread);

extern int eva_c_rt_resume_paused(Thread2 thread);

extern int eva_c_rt_resume_paused_unchecked(Thread2 thread);

extern Thread2 eva_c_rt_current(void);

extern void eva_c_rt_sleep_for(struct Duration2 time);

extern void eva_c_rt_sleep_until(struct Duration2 time);

extern bool eva_c_rt_suspend_and_yield_for(struct Duration2 time);

extern bool eva_c_rt_suspend_and_yield_until(struct Duration2 time);

extern bool eva_c_rt_suspend_and_yield_paused_for(struct Duration2 time);

extern bool eva_c_rt_suspend_and_yield_paused_until(struct Duration2 time);

extern void eva_c_rt_yield_now(void);

extern void eva_c_rt_pend_yield(void);

extern void eva_c_rt_abort(void);

extern bool eva_c_rt_is_paused(void);

extern bool eva_c_rt_try_pause(void);

extern bool eva_c_rt_try_unpause(void);

extern TlsKey2 eva_c_rt_tls_key_create(void (*dtor)(void*));

extern int eva_c_rt_tls_key_delete(TlsKey2 key);

extern int eva_c_rt_tls_set_specific(TlsKey2 key, void *data);

extern void *eva_c_rt_tls_get_specific(TlsKey2 key);

extern bool eva_c_rt_sync_mutex_try_lock(struct Mutex2 *mutex);

extern bool eva_c_rt_sync_mutex_try_lock_for(struct Mutex2 *mutex, struct Duration2 time);

extern bool eva_c_rt_sync_mutex_try_lock_until(struct Mutex2 *mutex, struct Duration2 time);

extern void eva_c_rt_sync_mutex_lock(struct Mutex2 *mutex);

extern void eva_c_rt_sync_mutex_unlock(struct Mutex2 *mutex);

extern bool eva_c_rt_sync_mutex_is_locked(struct Mutex2 *mutex);

extern void eva_c_rt_sync_condvar_wait(struct Condvar2 *cvar, struct Mutex2 *mutex);

extern bool eva_c_rt_sync_condvar_wait_for(struct Condvar2 *cvar,
                                           struct Mutex2 *mutex,
                                           struct Duration2 time);

extern bool eva_c_rt_sync_condvar_wait_until(struct Condvar2 *cvar,
                                             struct Mutex2 *mutex,
                                             struct Duration2 time);

extern void eva_c_rt_sync_condvar_notify_one(struct Condvar2 *cvar);

extern void eva_c_rt_sync_condvar_notify_all(struct Condvar2 *cvar);

#include <eva/abi.h>

void kputs(const char *str) {
    int len = 0;
    while(str[len] != 0)
        len += 1;
    eva_c_io_kwrite((const uint8_t*)str, len);
}

typedef struct {
    int count;
    int tx_ptr;
    int rx_ptr;
    int queue[10];
    Mutex2 lock;
    Condvar2 tx;
    Condvar2 rx;
} SyncQueue;

void queue_init(SyncQueue *queue) {
    queue->count = 0;
    queue->tx_ptr = 0;
    queue->rx_ptr = 0;
    queue->lock = MUTEX2_INIT;
    queue->tx = CONDVAR2_INIT;
    queue->rx = CONDVAR2_INIT;
}

void queue_push(SyncQueue *queue, int value) {
    eva_c_rt_sync_mutex_lock(&queue->lock);
    
    while (queue->count >= 10) {
        // Wait for one notification
        eva_c_rt_sync_condvar_wait(&queue->tx, &queue->lock);
    }

    // We have space
    queue->queue[queue->tx_ptr] = value;
    queue->tx_ptr = (queue->tx_ptr + 1) % 10;
    queue->count += 1;

    eva_c_rt_sync_condvar_notify_one(&queue->rx);
    eva_c_rt_sync_mutex_unlock(&queue->lock);
}

int queue_pop(SyncQueue *queue) {
    int ret = 0;
    eva_c_rt_sync_mutex_lock(&queue->lock);

    while (queue->count == 0) {
        // Wait for one notification
        eva_c_rt_sync_condvar_wait(&queue->rx, &queue->lock);
    }

    // We have at least one element
    ret = queue->queue[queue->rx_ptr];
    queue->rx_ptr = (queue->rx_ptr + 1) % 10;
    queue->count -= 1;
    
    eva_c_rt_sync_condvar_notify_one(&queue->tx);
    eva_c_rt_sync_mutex_unlock(&queue->lock);
    return ret;
}

void pusher(void *user) {
    SyncQueue *queue = (SyncQueue*)user;
    
    kputs("Entering pusher!\n");
    for (int i = 0; i < 100; i++) {
        kputs("Pushing!\n");
        queue_push(queue, i);
    }
    kputs("Exiting pusher!\n");
}

void popper(void *user) {
    SyncQueue *queue = (SyncQueue*)user;
    
    kputs("Entering popper!\n");
    for (int i = 0; i < 100; i++) {
        int val = queue_pop(queue);
        if (val != i) {
            kputs("Invalid!\n");
        } else {
            kputs("Valid!\n");
        }
    }
    kputs("Exiting popper!\n");
}

int main() {
    // Emulate malloc on top of the native EVA allocator
    SyncQueue *queue = eva_c_emu_malloc(sizeof(SyncQueue));
    queue_init(queue);

    Thread2 pusher_thread = eva_c_rt_spawn(64 * 1024, 0, pusher, "Pusher", queue);
    Thread2 popper_thread = eva_c_rt_spawn(64 * 1024, 0, popper, "Popper", queue);

    eva_c_rt_join(pusher_thread);
    eva_c_rt_join(popper_thread);

    eva_c_emu_free(queue);
    kputs("Exiting main!\n");

    return 0;
}
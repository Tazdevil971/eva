#include <eva/abi.h>

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
    
    eva_c_kputs("Entering pusher!\n");
    for (int i = 0; i < 100; i++) {
        eva_c_kputs("Pushing!\n");
        queue_push(queue, i);
    }
    eva_c_kputs("Exiting pusher!\n");
}

void popper(void *user) {
    SyncQueue *queue = (SyncQueue*)user;
    
    eva_c_kputs("Entering popper!\n");
    for (int i = 0; i < 100; i++) {
        int val = queue_pop(queue);
        if (val != i) {
            eva_c_kputs("Invalid!\n");
        } else {
            eva_c_kputs("Valid!\n");
        }
    }
    eva_c_kputs("Exiting popper!\n");
}

void eva_kmain() {
    // Emulate malloc on top of the native EVA allocator
    SyncQueue *queue = eva_c_emu_malloc(sizeof(SyncQueue));
    queue_init(queue);

    Thread2 pusher_thread = eva_c_rt_spawn(4096, 0, pusher, "Pusher", queue);
    Thread2 popper_thread = eva_c_rt_spawn(4096, 0, popper, "Popper", queue);

    eva_c_rt_join_unchecked(pusher_thread);
    eva_c_rt_join_unchecked(popper_thread);

    eva_c_kputs("Finish!\n");

    eva_c_emu_free(queue);
    eva_c_kputs("Returning...\n");
}